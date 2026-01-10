use clap::Parser;
#[derive(Parser, Debug)]

struct Args {
    #[arg(long, short)]
    name: String,
}

fn main() {
    let args = Args::parse();
    println!("Hi, {:?}", args);
}
