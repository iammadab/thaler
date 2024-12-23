use criterion::{black_box, criterion_group, criterion_main, Criterion};
use polynomial::multilinear::pairing_index::PairingIndex;
use polynomial::multilinear::pairing_index_2::index_pair;

fn generate_pair_vector(n_vars: usize, index: usize) -> Vec<(usize, usize)> {
    // f(a, b, c)
    // pairing index for a
    let pairing_iterator = PairingIndex::new(n_vars, index).unwrap();
    let shift_value = pairing_iterator.shift_value();
    pairing_iterator
        .map(|val| (val, val + shift_value))
        .collect()
}

fn generate_pair_vector_2(n_vars: u8, index: u8) -> Vec<(usize, usize)> {
    // f(a, b, c);
    index_pair(n_vars, index).collect()
}

pub fn bench_pair_shift_based_algo(c: &mut Criterion) {
    c.bench_function("pair_index_shift_18_var_12_index", |b| {
        b.iter(|| black_box(generate_pair_vector(18, 12)));
    });
    c.bench_function("pair_index_shift_19_var_12_index", |b| {
        b.iter(|| black_box(generate_pair_vector(19, 12)));
    });
    c.bench_function("pair_index_shift_20_var_12_index", |b| {
        b.iter(|| black_box(generate_pair_vector(20, 12)));
    });
    c.bench_function("pair_index_shift_21_var_12_index", |b| {
        b.iter(|| black_box(generate_pair_vector(21, 12)));
    });
}

pub fn bench_pair_bit_insert_algo(c: &mut Criterion) {
    c.bench_function("pair_index_18_var_12_index", |b| {
        b.iter(|| black_box(generate_pair_vector_2(18, 12)));
    });
    c.bench_function("pair_index_19_var_12_index", |b| {
        b.iter(|| black_box(generate_pair_vector_2(19, 12)));
    });
    c.bench_function("pair_index_20_var_12_index", |b| {
        b.iter(|| black_box(generate_pair_vector_2(20, 12)));
    });
    c.bench_function("pair_index_21_var_12_index", |b| {
        b.iter(|| black_box(generate_pair_vector_2(21, 12)));
    });
}

criterion_group!(
    benches,
    bench_pair_bit_insert_algo,
    bench_pair_shift_based_algo,
);
criterion_main!(benches);
