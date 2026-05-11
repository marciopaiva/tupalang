use criterion::{criterion_group, criterion_main, Criterion};
use serde_json::json;
use std::path::PathBuf;
use tupa_plugin::PluginManager;

// Compila o plugin de teste e retorna o caminho para a biblioteca
fn compile_test_plugin() -> Result<PathBuf, String> {
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let plugin_src_dir = manifest_dir
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("tests")
        .join("plugin_src");

    let target_dir = tempfile::tempdir().unwrap();
    let output = std::process::Command::new("cargo")
        .current_dir(&plugin_src_dir)
        .args(["build", "--release", "--target-dir"])
        .arg(target_dir.path())
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(format!(
            "cargo build failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let target_arch = std::env::var("CARGO_BUILD_TARGET").unwrap_or_else(|_| {
        if cfg!(target_os = "macos") {
            "universal2-apple-darwin".to_string()
        } else if cfg!(target_os = "linux") {
            "x86_64-unknown-linux-gnu".to_string()
        } else if cfg!(target_os = "windows") {
            "x86_64-pc-windows-msvc".to_string()
        } else {
            std::env::var("HOST").unwrap_or_else(|_| "unknown".to_string())
        }
    });

    let lib_dir = target_dir.path().join("release").join(target_arch);

    let lib_name = if cfg!(target_os = "windows") {
        "test_plugin.dll"
    } else if cfg!(target_os = "macos") {
        "libtest_plugin.dylib"
    } else {
        "libtest_plugin.so"
    };

    let lib_path = lib_dir.join(lib_name);
    if !lib_path.exists() {
        return Err(format!(
            "Plugin library not found at {}",
            lib_path.display()
        ));
    }

    Ok(lib_path)
}

fn bench_plugin_call_overhead(c: &mut Criterion) {
    // Compilar e carregar plugin (uma vez)
    let lib_path = match compile_test_plugin() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Skipping plugin benchmark: {}", e);
            return;
        }
    };

    let mut pm = PluginManager::new();
    pm.load_plugin(&lib_path).expect("load failed");

    let input = json!({"value": 42});

    // Benchmark chamada Identity (retorna input)
    c.bench_function("plugin_call::identity", |b| {
        b.iter(|| pm.call("identity", input.clone()).expect("call failed"))
    });

    // Benchmark chamada Double (operações simples)
    c.bench_function("plugin_call::double", |b| {
        b.iter(|| pm.call("double", input.clone()).expect("call failed"))
    });

    // Benchmark chamada Const42 (retorna constante)
    c.bench_function("plugin_call::const42", |b| {
        b.iter(|| pm.call("const42", json!(null)).expect("call failed"))
    });
}

fn bench_plugin_load_once(c: &mut Criterion) {
    c.bench_function("plugin_load::once", |b| {
        b.iter(|| {
            let lib_path = compile_test_plugin().unwrap();
            let mut pm = PluginManager::new();
            pm.load_plugin(&lib_path).unwrap();
        })
    });
}

criterion_group!(benches, bench_plugin_call_overhead, bench_plugin_load_once);
criterion_main!(benches);
