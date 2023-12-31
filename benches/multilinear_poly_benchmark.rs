use ark_ff::{Fp64, MontBackend, MontConfig};
use ark_poly::{DenseMultilinearExtension, MultilinearExtension};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use thaler::polynomial::multilinear_poly::MultiLinearPolynomial;

#[derive(MontConfig)]
#[modulus = "17"]
#[generator = "3"]
pub struct FqConfig;
pub type Fq = Fp64<MontBackend<FqConfig, 1>>;

fn poly_a() -> MultiLinearPolynomial<Fq> {
    // p = 2ab + 8bc
    MultiLinearPolynomial::new(
        3,
        vec![
            (Fq::from(2), vec![true, true, false]),
            (Fq::from(8), vec![false, true, true]),
        ],
    )
    .unwrap()
}

fn poly_a_evaluations() -> Vec<Fq> {
    // p = 2ab + 8bc
    let p = poly_a();
    let mut evaluations = vec![];
    evaluations.push(
        p.evaluate(&[Fq::from(0), Fq::from(0), Fq::from(0)])
            .unwrap(),
    );
    evaluations.push(
        p.evaluate(&[Fq::from(0), Fq::from(0), Fq::from(1)])
            .unwrap(),
    );
    evaluations.push(
        p.evaluate(&[Fq::from(0), Fq::from(1), Fq::from(0)])
            .unwrap(),
    );
    evaluations.push(
        p.evaluate(&[Fq::from(0), Fq::from(1), Fq::from(1)])
            .unwrap(),
    );
    evaluations.push(
        p.evaluate(&[Fq::from(1), Fq::from(0), Fq::from(0)])
            .unwrap(),
    );
    evaluations.push(
        p.evaluate(&[Fq::from(1), Fq::from(0), Fq::from(1)])
            .unwrap(),
    );
    evaluations.push(
        p.evaluate(&[Fq::from(1), Fq::from(1), Fq::from(0)])
            .unwrap(),
    );
    evaluations.push(
        p.evaluate(&[Fq::from(1), Fq::from(1), Fq::from(1)])
            .unwrap(),
    );
    evaluations
}

fn arkworks_dense_poly() -> DenseMultilinearExtension<Fq> {
    DenseMultilinearExtension::from_evaluations_slice(3, &poly_a_evaluations())
}

fn arkworks_multilinear_poly_evaluation_benchmark(c: &mut Criterion) {
    c.bench_function("arkworks multlinear evaluation", |b| {
        let poly = arkworks_dense_poly();
        b.iter(|| {
            let result = poly
                .evaluate(&[Fq::from(2), Fq::from(5), Fq::from(10)])
                .unwrap();
        });
    });
}

fn multilinear_poly_evaluation_benchmark(c: &mut Criterion) {
    c.bench_function("multilinear evaluation", |b| {
        let poly = poly_a();
        b.iter(|| poly.evaluate(&[Fq::from(2), Fq::from(5), Fq::from(10)]));
    });
}

fn multilinear_poly_addition_benchmark(c: &mut Criterion) {
    c.bench_function("multilinear addition", |b| {
        let pa = poly_a();
        let pb = poly_a();
        b.iter(|| {
            let result = (&pa + &pb).unwrap();
        });
    });
}

fn multilinear_poly_multiplication_benchmark(c: &mut Criterion) {
    c.bench_function("multilinear multiplication", |b| {
        let pa = poly_a();
        let pb = poly_a();
        b.iter(|| {
            let result = &pa * &pb;
        });
    });
}

fn multlinear_poly_interpolation_benchmark(c: &mut Criterion) {
    c.bench_function("multlinear interpolation", |b| {
        b.iter(|| {
            let pa = MultiLinearPolynomial::<Fq>::interpolate(&[
                Fq::from(2),
                Fq::from(4),
                Fq::from(8),
                Fq::from(20),
                Fq::from(2),
                Fq::from(4),
                Fq::from(8),
                Fq::from(20),
                Fq::from(2),
                Fq::from(4),
                Fq::from(8),
                Fq::from(20),
                Fq::from(2),
                Fq::from(4),
                Fq::from(8),
                Fq::from(20),
            ]);
        });
    });
}

fn multilinear_poly_scalar_mul_benchmark(c: &mut Criterion) {
    c.bench_function("multilinear scalar multiplication", |b| {
        b.iter(|| {
            let p = poly_a();
            p.scalar_multiply(&Fq::from(2));
        });
    });
}

criterion_group!(
    benches,
    arkworks_multilinear_poly_evaluation_benchmark,
    multilinear_poly_evaluation_benchmark,
    multilinear_poly_addition_benchmark,
    multilinear_poly_multiplication_benchmark,
    multlinear_poly_interpolation_benchmark,
    multilinear_poly_scalar_mul_benchmark
);

criterion_main!(benches);
