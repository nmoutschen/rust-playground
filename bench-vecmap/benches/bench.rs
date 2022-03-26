use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::{
    collections::{BTreeMap, HashMap},
};

const SIZE: usize = 100000;

fn create<T>() -> T
where
    T: FromIterator<(String, String)>,
{
    (0..SIZE).map(|i| (format!("key {}", i), format!("item {}", i))).collect::<T>()
}

fn uppercase_vec(c: &mut Criterion) {
    let input: Vec<_> = create();

    c.bench_function("vec-vec", |b| b.iter(|| {
        let _result: Vec<_> = black_box(&input).iter().map(|(k, v)| (k.to_uppercase(), v.to_uppercase())).collect();
    }));
    c.bench_function("vec-hashmap", |b| b.iter(|| {
        let _result: HashMap<_, _> = black_box(&input).iter().map(|(k, v)| (k.to_uppercase(), v.to_uppercase())).collect();
    }));
    c.bench_function("vec-btreemap", |b| b.iter(|| {
        let _result: BTreeMap<_, _> = black_box(&input).iter().map(|(k, v)| (k.to_uppercase(), v.to_uppercase())).collect();
    }));
}
fn uppercase_hashmap(c: &mut Criterion) {
    let input: HashMap<_, _> = create();

    c.bench_function("hashmap-vec", |b| b.iter(|| {
        let _result: Vec<_> = black_box(&input).iter().map(|(k, v)| (k.to_uppercase(), v.to_uppercase())).collect();
    }));
    c.bench_function("hashmap-hashmap", |b| b.iter(|| {
        let _result: HashMap<_, _> = black_box(&input).iter().map(|(k, v)| (k.to_uppercase(), v.to_uppercase())).collect();
    }));
    c.bench_function("hashmap-btreemap", |b| b.iter(|| {
        let _result: BTreeMap<_, _> = black_box(&input).iter().map(|(k, v)| (k.to_uppercase(), v.to_uppercase())).collect();
    }));
}

fn uppercase_btreemap(c: &mut Criterion) {
    let input: BTreeMap<_, _> = create();

    c.bench_function("btreemap-vec", |b| b.iter(|| {
        let _result: Vec<_> = black_box(&input).iter().map(|(k, v)| (k.to_uppercase(), v.to_uppercase())).collect();
    }));
    c.bench_function("btreemap-hashmap", |b| b.iter(|| {
        let _result: HashMap<_, _> = black_box(&input).iter().map(|(k, v)| (k.to_uppercase(), v.to_uppercase())).collect();
    }));
    c.bench_function("btreemap-btreemap", |b| b.iter(|| {
        let _result: BTreeMap<_, _> = black_box(&input).iter().map(|(k, v)| (k.to_uppercase(), v.to_uppercase())).collect();
    }));
}

criterion_group!(benches, uppercase_vec, uppercase_hashmap, uppercase_btreemap);
criterion_main!(benches);