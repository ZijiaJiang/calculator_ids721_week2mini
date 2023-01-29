/* An adding calculator that will take two integer and  return their sum*/

use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Zijia Jiang",
    about = "An adding calculator"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Zijia Jiang")]
    Add {
        #[clap(short, long)]
        x: i32,
        #[clap(short, long)]
        y: i32,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Add { x, y }) => {
            let sum = calculator::add(x, y);
            println!("The sum of {} and {} is {}", x, y, sum);
        }
        None => println!("No command given"),
    }
}
