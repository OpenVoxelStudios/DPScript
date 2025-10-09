use criterion::{Criterion, criterion_group, criterion_main};
use dpscript::{
    dpscript::tokenizer::{Token, Tokenizer},
    util::Spanned,
};

const BENCH_CODE: &str = r#"
const test: str = "aaa";

pub fn thing(#[this] other: int) -> int {
    return -3 * 45 * 493 + 75 - 12;
}
"#;

fn tokenize(lines: usize) -> Vec<Spanned<Token>> {
    let mut tkn = Tokenizer::new("benchmark.dps", BENCH_CODE.repeat(lines));
    let res = tkn.run();

    assert!(res.is_ok());

    res.unwrap().tokens()
}

fn bench_tokenizer(c: &mut Criterion) {
    c.bench_function("tokenizer 100x", |b| b.iter(|| tokenize(100)));
    // c.bench_function("tokenizer 1000x", |b| b.iter(|| tokenize(1000)));
    // c.bench_function("tokenizer 10000x", |b| b.iter(|| tokenize(10000)));
}

criterion_group!(tokenizer, bench_tokenizer);
criterion_main!(tokenizer);
