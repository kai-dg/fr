use colorize::AnsiColor;
use std::process;

pub struct UserArgs {
    pub replace: String,
    pub with: String,
    pub folder: Option<String>,
    pub modes: Vec<String>,
}

impl UserArgs {
    pub fn new(args: Vec<String>) -> Self {
        if args.len() == 0 {
            println!("{}", "ERROR: No arguments were provided.".red());
            process::exit(1);
        }

        let mut replace: String = "".to_string();
        let mut with: String = "".to_string();
        let mut folder: Option<String> = Default::default();
        let mut modes: Vec<String> = vec![];
        // loop args and fill them in

        let ok_flags: Vec<String> = vec![String::from("--strict")];

        let mut iter = args.iter().peekable();

        while let Some(arg) = iter.next() {
            if arg.starts_with("--") && ok_flags.contains(arg) {
                modes.push(arg.to_string());
            } else if let Some(eq_pos) = arg.find('=') {
                let parts: Vec<&str> = arg.split('=').collect();
                let arg_type = &parts[0].to_lowercase();
                let arg_value = &parts[1];
                match arg_type.as_str() {
                    "replace" => {
                        replace = arg_value.to_string();
                    }
                    "with" => {
                        with = arg_value.to_string();
                    }
                    "folder" => {
                        folder = Some(arg_value.to_string());
                    }
                    _ => {
                        println!("{} is not an argument.", eq_pos.to_string().red());
                        process::exit(1);
                    }
                }
            }
        }

        if replace == "" {
            println!("{}", "replace argument not found.".red());
            process::exit(1);
        }

        if with == "" {
            println!("{}", "with argument not found.".red());
            process::exit(1);
        }

        UserArgs {
            replace,
            with,
            folder,
            modes,
        }
    }
}
