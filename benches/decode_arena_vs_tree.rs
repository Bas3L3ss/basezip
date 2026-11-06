use criterion::{ criterion_group, criterion_main, BenchmarkId, Criterion };
use basezip::decode::Decoder;
use basezip::encode::Encoder;

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
        state = state.wrapping_mul(1664525).wrapping_add(1013904223);
        data.push((state >> 24) as u8);
    }

    data
}

fn benchmark_huffman(c: &mut Criterion) {
    let mut group = c.benchmark_group("huffman_decode");

    let sizes = [1024 * 1024, 4 * 1024 * 1024, 16 * 1024 * 1024];

    for size in sizes {
        let datasets = [
            ("uniform", generate_uniform(size)),
            ("skewed", generate_skewed(size)),
            ("random_like", generate_random_like(size)),
        ];

        for (distribution, data) in datasets {
            // Compression and header parsing are excluded from measurement.
            let compressed = Encoder::encode(&data);
            let (input_slice, header) = Decoder::prep(&compressed.data).unwrap();

            group.bench_with_input(
                BenchmarkId::new("pointer_tree", format!("{distribution}_{size}")),
                &input_slice,
                |b, input| {
                    b.iter(|| { Decoder::with_tree(black_box(input), black_box(header)).unwrap() });
                }
            );

            group.bench_with_input(
                BenchmarkId::new("arena_tree", format!("{distribution}_{size}")),
                &input_slice,
                |b, input| {
                    b.iter(|| {
                        Decoder::with_arena(black_box(input), black_box(header)).unwrap()
                    });
                }
            );
        }
    }

    group.finish();
}

use std::hint::black_box;

criterion_group!(benches, benchmark_huffman);
criterion_main!(benches);
