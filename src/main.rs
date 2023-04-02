use dialoguer::console::Term;
use dialoguer::MultiSelect;
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

fn get_all_files(to_replace: String) -> Vec<String> {
    let arguments = vec![to_replace.to_owned(), "--files-with-matches".to_owned()];
    let output = Command::new("rg")
        .args(arguments)
        .output()
        .expect("ERROR: failed to execute process");
    let output_string = String::from_utf8_lossy(&output.stdout);
    let filenames = output_string.trim().split('\n');
    return filenames.map(str::to_string).collect();
}

fn replace_in_file(file: String, to_replace: String, replace_with: String) {
    let path = PathBuf::from(file);
    let contents = fs::read_to_string(&path).expect("ERROR: failed to read file.");
    let new_contents = contents.replace(&to_replace, &replace_with);
    fs::write(&path, new_contents).expect("ERROR: failed to write file");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        let base_command = args[0].clone();
        eprintln!("Usage: {base_command} <command> [args...]")
    }
    let to_replace = args[1].clone();
    let replace_with = args[2].clone();

    let all_files = get_all_files(to_replace.clone());

    let selections = MultiSelect::new()
        .with_prompt("Select files")
        .items(&all_files)
        .interact_on_opt(&&Term::stderr())
        .unwrap();

    match selections {
        Some(indices) => {
            println!("You selected:");
            for index in indices {
                replace_in_file(
                    all_files[index].clone(),
                    to_replace.clone(),
                    replace_with.clone(),
                );
                println!("{}", all_files[index]);
            }
        }
        None => println!("No items selected"),
    }
}
