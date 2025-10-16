use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use dpscript::{
    dpscript::{
        ast::ast::AST,
        lexer::FullLexer,
        tokenizer::{Token, Tokenizer},
    },
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

    tkn.tokens()
}

fn run_lexer(tokens: Vec<Spanned<Token>>, lines: usize) -> AST {
    let lex = FullLexer::new(
        "benchmark".into(),
        "benchmark".into(),
        "benchmark.dps".into(),
        BENCH_CODE.repeat(lines).into(),
        true,
        tokens,
    );

    let res = lex.run();

    assert!(res.is_ok());

    res.unwrap()
}

fn bench_lexer(c: &mut Criterion) {
    c.bench_function("run lexer 100x", |b| {
        b.iter_batched(
            || tokenize(100),
            |data| run_lexer(data, 100),
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(lexer, bench_lexer);
criterion_main!(lexer);
