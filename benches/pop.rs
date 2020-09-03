use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use rust_bheap_bench::*;

#[path = "./utils.rs"]
mod utils;
use utils::{INPUTS, get_sizes};

pub fn pop(c: &mut Criterion) {
    fn helper(c: &mut Criterion, name: &str, get_input: fn(usize) -> Vec<isize>) {
        let mut group = c.benchmark_group("pop_".to_string() + name);

        for size in get_sizes() {
            let input = get_input(size);
            let std_bheap = std_bheap::BinaryHeap::from(input.clone());
            let better_bheap = better_bheap::BinaryHeap::from(input.clone());
            bench_binaryheap!(
                group,
                size,
                (
                    || std_bheap.clone(),
                    "std binaryheap",
                    |mut bheap: std_bheap::BinaryHeap<isize>| {
                        while !bheap.is_empty() {
                            black_box(bheap.pop());
                        }
                    }
                ),
                (
                    || better_bheap.clone(),
                    "better binaryheap",
                    |mut bheap: better_bheap::BinaryHeap<isize>| {
                        while !bheap.is_empty() {
                            black_box(bheap.pop());
                        }
                    }
                )
            )
        }

        group.finish();
    }

    for &(name, get_input) in INPUTS {
        helper(c, name, get_input);
    }
}

criterion_group!(pop_bench, pop);
criterion_main!(pop_bench);
