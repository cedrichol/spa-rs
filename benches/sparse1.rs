use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::distributions::Distribution;
use rand::distributions::Uniform;
use sparse::prelude::*;

fn make_random(m: usize, n: usize, k: usize) -> CoordsMatrix<f32, u32> {
    let mut rng = rand::thread_rng();
    let mx = Uniform::from(1..m);
    let nx = Uniform::from(1..n);
    let kx = Uniform::from(0.0..1.0);

    let i = (0..k).map(|_| mx.sample(&mut rng)).collect();
    let j = (0..k).map(|_| nx.sample(&mut rng)).collect();
    let x = (0..k).map(|_| kx.sample(&mut rng)).collect();

    CoordsMatrix { i, j, x };
}

pub fn sparse_create(c: &mut Criterion) {
    let m = 1000;
    let n = 1000;
    let k = 3000;
    let coo = make_random(m, n, k);

    c.bench_function("make 1000x1000-3000", |b| {
        b.iter(|| {
            SMatrix::from_coords_dedup_accumulate(
                black_box((m as usize, n as usize)),
                black_box(&coo),
            )
        })
    });
}

criterion_group!(benches, sparse_create);
criterion_main!(benches);
