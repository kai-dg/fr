use colorize::AnsiColor;
use std::fs;
use std::path::PathBuf;
use std::process;
use std::process::Command;

pub fn get_all_files(to_replace: String) -> Vec<String> {
    let arguments = vec![to_replace.to_owned(), "--files-with-matches".to_owned()];
    let output = Command::new("rg")
        .args(arguments)
        .output()
        .expect("ERROR: failed to execute process");
    let output_string = String::from_utf8_lossy(&output.stdout);
    let filenames = output_string.trim().split('\n');
    return filenames.map(str::to_string).collect();
}

pub fn replace_in_file(file: String, to_replace: String, replace_with: String) {
    let path = PathBuf::from(file);
    let contents = fs::read_to_string(&path).expect("ERROR: failed to read file.");
    let new_contents = contents.replace(&to_replace, &replace_with);
    fs::write(&path, new_contents).expect("ERROR: failed to write file");
}

pub fn ensure_ripgrep() {
    let output = Command::new("rg")
        .arg("--version")
        .output()
        .unwrap_or_else(|error| {
            panic!(
                "{}\nERROR: {}",
                "Ripgrep is probably not installed, please install that first.".red(),
                error
            )
        });

    if !output.status.success() {
        println!("Please install ripgrep first.");
        process::exit(1);
    }
}
