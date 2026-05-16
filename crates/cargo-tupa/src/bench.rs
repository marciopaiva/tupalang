//! Per-step profiling: run the compiled pipeline binary many times and emit
//! per-step timing metrics derived from `--metrics-output`.

use anyhow::Result;
use serde::Serialize;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use super::discover::{build_binary, discover_binary_target, execute_binary};

/// Result of a single pipeline run (per-step median times).
#[derive(Debug, Serialize, Clone)]
pub struct StepTiming {
    step_id: String,
    median_ms: f64,
    p95_ms: f64,
    runs: usize,
}

/// Full profiling report.
#[derive(Debug, Serialize, Clone)]
pub struct BenchReport {
    pipeline_name: String,
    total_runs: usize,
    total_elapsed_ms: f64,
    throughput_per_sec: f64,
    steps: Vec<StepTiming>,
}

/// Collect metrics from a JSON metrics-output file produced by `Executor`.
/// Expects format: `[{"step_id":"...","start_nanos":...,"end_nanos":...}, ...]`
fn parse_metrics_file(path: &PathBuf) -> Result<Vec<StepTimingRow>> {
    let content = std::fs::read_to_string(path)?;
    let rows: Vec<StepTimingRow> = serde_json::from_str(&content)?;
    Ok(rows)
}

#[derive(Debug, serde::Deserialize)]
struct StepTimingRow {
    step_id: String,
    start_nanos: u64,
    end_nanos: u64,
}

impl StepTimingRow {
    fn duration_nanos(&self) -> u64 {
        self.end_nanos.saturating_sub(self.start_nanos)
    }
}

/// Run the pipeline `runs` times, collecting per-step metrics, and print a
/// human-readable + JSON report to stdout.
pub fn bench(
    manifest_path: Option<PathBuf>,
    runs: usize,
    json_output: Option<PathBuf>,
) -> Result<()> {
    if runs == 0 {
        anyhow::bail!("--runs must be >= 1");
    }

    // Resolve manifest
    let manifest = if let Some(ref p) = manifest_path {
        p.clone()
    } else {
        let mut dir = std::env::current_dir()?;
        loop {
            if dir.join("Cargo.toml").exists() {
                break;
            }
            let next = dir.parent().map(|p| p.to_path_buf());
            match next {
                Some(ref p) if *p != dir => dir = p.clone(),
                _ => anyhow::bail!("Cargo.toml not found"),
            }
        }
        dir.join("Cargo.toml")
    };

    // Build once
    let bin_name = discover_binary_target(&manifest)?;
    let binary_path = build_binary(&manifest, &bin_name)?;

    let tmp_dir = std::env::temp_dir().join("tupa-bench");
    std::fs::create_dir_all(&tmp_dir)?;

    // Store per-step duration samples: step_id -> Vec<Duration>
    use std::collections::HashMap;
    let mut step_samples: HashMap<String, Vec<Duration>> = HashMap::new();

    let mut run_times = Vec::with_capacity(runs);

    for i in 0..runs {
        let metrics_path = tmp_dir.join(format!("run_{}.json", i));
        let start = Instant::now();

        execute_binary(&binary_path, None, false, Some(&metrics_path))?;

        let elapsed = start.elapsed();
        run_times.push(elapsed);

        // Parse metrics
        if let Ok(rows) = parse_metrics_file(&metrics_path) {
            for row in rows {
                let nanos = row.duration_nanos();
                let dur = Duration::from_nanos(nanos);
                step_samples.entry(row.step_id).or_default().push(dur);
            }
        }
    }

    // Aggregate step timings
    let mut steps: Vec<StepTiming> = Vec::new();
    for (step_id, samples) in step_samples.iter() {
        let mut sorted = samples.clone();
        sorted.sort_by_key(|a| a.as_nanos());
        let n = sorted.len();
        let median = sorted[n / 2];
        let p95_idx = ((n as f64 * 0.95).ceil() as usize).min(n - 1);
        let p95 = sorted[p95_idx];
        steps.push(StepTiming {
            step_id: step_id.clone(),
            median_ms: median.as_secs_f64() * 1000.0,
            p95_ms: p95.as_secs_f64() * 1000.0,
            runs: n,
        });
    }
    steps.sort_by(|a, b| a.median_ms.partial_cmp(&b.median_ms).unwrap());

    // Compute total
    let total_elapsed = run_times.iter().sum::<Duration>();
    let total_elapsed_ms = total_elapsed.as_secs_f64() * 1000.0;
    let throughput = if total_elapsed.is_zero() {
        0.0
    } else {
        runs as f64 / total_elapsed.as_secs_f64()
    };

    // Print human-readable report
    println!("╔══════════════════════════════════════════════╗");
    println!("║          Tupã Pipeline Benchmark              ║");
    println!("╠══════════════════════════════════════════════╣");
    println!("║ Pipeline : {:<33}║", bin_name);
    println!("║ Runs     : {:<33}║", runs);
    println!(
        "║ Total    : {:>8.2} ms  throughput {:>8.1} /s ║",
        total_elapsed_ms, throughput
    );
    println!("╠══════════════════════════════════════════════╣");
    println!(
        "║ {:>8} {:>12} {:>12} {:>6} ║",
        "median_ms", "p95_ms", "runs", "step"
    );
    println!("╠══════════════════════════════════════════════╣");
    for s in &steps {
        println!(
            "║ {:>8.3} {:>12.3} {:>12} {:>6} ║",
            s.median_ms, s.p95_ms, s.runs, s.step_id
        );
    }
    println!("╚══════════════════════════════════════════════╝");

    // Optionally write JSON
    if let Some(ref out) = json_output {
        let report = BenchReport {
            pipeline_name: bin_name.clone(),
            total_runs: runs,
            total_elapsed_ms,
            throughput_per_sec: throughput,
            steps: steps.clone(),
        };
        std::fs::write(out, serde_json::to_string_pretty(&report)?)?;
        println!("Benchmark JSON written to {}", out.display());
    }

    Ok(())
}
