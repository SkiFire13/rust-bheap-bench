use rand::prelude::*;

fn asc_sorted(size: usize) -> Vec<isize> {
    (0..size as isize).collect()
}
fn desc_sorted(size: usize) -> Vec<isize> {
    (0..size as isize).rev().collect()
}
fn rotated(size: usize) -> Vec<isize> {
    (1..size as isize).chain(std::iter::once(0)).collect()
}
fn asc_desc(size: usize) -> Vec<isize> {
    (0..size as isize)
        .step_by(2)
        .chain((1..size as isize).step_by(2).rev())
        .collect()
}
fn shuffled(size: usize) -> Vec<isize> {
    let mut rng = rand::thread_rng();
    let mut vec = (0..size as isize).collect::<Vec<_>>();
    vec.shuffle(&mut rng);
    vec
}
fn random10(size: usize) -> Vec<isize> {
    let mut rng = rand::thread_rng();
    [0, 1]
        .iter()
        .copied()
        .cycle()
        .take(size * 4)
        .choose_multiple(&mut rng, size)
}

#[allow(dead_code)]
pub fn get_sizes() -> impl IntoIterator<Item = usize> {
    static SIZES: &[usize] = &[
        1, 2,
        7, 8,
        15, 16,
        63, 64,
        255, 256
    ];
    SIZES.iter().copied()
}

pub const INPUTS: &'static[(&'static str, fn(usize) -> Vec<isize>)] = &[
    ("shuffled", shuffled),
    ("random10", random10),
    ("desc_sorted", desc_sorted),
    ("rotated", rotated),
    ("asc_sorted", asc_sorted),
    ("asc_desc", asc_desc),
];

#[macro_export]
macro_rules! bench_binaryheap {
    (
        $group:ident,
        $size:ident,
        $((
            $input_transform:expr,
            $name:literal,
            $bench_func:expr
        )),*
    ) => {{
        $(
            let input = ($input_transform)();
            $group.bench_with_input(BenchmarkId::new($name.to_string(), $size), &$size, |b, _| {
                b.iter_batched(
                    || input.clone(),
                    |input| ($bench_func)(black_box(input)),
                    BatchSize::SmallInput,
                );
            });
        )*
    }};
}