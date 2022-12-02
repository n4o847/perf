use std::{io, time::Duration};

use clap::Parser;
use perf::{diskstats::read_diskstats_io, stats::read_stat_cpu};
use tokio::time;

/// System performance monitoring tool
#[derive(Parser, Debug)]
struct Args {
    /// Device name to be monitored
    #[arg(short, long)]
    device: String,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let args = Args::parse();

    println!("# elapsed time (ms) | reads completed (/s) | sectors read (/s) | writes completed (/s) | sectors written (/s) | cpu usage (10ms/s) | cpu* usage (10ms/s) ...");
    let mut interval = time::interval(Duration::from_secs(1));
    let instant = interval.tick().await;
    let stat = read_diskstats_io(&args.device)?;
    let first_instant = instant;
    let mut last_stat = stat;
    let stat_cpu = read_stat_cpu()?;
    let mut last_stat_cpu = stat_cpu;
    loop {
        let instant = interval.tick().await;
        print!("{} ", (instant - first_instant).as_millis());
        let stat = read_diskstats_io(&args.device)?;
        print!("{} ", stat.diff(&last_stat));
        last_stat = stat;
        let stat_cpu = read_stat_cpu()?;
        println!("{}", stat_cpu.diff(&last_stat_cpu));
        last_stat_cpu = stat_cpu;
    }
}
