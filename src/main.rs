use std::{env, f32::consts::E, ops::Sub, os::unix::fs::MetadataExt, time::SystemTime};

use clap::Parser;
use fs_extra::dir::get_size;
use walkdir::WalkDir;
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}
fn bytes_to_mb(bytes: u64) -> f64 {
    const BYTES_IN_MB: f64 = 1024.0 * 1024.0;
    bytes as f64 / BYTES_IN_MB
}

fn main() {
    let args = Args::parse();
    let start = std::time::Instant::now();
    // Get the current directory
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let parent = current_dir.parent().expect("msg");
    let current_time = SystemTime::now();
    println!("{:?}", parent);
    let mut counter = 0;
    // Use WalkDir to traverse the directory and find all folders named "node_modules"
    for entry in WalkDir::new(&parent) {
        let entry = entry.expect("Failed to read entry");
        if (entry.path().to_str().unwrap().contains("node_modules")) {
            continue;
        }
        let package_json_path = entry.path().join("package.json");
        if package_json_path.exists() && package_json_path.is_file() {
            counter += 1;
            let node_modules = entry.path().join("node_modules");
            if (node_modules.exists() && node_modules.is_dir()) {
                println!("Found node_modules folder: {:?}", entry.path());
                println!(
                    "last modified: {:?}",
                    ((current_time
                        .duration_since(entry.metadata().unwrap().modified().unwrap())
                        .unwrap()
                        .as_secs_f64()
                        / 60.0)
                        / 60.0)
                        / 24.0
                );
            }
        }
    }
    println!("found {} projects using node_modules", counter);
    eprintln!("{:?}", start.elapsed());
}
