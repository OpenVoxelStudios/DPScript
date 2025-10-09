use criterion::{Criterion, criterion_group, criterion_main};
use dpscript::dpscript::tokenizer::Tokenizer;

const BENCH_CODE: &str = r#"
const test: str = "aaa";

pub fn thing(#[this] other: int) -> int {
    return -3 * 45 * 493 + 75 - 12;
}
"#;

fn tokenize(lines: usize) {
    let mut tkn = Tokenizer::new("benchmark.dps", BENCH_CODE.repeat(lines));
    let res = tkn.run();

    assert!(res.is_ok());
}

fn bench_tokenizer(c: &mut Criterion) {
    c.bench_function("tokenize 100x", |b| b.iter(|| tokenize(100)));
}

criterion_group!(tokenizer, bench_tokenizer);
criterion_main!(tokenizer);
