use criterion::{criterion_group, criterion_main, Criterion};
use tokio::runtime::Runtime;
use tupa_core::pipeline;
use tupa_engine::Executor;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Input {
    value: i64,
}

fn step1(input: &Input) -> i64 {
    input.value * 2
}
fn step2(input: &Input) -> i64 {
    input.value * 3
}
fn step3(input: &Input) -> i64 {
    input.value + 10
}

pipeline! {
    name: BenchPipeline,
    input: Input,
    steps: [
        step("s1") { step1(input) },
        step("s2") { step2(input) },
        step("s3") { step3(input) }
    ],
    constraints: [
        metric("s1").gt(0),
        metric("s2").lt(200)
    ]
}

// Pipeline with independent steps for parallel benchmark
pipeline! {
    name: IndepPipeline,
    input: Input,
    steps: [
        step("p1") { step1(input) },
        step("p2") { step2(input) },
        step("p3") { step3(input) },
        step("p4") { step1(input) },
        step("p5") { step2(input) }
    ],
    constraints: []
}

// Pipeline with DAG dependencies for parallel benchmarking
pipeline! {
    name: DagPipeline,
    input: Input,
    steps: [
        step("a") { step1(input) } produces ["a_val"],
        step("b") { step2(input) } produces ["b_val"] requires ["a_val"],
        step("c") { step3(input) } produces ["c_val"] requires ["a_val"]
    ],
    constraints: []
}

fn bench_sequential(c: &mut Criterion) {
    let plan = BenchPipeline::new();
    let executor = Executor::new();
    let input = Input { value: 42 };

    c.bench_function("sequential::3steps_with_constraints", |b| {
        b.iter(|| executor.run(&plan, &input).unwrap())
    });
}

fn bench_parallel(c: &mut Criterion) {
    let plan = IndepPipeline::new();
    let executor = Executor::new();
    let input = Input { value: 42 };

    // Build a runtime once
    let rt = Runtime::new().unwrap();

    c.bench_function("parallel::5steps_independent", |b| {
        b.iter(|| rt.block_on(executor.run_parallel(&plan, &input)).unwrap())
    });
}

fn bench_dag_parallel(c: &mut Criterion) {
    let plan = DagPipeline::new();
    let executor = Executor::new();
    let input = Input { value: 42 };
    let rt = Runtime::new().unwrap();

    c.bench_function("parallel::dag_3_steps_sequential_deps", |b| {
        b.iter(|| rt.block_on(executor.run_parallel(&plan, &input)).unwrap())
    });
}

fn bench_constraint_throughput(c: &mut Criterion) {
    // Create a pipeline with many constraints to stress constraint checking
    pipeline! {
        name: ManyConstraintsPipeline,
        input: Input,
        steps: [
            step("compute") { step1(input) }
        ],
        constraints: [
            metric("compute").gt(0),
            metric("compute").lt(1000),
            metric("compute").ne(0),
            metric("compute").ge(10),
            metric("compute").le(10000)
        ]
    }

    let plan = ManyConstraintsPipeline::new();
    let executor = Executor::new();
    let input = Input { value: 42 };

    c.bench_function("constraint_check::5_checks", |b| {
        b.iter(|| executor.run(&plan, &input).unwrap())
    });
}

fn bench_metrics_collection(c: &mut Criterion) {
    let plan = IndepPipeline::new();
    let executor = Executor::new();
    let input = Input { value: 42 };
    let rt = Runtime::new().unwrap();

    c.bench_function("metrics::collection_5_steps", |b| {
        b.iter(|| rt.block_on(executor.run_parallel(&plan, &input)).unwrap())
    });
}

fn bench_executor_new(c: &mut Criterion) {
    c.bench_function("executor::new", |b| b.iter(|| Executor::new()));
}

criterion_group!(
    benches,
    bench_sequential,
    bench_parallel,
    bench_dag_parallel,
    bench_constraint_throughput,
    bench_metrics_collection,
    bench_executor_new
);
criterion_main!(benches);
