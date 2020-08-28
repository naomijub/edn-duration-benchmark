use criterion::{criterion_group, criterion_main, Criterion};
use edn_rs;
use std::str::FromStr;

fn criterion_benchmark(c: &mut Criterion) {
    let edn_str = edn_str();
    let edn = edn_rs::Edn::from_str(&edn_str).unwrap();
    c.bench_function("navigate", |b| b.iter(|| edn[":associates"][0][":role"].clone()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

fn edn_str() -> String {
    "{
        :type :human
        :first-name \"bench\"
        :last-name \"mark\"
        :age 13
        :version 0.13
        :associates [
            {
                :name :julia
                :role :adm
            }
            {
                :name :otavio
                :role :contributor
            }
            {
                :name :juxt
                :role :great-ideas
            }
        ]
    }".to_string()
}