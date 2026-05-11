use criterion::{criterion_group, criterion_main, Criterion};
use tupa_core::pipeline;
use tupa_engine::Executor;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Input { value: i64 }

fn step1(input: &Input) -> i64 { input.value * 2 }
fn step2(input: &Input) -> i64 { input.value * 3 }

pipeline! {
    name: BenchPipeline,
    input: Input,
    steps: [
        step("s1") { step1(input) },
        step("s2") { step2(input) }
    ],
    constraints: []
}

fn bench_sequential(c: &mut Criterion) {
    let plan = BenchPipeline::new();
    let executor = Executor::new();
    let input = Input { value: 42 };
    
    c.bench_function("sequential::2steps", |b| {
        b.iter(|| executor.run(&plan, &input).unwrap())
    });
}

criterion_group!(benches, bench_sequential);
criterion_main!(benches);
