use std::hint::black_box;
use criterion::{ criterion_group, criterion_main, BenchmarkId, Criterion };
use basezip::compressor::huffman_tree::Node;

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

fn benchmark_huffman(c: &mut Criterion) {
    let mut group = c.benchmark_group("huffman_tree");

    let sizes = [
        1024 * 1024, // 1 MiB
        4 * 1024 * 1024, // 4 MiB
        16 * 1024 * 1024, // 16 MiB
    ];

    for size in sizes {
        let datasets = [
            ("uniform", generate_uniform(size)),
            ("skewed", generate_skewed(size)),
            ("random_like", generate_random_like(size)),
        ];

        for (distribution, data) in datasets {
            group.bench_with_input(
                BenchmarkId::new("vec", format!("{distribution}_{size}")),
                &data,
                |b, data| {
                    b.iter(|| { Node::build_with_vec(black_box(data.as_slice())) });
                }
            );

            group.bench_with_input(
                BenchmarkId::new("heap", format!("{distribution}_{size}")),
                &data,
                |b, data| {
                    b.iter(|| { Node::build_with_heap(black_box(data.as_slice())) });
                }
            );
        }
    }

    group.finish();
}

criterion_group!(benches, benchmark_huffman);
criterion_main!(benches);
