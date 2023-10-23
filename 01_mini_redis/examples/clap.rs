use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(name = "Hello Clap", version = "1.0.0", author, about, long_about = None)]
struct Cli {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Cli::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}