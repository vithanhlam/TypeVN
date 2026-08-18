use criterion::{black_box, Criterion, Throughput};
use typevn_core::{KeyEvent, VietnameseEngine};

fn run_n(n: usize) {
    let mut eng = VietnameseEngine::new();
    let seq: Vec<char> = "tieengs duwowngf hom nay "
        .chars()
        .cycle()
        .take(n)
        .collect();
    for c in seq {
        black_box(eng.process_key(KeyEvent::from_char(c)));
    }
}

fn percentiles(samples: &[u64]) -> (f64, f64, f64, u64) {
    let mut v = samples.to_vec();
    v.sort_unstable();
    let n = v.len();
    let avg = v.iter().sum::<u64>() as f64 / n as f64;
    let p95 = v[(n * 95) / 100] as f64;
    let p99 = v[(n * 99) / 100] as f64;
    let max = *v.last().unwrap_or(&0);
    (avg, p95, p99, max)
}

fn bench_keypress(c: &mut Criterion) {
    let mut group = c.benchmark_group("typevn_core");
    for n in [100_000usize, 1_000_000] {
        group.throughput(Throughput::Elements(n as u64));
        group.bench_function(format!("{n}_keypress"), |b| {
            b.iter(|| run_n(black_box(n)));
        });
    }
    group.finish();

    // One-shot latency distribution (not criterion plot)
    let mut eng = VietnameseEngine::new();
    let seq: Vec<char> = "tieengs ".chars().cycle().take(100_000).collect();
    let mut samples = Vec::with_capacity(seq.len());
    for c in seq {
        let t0 = std::time::Instant::now();
        black_box(eng.process_key(KeyEvent::from_char(c)));
        samples.push(t0.elapsed().as_nanos() as u64);
    }
    let (avg, p95, p99, max) = percentiles(&samples);
    eprintln!(
        "typevn-core 100k latency ns: avg={avg:.1} p95={p95:.0} p99={p99:.0} max={max} throughput={:.0}/s",
        1_000_000_000.0 / avg.max(1.0)
    );
}

fn main() {
    let mut c = Criterion::default().configure_from_args();
    bench_keypress(&mut c);
    c.final_summary();
}
