use criterion::*;

fn benchmark_bitvector_simd(c: &mut Criterion) {
    let b1 = bitvec_simd::BitVec::ones(100_000);
    let b2 = bitvec_simd::BitVec::zeros(100_000);
    c.bench_function("bitvec_simd(this crate)", |b| {
        b.iter(|| {
            black_box(b1.and_cloned(&b2));
        })
    });
}

fn benchmark_bitvector_simd2(c: &mut Criterion) {
    c.bench_function("bitvec_simd(this crate) with creation", |b| {
        b.iter(|| {
            let b1 = bitvec_simd::BitVec::ones(100_000);
            let b2 = bitvec_simd::BitVec::zeros(100_000);
            black_box(b1.and(b2));
        })
    });
}

fn benchmark_bitvector_simd_u16x8(c: &mut Criterion) {
    let b1 = bitvec_simd::BitVecSimd::<wide::u16x8, u16, 8>::ones(100_000);
    let b2 = bitvec_simd::BitVecSimd::<wide::u16x8, u16, 8>::zeros(100_000);
    c.bench_function("bitvec_simd_u16x8(this crate)", |b| {
        b.iter(|| {
            black_box(b1.and_cloned(&b2));
        })
    });
}

fn benchmark_bitvector_simd2_u16x8(c: &mut Criterion) {
    c.bench_function("bitvec_simd_u16x8(this crate) with creation", |b| {
        b.iter(|| {
            let b1 = bitvec_simd::BitVecSimd::<wide::u16x8, u16, 8>::ones(100_000);
            let b2 = bitvec_simd::BitVecSimd::<wide::u16x8, u16, 8>::zeros(100_000);
            black_box(b1.and(b2));
        })
    });
}

fn benchmark_bitvector_bitvec(c: &mut Criterion) {
    let b1 = bit_vec::BitVec::from_elem(100_000, true);
    let b2 = bit_vec::BitVec::from_elem(100_000, false);
    c.bench_function("bit-vec 0.6", |b| {
        b.iter(|| {
            black_box(b1.clone().and(&b2));
        })
    });
}

fn benchmark_bitvector_bitvec2(c: &mut Criterion) {
    c.bench_function("bit-vec 0.6 with creation", |b| {
        b.iter(|| {
            let mut b1 = bit_vec::BitVec::from_elem(100_000, true);
            let b2 = bit_vec::BitVec::from_elem(100_000, false);
            black_box(b1.and(&b2));
        })
    });
}

fn benchmark_bitvector_bitvec_n(c: &mut Criterion) {
    let b1 = bitvec::bitvec![bitvec::order::Msb0, usize; 1; 100_000];
    let b2 = bitvec::bitvec![bitvec::order::Msb0, usize; 0; 100_000];
    c.bench_function("bitvec 0.22", |b| {
        b.iter(|| {
            black_box(b1.clone() & b2.clone());
        })
    });
}

fn benchmark_bitvector_bitvec_n2(c: &mut Criterion) {
    c.bench_function("bitvec 0.22 with creation", |b| {
        b.iter(|| {
            let b1 = bitvec::bitvec![bitvec::order::Msb0, usize; 1; 100_000];
            let b2 = bitvec::bitvec![bitvec::order::Msb0, usize; 0; 100_000];
            black_box(b1 & b2);
        })
    });
}

criterion_group!(
    normal_benches,
    benchmark_bitvector_simd,
    benchmark_bitvector_simd_u16x8,
    benchmark_bitvector_bitvec,
    benchmark_bitvector_bitvec_n
);
criterion_group!(
    with_creation_benches,
    benchmark_bitvector_simd2,
    benchmark_bitvector_simd2_u16x8,
    benchmark_bitvector_bitvec2,
    benchmark_bitvector_bitvec_n2
);
criterion_main!(normal_benches, with_creation_benches);
