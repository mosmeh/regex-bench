#[macro_use]
extern crate criterion;

use criterion::{BenchmarkId, Criterion};
use regex::Regex;

fn bench(c: &mut Criterion) {
    let range = 5..50;
    let step = 5;
    let frag = "fooあいうbaz";
    let query = "うa";
    let regex = Regex::new(&regex_syntax::escape(query)).unwrap();

    let mut group = c.benchmark_group("contains");
    for i in range.clone().step_by(step) {
        let mut text = String::new();
        for _ in 0..i {
            text.push_str(frag);
        }
        group.bench_with_input(BenchmarkId::new("str::contains", i), &text, |b, text| {
            b.iter(|| text.contains(query))
        });
        group.bench_with_input(BenchmarkId::new("Regex::is_match", i), &text, |b, text| {
            b.iter(|| regex.is_match(text))
        });
    }
    group.finish();

    let mut group = c.benchmark_group("indices");
    for i in range.step_by(step) {
        let mut text = String::new();
        for _ in 0..i {
            text.push_str(frag);
        }
        group.bench_with_input(
            BenchmarkId::new("str::match_indices", i),
            &text,
            |b, text| b.iter(|| text.match_indices(query).collect::<Vec<_>>()),
        );
        group.bench_with_input(BenchmarkId::new("Regex::find_iter", i), &text, |b, text| {
            b.iter(|| regex.find_iter(text).collect::<Vec<_>>())
        });
    }
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
