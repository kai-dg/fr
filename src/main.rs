use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        let base_command = args[0].clone();
        eprintln!("Usage: {base_command} <command> [args...]")
    }
    let to_replace = args[1].clone();
    let replace_with = args[2].clone();
    let arguments = vec![to_replace.to_owned(), "--files-with-matches".to_owned()];
    let output = Command::new("rg")
        .args(arguments)
        .output()
        .expect("ERROR: failed to execute process");
    let output_string = String::from_utf8_lossy(&output.stdout);
    let filenames = output_string.trim().split('\n');
    for file in filenames {
        let path = PathBuf::from(file);
        let contents = fs::read_to_string(&path).expect("ERROR: failed to read file.");
        let new_contents = contents.replace(&to_replace, &replace_with);
        fs::write(&path, new_contents).expect("ERROR: failed to write file");
    }
}
