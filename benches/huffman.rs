use std::hint::black_box;
use criterion::{ criterion_group, criterion_main, BenchmarkId, Criterion };

fn generate_uniform(size: usize) -> Vec<u8> {
    (0..size).map(|i| (i % 256) as u8).collect()
}

fn generate_skewed(size: usize) -> Vec<u8> {
    let mut data = Vec::with_capacity(size);

    for i in 0..size {
        let value = match i % 100 {
            0..=69 => b'A',
            70..=84 => b'B',
            85..=94 => b'C',
            _ => (i % 256) as u8,
        };

        data.push(value);
    }

    data
}

fn generate_random_like(size: usize) -> Vec<u8> {
    let mut state: u32 = 12345;
    let mut data = Vec::with_capacity(size);

    for _ in 0..size {
        // Deterministic pseudo-random number generator.
        state = state.wrapping_mul(1664525).wrapping_add(1013904223);
        data.push((state >> 24) as u8);
    }

    data
}

fn benchmark_window(c: &mut Criterion) {
    let mut group = c.benchmark_group("lz77_window_comparison");

    let sizes = [1024 * 1024, 4 * 1024 * 1024, 16 * 1024 * 1024];

    for size in sizes {
        let datasets = [
            ("uniform", generate_uniform(size)),
            ("skewed", generate_skewed(size)),
            ("random_like", generate_random_like(size)),
        ];
    }

    group.finish();
}

criterion_group!(benches, benchmark_window);
criterion_main!(benches);
