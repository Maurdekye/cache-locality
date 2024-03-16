use clap::Parser;
use progress_observer::prelude::*;
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::{
    error::Error,
    fs::File,
    io::{stdout, Write},
    path::PathBuf,
    time::{Duration, Instant, SystemTime},
};

#[derive(Parser)]
struct Args {
    /// Amount of memory to allocate for test
    #[clap(short, long, default_value_t = 1024 * 1024 * 1024)]
    total_size: usize,

    /// Initial step size
    #[clap(short = 'd', long, default_value_t = 1)]
    initial_step_size: usize,

    /// Maximum step size. Must be >= --initial-step_size
    #[clap(short, long)]
    max_step_size: Option<usize>,

    /// Number of iterations to run per step
    #[clap(short, long, default_value_t = 1_000_000_000)]
    iterations: usize,

    /// Output file to record results to
    #[clap(short, long)]
    out: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    println!("Allocating random data");
    let mem: Vec<u8> = (0..args.total_size)
        .into_par_iter()
        .map(|_| rand::random())
        .collect();

    let mut out = args.out.map(|out| File::create(out)).transpose()?;
    if let Some(out) = &mut out {
        writeln!(
            out,
            "start_time,step_size,total_duration_millis,steps_per_second"
        )?;
    }

    let max_step_size = args.max_step_size.unwrap_or(args.total_size);
    let mut step_size = args.initial_step_size;
    let mut rng = thread_rng();
    while step_size <= max_step_size {
        println!("Testing step size {step_size}");
        let mut sum: u8 = 0;
        let mut position: usize = 0;
        let initial_time = Instant::now();
        for (steps, should_print) in Observer::new_with(
            Duration::from_millis(100),
            Options {
                first_checkpoint: 1000,
                ..Default::default()
            },
        )
        .take(args.iterations)
        .enumerate()
        {
            let step: usize = rng.gen();
            let step = step % step_size;
            if rng.gen() {
                position = position.wrapping_add(step);
            } else {
                position = position.wrapping_sub(step);
            }
            position %= args.total_size;
            sum = sum.wrapping_add(mem[position]);
            if should_print {
                let current_time = Instant::now();
                let duration = current_time.duration_since(initial_time).as_secs_f32();
                let steps_per_second = (steps as f32) / duration;
                print!("\r{steps_per_second:.2} steps/sec");
                stdout().flush().unwrap();
            }
        }
        let total_duration = Instant::now().duration_since(initial_time);
        let total_duration_float = total_duration.as_secs_f32();
        let steps_per_second = (args.iterations as f32) / total_duration_float;
        println!(
            "\rCompleted testing: took {total_duration_float:.3} secs, with an average access rate of {steps_per_second:.2} steps/sec. sum: {sum}"
        );
        if let Some(out) = &mut out {
            let start_time = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis();
            let total_duration_millis = total_duration.as_millis();
            writeln!(
                out,
                "{start_time},{step_size},{total_duration_millis},{steps_per_second}"
            )?;
        }
        step_size <<= 1;
    }

    Ok(())
}