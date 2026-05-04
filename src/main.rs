mod sequential;
mod parallel;
use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();
    let benchmark = args.contains(&"--benchmark".to_string());
    let parallel = args.contains(&"--parallel".to_string());

    let offset = benchmark as usize + parallel as usize;

    if args.len() != 3 + offset {
        eprintln!("Usage: mandelbrot [--benchmark] [--parallel] <width> <height>");
        std::process::exit(1);
    }

    let width = args[1 + offset].parse::<u32>().unwrap();
    let height = args[2 + offset].parse::<u32>().unwrap();

    println!(
        "Running {} mandelbrot with {}x{}",
        if parallel { "parallel" } else { "sequential" },
        width,
        height
    );

    let img = if parallel {
        parallel::render(width, height)
    } else {
        sequential::render(width, height)
    };

    if !benchmark {
        let filename = if parallel {
            format!("mandelbrot_parallel_{}x{}.png", width, height)
        } else {
            format!("mandelbrot_sequential_{}x{}.png", width, height)
        };
        img.save(filename).unwrap();
    }



}
