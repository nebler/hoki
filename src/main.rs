use rayon::prelude::*;
use std::{env, fs, time::SystemTime};

use clap::Parser;
use walkdir::WalkDir;
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 31)]
    days: u8,

    #[arg(short, long, default_value_t = 0)]
    months: u8,
}

fn main() {
    let args = Args::parse();
    let start = std::time::Instant::now();
    // Get the current directory
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let current_time = SystemTime::now();
    // Use WalkDir to traverse the directory and find all folders named "node_modules"
    let results: Vec<_> = WalkDir::new(&current_dir)
        .into_iter()
        .par_bridge()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.path().to_str()?.contains("node_modules") {
                return None;
            }
            let package_json_path = entry.path().join("package.json");
            if package_json_path.exists() && package_json_path.is_file() {
                let mut node_modules = entry.path().join("node_modules");
                if node_modules.exists() && node_modules.is_dir() {
                    println!("Found node_modules folder: {:?}", entry.path());

                    let last_modified_in_days = current_time
                        .duration_since(entry.metadata().unwrap().modified().unwrap())
                        .unwrap()
                        .as_secs_f64()
                        / 86400.0;
                    let too_old_because_of_months =
                        args.months != 0 && last_modified_in_days / 31.0 > f64::from(args.months);
                    let too_old_because_of_days = last_modified_in_days > f64::from(args.days);
                    if (too_old_because_of_months || too_old_because_of_days) {
                        match fs::remove_dir_all(node_modules) {
                            Ok(_) => {
                                println!("node_modules in '{:?}' deleted successfully.", entry)
                            }
                            Err(e) => {
                                eprintln!(
                                    "Error deleting node_modules folder in '{:?}': {}",
                                    entry, e
                                )
                            }
                        };
                        //delete it
                    }
                    Some(())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    println!("found {} projects using node_modules", results.len());
    println!("This too{:?}", start.elapsed());
}
