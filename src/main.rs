mod sequential;
mod parallel;
mod parallel_improved;
mod utils;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let benchmark = args.contains(&"--benchmark".to_string());
    let parallel = args.contains(&"--parallel".to_string());
    let parallel2 = args.contains(&"--parallel2".to_string());

    // Count flags
    let offset = (benchmark as usize) + (parallel as usize) + (parallel2 as usize);

    if args.len() != 3 + offset {
        eprintln!("Usage: mandelbrot [--benchmark] [--parallel] [--parallel2] <width> <height>");
        std::process::exit(1);
    }

    let width = args[1 + offset].parse::<u32>().unwrap();
    let height = args[2 + offset].parse::<u32>().unwrap();

    let mode = if parallel2 {
        "parallel_improved"
    } else if parallel {
        "parallel"
    } else {
        "sequential"
    };

    println!("Running {} mandelbrot with {}x{}", mode, width, height);

    let img = if parallel2 {
        parallel_improved::render(width, height)
    } else if parallel {
        parallel::render(width, height)
    } else {
        sequential::render(width, height)
    };

    if !benchmark {
        let filename = format!("mandelbrot_{}_{}x{}.png", mode, width, height);
        img.save(filename).unwrap();
        std::thread::sleep(std::time::Duration::from_secs(2));
        println!("Done!");
    }
}
