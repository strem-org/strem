use std::{fs, io::BufReader, path::PathBuf, time::Duration};

use criterion::{BenchmarkId, Criterion};
use strem::{
    compiler::Compiler,
    datastream::{
        frame::Frame,
        reader::DataReader,
        reader::{DataRead, Sample},
        DataStream,
    },
    matcher::Matcher,
};

const ROOTDIR: &str = env!("CARGO_MANIFEST_DIR");
const NSAMPLES: usize = 25;

fn matchit(matcher: &mut Matcher, frames: &[Frame]) {
    let mut start = frames.len();

    while start > 0 {
        matcher.reset();
        if let Some(m) = matcher.run(&frames[..start]) {
            start = m.frames.first().unwrap().index;

            continue;
        }

        start -= 1;
    }
}

fn objects(frames: &[Frame]) -> usize {
    let mut count = 0;
    for frame in frames.iter() {
        for sample in &frame.samples {
            match sample {
                Sample::ObjectDetection(d) => {
                    for annotations in d.annotations.values() {
                        count += annotations.len();
                    }
                }
            }
        }
    }

    count
}

fn bench_matcher(c: &mut Criterion) {
    let mut queries = PathBuf::from(self::ROOTDIR);
    queries.push("benches/data/queries/");

    let mut data = PathBuf::from(self::ROOTDIR);
    data.push("benches/data/datasets/woven/train.json");

    // We first want to load all the data into memory because we are not
    // concerned with testing the loading time but rather just the matching time.
    let mut reader = DataReader::new(BufReader::new(fs::File::open(data).unwrap()));
    let mut datastream = DataStream::new();

    let mut index = 0;
    while let Some(sample) = reader.next().unwrap() {
        let mut frame = Frame::new(index);
        frame.samples.push(sample);

        datastream.frames.push(frame);
        index += 1;
    }

    // After the data is loaded, we need to create the appropriate components to
    // perform matching for each new query as these are separate components that
    // should be ran. However, we first must setup the `criterion` package's
    // testing framwork components.
    //
    // This includes the criterion group.
    let mut group = c.benchmark_group("Matching Benchmarks");

    // For each query, we want to evaluate the matching performance from 0 to N
    // frames provided by the dataset.
    let queries = fs::read_dir(queries).unwrap();

    for query in queries {
        let query = query.unwrap().path();
        let filename = query.file_name().unwrap().to_str().unwrap();

        let pattern = fs::read_to_string(&query).unwrap();

        // Setup the matching frameworks.
        let ast = Compiler::new().compile(&pattern).unwrap();
        let mut matcher = Matcher::from(&ast);

        // For the actually testing, the test procedures are as follows:
        //
        // 1. For each query, we perform a match with 0 to N frames.
        // 2. We also would like to track the throughput of each matching procedure for each 0 to j frames.
        let stream = &datastream.frames;
        let step = stream.len() / self::NSAMPLES;

        for i in (0..stream.len()).step_by(step) {
            let nobjects = self::objects(&stream[0..i]);
            let frames = &stream[0..i];

            group.throughput(criterion::Throughput::Elements(nobjects as u64));
            group.bench_with_input(
                BenchmarkId::new(format!("{} ({})", filename, nobjects), i),
                &frames,
                |b, &frames| b.iter(|| self::matchit(&mut matcher, frames)),
            );
        }
    }
}

// The group of benchmarks associated with general performance evaluation of the
// matching framework used by STREM.
criterion::criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10).measurement_time(Duration::from_secs(30));
    targets = bench_matcher
}

criterion::criterion_main! {
    benches
}
