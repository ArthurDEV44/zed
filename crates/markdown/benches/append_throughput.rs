//! `Markdown::append` throughput microbenchmark.
//!
//! This isolates string accumulation from parsing and GPUI scheduling. The
//! fixture matches Paneflow's streaming shape: 60 KB across 100 chunks.

use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use gpui::SharedString;

const CHUNKS: usize = 100;
const CHUNK_BYTES: usize = 600;
const TOTAL_BYTES: usize = CHUNKS * CHUNK_BYTES;

fn make_chunks() -> Vec<String> {
    const FIXTURE_BYTES: &[u8] = b"markdown append streaming fixture with code blocks and prose\n";

    (0..CHUNKS)
        .map(|chunk_index| {
            (0..CHUNK_BYTES)
                .map(|byte_index| {
                    FIXTURE_BYTES[(chunk_index + byte_index) % FIXTURE_BYTES.len()] as char
                })
                .collect()
        })
        .collect()
}

fn pre_fix_concat(chunks: &[String]) -> SharedString {
    let mut source = SharedString::new_static("");

    for chunk in chunks {
        source = SharedString::new(source.to_string() + black_box(chunk.as_str()));
    }

    source
}

fn post_fix_buffered(chunks: &[String]) -> SharedString {
    let mut source_buf = String::new();

    for chunk in chunks {
        source_buf.push_str(black_box(chunk.as_str()));
    }

    SharedString::from(source_buf)
}

fn bench_append_throughput(criterion: &mut Criterion) {
    let chunks = make_chunks();
    let mut group = criterion.benchmark_group("markdown_append");
    group.throughput(Throughput::Bytes(TOTAL_BYTES as u64));

    group.bench_function(BenchmarkId::new("pre_fix_concat", "60kb_100x600b"), |b| {
        b.iter(|| {
            let source = pre_fix_concat(black_box(&chunks));
            black_box(source);
        });
    });

    group.bench_function(
        BenchmarkId::new("post_fix_buffered", "60kb_100x600b"),
        |b| {
            b.iter(|| {
                let source = post_fix_buffered(black_box(&chunks));
                black_box(source);
            });
        },
    );

    group.finish();
}

criterion_group!(markdown_append, bench_append_throughput);
criterion_main!(markdown_append);
