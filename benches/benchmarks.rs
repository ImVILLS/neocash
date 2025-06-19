use criterion::{Criterion, black_box, criterion_group, criterion_main};
use neocash::completion::ShellCompleter;
use std::path::PathBuf;

fn bench_command_completion(c: &mut Criterion) {
    let completer = ShellCompleter::new();

    c.bench_function("get_all_commands", |b| {
        b.iter(|| {
            let result = black_box(completer.get_all_commands());
            assert!(!result.is_empty());
        })
    });

    c.bench_function("filter_commands 'ca'", |b| {
        b.iter(|| {
            let result = black_box(completer.filter_commands("ca"));
            assert!(!result.is_empty());
        })
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(20);
    targets = bench_command_completion
);
criterion_main!(benches);
