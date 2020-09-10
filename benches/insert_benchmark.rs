use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::prelude::*;
use redblack::tree::RedBlackTree;
use std::collections::BTreeSet;

fn insert(n: u64) {
    let mut tree: RedBlackTree<u64> = Default::default();
    let mut rng = thread_rng();
    for _ in 0..n {
        tree.insert(rng.gen_range(0, n));
    }
    let sorted_vec: Vec<u64> = tree.into();
    for i in 1..sorted_vec.len() {
        if sorted_vec[i] <= sorted_vec[i - 1] {
            panic!("wrong rbtree");
        }
    }
}

fn insert_btree(n: u64) {
    let mut tree: BTreeSet<u64> = Default::default();
    let mut rng = thread_rng();
    for _ in 0..n {
        tree.insert(rng.gen_range(0, n));
    }
    let sorted_vec: Vec<u64> = tree.into_iter().collect();
    for i in 1..sorted_vec.len() {
        if sorted_vec[i] <= sorted_vec[i - 1] {
            panic!("wrong btreeset");
        }
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("insert 0.1 million elements in the self-made rbtree", |b| {
        b.iter(|| insert(black_box(100_000)))
    });
    c.bench_function(
        "insert 0.1 million elements in the official btreeset",
        |b| b.iter(|| insert_btree(black_box(100_000))),
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
