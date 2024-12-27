use criterion::{criterion_group, criterion_main, Criterion};
use reverse::{Params, Reverse};

fn generate_signal() -> f32 {
  fastrand::f32() * 2. - 1.
}

fn generate_signal_stream(length: usize) -> Vec<f32> {
  (0..length).map(|_| generate_signal()).collect()
}

fn reverse_bench(c: &mut Criterion) {
  let mut reverse = Reverse::new(44100.);
  let mut params = Params::new(44100.);
  params.set(250., 0.5, 0.5);
  let signal_stream = generate_signal_stream(44100);
  c.bench_function("reverse", |b| {
    b.iter(|| {
      for signal in &signal_stream {
        reverse.process(*signal, &mut params);
      }
    })
  });
}

criterion_group!(benches, reverse_bench);
criterion_main!(benches);
