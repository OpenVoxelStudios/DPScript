use criterion::{Criterion, criterion_group, criterion_main};

const INPUT: &str = include_str!("../../../std/src/gm/sqrt.dps");

pub fn parse_sqrt(c: &mut Criterion) {
    c.bench_function("parse sqrt.dps", |b| {
        b.iter(|| parser_v2::FileParser::parse("sqrt.dps", "std::gm::sqrt", "std", INPUT).unwrap())
    });
}

criterion_group!(benches, parse_sqrt);
criterion_main!(benches);
