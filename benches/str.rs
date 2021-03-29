use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use pipeline::str::total_spaces;
use std::fmt;

struct Parameters {
    basis: &'static str,
}

impl fmt::Display for Parameters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.basis)
    }
}

static PARAMETERS: [Parameters; 4] = [
    Parameters { basis: "a b c" },
    Parameters {
        basis: "a b c d e f g h",
    },
    Parameters {
        basis: "a b cccccccccccccccccccccccccccccccccccccccc d",
    },
    Parameters {
        basis: "                    a b ccc d",
    },
];

fn benchmark_total_spaces(c: &mut Criterion) {
    let mut group = c.benchmark_group("total_spaces");
    for param in &PARAMETERS {
        group.throughput(Throughput::Bytes(param.basis.len() as u64));

        group.bench_with_input(BenchmarkId::from_parameter(param), &param, |b, &param| {
            b.iter(|| total_spaces(param.basis))
        });
    }
}

criterion_group!(name = str;
                 config = Criterion::default();
                 targets = benchmark_total_spaces);
criterion_main!(str);
