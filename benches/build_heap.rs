use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use bheap_bench::*;

#[path = "./utils.rs"]
mod utils;
use utils::{INPUTS, get_sizes};

pub fn build_heap(c: &mut Criterion) {
    fn helper(c: &mut Criterion, name: &str, get_input: fn(usize) -> Vec<isize>) {
        let mut group = c.benchmark_group("build_heap_".to_string() + name);

        for size in get_sizes() {
            let input = get_input(size);
            bench_binaryheap!(
                group,
                size,
                (
                    || input.clone(),
                    "std binaryheap",
                    |vec| { black_box(std_bheap::BinaryHeap::from(vec)); }
                ),
                (
                    || input.clone(),
                    "better binaryheap",
                    |vec| { black_box(better_bheap::BinaryHeap::from(vec)); }
                )
            )
        }

        group.finish();
    }

    for &(name, get_input) in INPUTS {
        helper(c, name, get_input);
    }
}

criterion_group!(build_heap_bench, build_heap);
criterion_main!(build_heap_bench);
