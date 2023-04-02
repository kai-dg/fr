use colorize::AnsiColor;
use dialoguer::console::Term;
use dialoguer::MultiSelect;
use std::env;
use std::process;
mod helpers;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        let base_command = args[0].clone();
        eprintln!("Usage: {base_command} <command> [args...]")
    }
    let ref to_replace = args[1];
    let ref replace_with = args[2];

    let all_files = helpers::get_all_files(to_replace.to_string());

    if all_files.len() == 1 && all_files[0].is_empty() == true {
        println!(
            "{} {}",
            "No files found with the word".yellow(),
            to_replace.to_string().black().yellowb()
        );
        process::exit(0);
    }

    let selections = MultiSelect::new()
        .with_prompt("Select files")
        .items(&all_files)
        .interact_on_opt(&&Term::stderr())
        .unwrap();

    match selections {
        Some(indices) => {
            println!(
                "You replaced {} with {} in files:",
                to_replace.to_string().blue(),
                replace_with.to_string().blue()
            );
            for index in indices {
                helpers::replace_in_file(
                    all_files[index].to_string(),
                    to_replace.to_string(),
                    replace_with.to_string(),
                );
                println!("{}", all_files[index].to_string().green());
            }
        }
        None => println!("No items selected"),
    }
    process::exit(0);
}
