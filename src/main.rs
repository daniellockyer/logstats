fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    let mut log_watcher = logwatcher::LogWatcher::register(&args[1])?;
    let mut histogram = histogram::Histogram::new();

    let mut i = 0;

    log_watcher.watch(&mut move |line: String| {
        let time = (line.split(" ").last().unwrap().parse::<f64>().unwrap() * 1000.0) as u64;
        histogram.increment(time).unwrap();

        i += 1;

        if i % 10 == 0 {
            println!(
                "Percentiles: p50: {} p90: {} p99: {} p999: {} - Latency: Min: {} Avg: {} Max: {} StdDev: {}",
                histogram.percentile(50.0).unwrap(),
                histogram.percentile(90.0).unwrap(),
                histogram.percentile(99.0).unwrap(),
                histogram.percentile(99.9).unwrap(),
                histogram.minimum().unwrap(),
                histogram.mean().unwrap(),
                histogram.maximum().unwrap(),
                histogram.stddev().unwrap(),
            );
        }

        logwatcher::LogWatcherAction::None
    });

    Ok(())
}
