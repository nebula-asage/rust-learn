mod commands;
mod models;
mod repositories;
mod services;

use commands::user_command::UserCommand;
use std::env;

fn print_usage() {
    println!("Usage:");
    println!("  create <email> <username> <phone> <age>");
    println!("  update <email> <username> <phone> <age>");
    println!("  list");
    println!("  get <email>");
    println!("  delete <email>");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        return;
    }

    let command = UserCommand::new();
    let result = match args[1].as_str() {
        "create" => command.create(&args[2..]),
        "update" => command.update(&args[2..]),
        "list" => command.list(),
        "get" => command.get(&args[2..]),
        "delete" => command.delete(&args[2..]),
        _ => {
            print_usage();
            Ok(())
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }
}
