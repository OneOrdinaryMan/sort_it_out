use clap::Parser;
use sort_it_out::sorter::SortStruct;
use std::{fmt::Display, io};
#[derive(Parser)]
#[command(author, version, about, long_about=None)]
struct Cli {
    input: Vec<String>,
    #[arg(short, long)]
    reverse: bool,
    #[arg(short, long)]
    integer: bool,
    #[arg(short, long)]
    sort: Option<String>,
}

enum InputType<'a> {
    Slice(Vec<&'a str>),
    Integer(Vec<isize>),
}

fn sort_function<T: Copy + PartialOrd + Display>(
    input: Vec<T>,
    sort: Option<String>,
    reverse: bool,
) {
    let mut sorter_1 = SortStruct::new(input);
    match sort {
        Some(value) => match value.as_str() {
            "bubble" => sorter_1.bubble_sort(),
            "selection" => sorter_1.selection_sort(),
            "insertion" => sorter_1.insertion_sort(),
            "merge" => sorter_1.merge_sort(),
            "quick" => sorter_1.quick_sort(),
            _ => {
                println!("Cannot find the sort method.");
                std::process::exit(1);
            }
        },
        None => sorter_1.quick_sort(),
    }
    if reverse {
        sorter_1.reverse();
    }
    sorter_1.print_vector();
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
    let input_type = match cli.integer {
        true => InputType::Integer(
            input
                .into_iter()
                .map(|x| x.parse().expect("Not an integer"))
                .collect(),
        ),
        false => InputType::Slice(input.iter().map(|x| x.as_str()).collect()),
    };
    match input_type {
        InputType::Integer(i) => sort_function(i, cli.sort, cli.reverse),
        InputType::Slice(s) => sort_function(s, cli.sort, cli.reverse),
    };
}
