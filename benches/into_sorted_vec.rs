use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use bheap_bench::*;

#[path = "./utils.rs"]
mod utils;
use utils::{INPUTS, get_sizes};

pub fn into_sorted_vec(c: &mut Criterion) {
    fn helper(c: &mut Criterion, name: &str, get_input: fn(usize) -> Vec<isize>) {
        let mut group = c.benchmark_group("into_sorted_vec_".to_string() + name);

        for size in get_sizes() {
            let input = get_input(size);
            bench_binaryheap!(
                group,
                size,
                (
                    || std_bheap::BinaryHeap::from(input.clone()),
                    "std binaryheap",
                    |bheap: std_bheap::BinaryHeap<isize>| { black_box(bheap.into_sorted_vec()); }
                ),
                (
                    || better_bheap::BinaryHeap::from(input.clone()),
                    "better binaryheap",
                    |bheap: better_bheap::BinaryHeap<isize>| { black_box(bheap.into_sorted_vec()); }
                )
            )
        }

        group.finish();
    }

    for &(name, get_input) in INPUTS {
        helper(c, name, get_input);
    }
}

criterion_group!(into_sorted_vec_bench, into_sorted_vec);
criterion_main!(into_sorted_vec_bench);