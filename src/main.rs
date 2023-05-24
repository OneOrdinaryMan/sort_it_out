use clap::Parser;
use sort_it_out::sorter::SortStruct;
use std::io;
#[derive(Parser)]
#[command(author, version, about, long_about=None)]
struct Cli {
    input: Vec<String>,
}
fn main() {
    let cli = Cli::parse();

    let mut input: Vec<String> = Vec::new();

    if cli.input.len() == 0 {
        let lines = io::stdin().lines();
        for line in lines {
            input.push(line.expect("Error"));
        }
    } else {
        for item in cli.input {
            input.push(item);
        }
    }

    let input = input.iter().map(|x| x.as_str()).collect();

    let mut sorter_1 = SortStruct::new(input);
    sorter_1.quick_sort();
    sorter_1.print_vector();
}
