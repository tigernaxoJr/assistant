use std::env;
use std::path::PathBuf;
use assistant::weather;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        return;
    }
    
    match args[1].as_str() {
        "weather" => {
            let output_dir = if args.len() > 2 {
                PathBuf::from(&args[2])
            } else {
                PathBuf::from(".")
            };
            
            println!("Fetching latest CWA weather chart...");
            match weather::download_weather_chart_to(&output_dir) {
                Ok(path) => {
                    println!("Successfully downloaded weather chart to: {}", path.display());
                }
                Err(e) => {
                    eprintln!("Error downloading weather chart: {}", e);
                    std::process::exit(1);
                }
            }
        }
        "help" | "--help" | "-h" => {
            print_usage();
        }
        cmd => {
            eprintln!("Unknown command: {}", cmd);
            print_usage();
            std::process::exit(1);
        }
    }
}

fn print_usage() {
    println!("Personal Assistant CLI Tools");
    println!("Usage:");
    println!("  assistant <command> [args...]");
    println!("\nCommands:");
    println!("  weather [output_dir]  Download the latest CWA weather chart (defaults to current directory)");
    println!("  help                  Show this help menu");
}
