use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use std::fmt::Write;
use std::str::FromStr;
use vls::Vls;

/// Build a vls string with `n` constraints, e.g. ">=1.0.0|!=1.0.1|<=1.0.2|..."
fn build_vls_string(n: usize) -> String {
    let comparators = [">=", "!=", "<=", ">", "<", "=", ""];
    let mut buf = String::new();
    for i in 0..n {
        if i > 0 {
            buf.push('|');
        }
        let cmp = comparators[i % comparators.len()];
        write!(
            buf,
            "{}{}.{}.{}",
            cmp,
            i / 10_000,
            (i / 100) % 1000,
            i % 100
        )
        .unwrap();
    }
    buf
}

fn bench_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("vls_parse");

    for count in [100, 1_000, 10_000, 50_000, 100_000] {
        let input = build_vls_string(count);
        group.bench_with_input(
            BenchmarkId::new("constraints", count),
            &input,
            |b, input| {
                b.iter(|| Vls::from_str(input).unwrap());
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_parsing);
criterion_main!(benches);
