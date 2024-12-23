use std::{path::Path, process::exit};
pub type InputValidator = fn(input: String) -> bool;

pub fn read_option(
    stdin: &std::io::Stdin,
    prompt: &str,
    validator: Option<InputValidator>,
) -> String {
    // display the prompt.
    println!("{}", prompt);

    // read the input response.
    let mut buf = String::new();
    match stdin.read_line(&mut buf) {
        Ok(_) => {}
        Err(error) => {
            println!("Error while reading stdin. Please restart the tool.\nError: {error}");
            exit(1);
        }
    }

    // trim the newline that's always present when sending a line of stdin by pressing enter.
    buf = buf
        .trim_end_matches("\r\n")
        .trim_end_matches("\n")
        .to_string();

    // we apply a custom input-validator if provided.
    // if it fails, recursively attempt to get a proper input.
    if let Some(func) = validator {
        let input_was_valid = func(buf.clone());
        if !input_was_valid {
            println!("Invalid option {}. Retrying..", buf);
            return read_option(stdin, prompt, validator);
        }
    }

    return buf;
}
pub fn read_several_options(stdin: &std::io::Stdin, init_prompt: &str) -> Vec<String> {
    println!("{}", init_prompt);
    let mut options: Vec<String> = Vec::new();

    // read several option responses from a single prompt.
    // this works until 'done' is inputted.
    loop {
        let mut buf = String::new();
        match stdin.read_line(&mut buf) {
            Ok(_) => {}
            Err(error) => {
                println!("Error while reading stdin. Please restart the tool.\nError: {error}");
                exit(1);
            }
        }
        buf = buf
            .trim_end_matches("\r\n")
            .trim_end_matches("\n")
            .to_string();
        if buf.is_empty() {
            println!("not adding empty option");
            continue;
        }
        if buf == "done" {
            println!("done fetching options");
            break;
        } else {
            let response = read_option(
                stdin,
                format!("adding library {}. Are you sure? [y/n]", buf.clone()).as_str(),
                Some(validate_yes_no_response),
            );

            if response.to_lowercase() == "n" {
                println!("not adding {}. retry, or type 'done' when finished.", buf);
                continue;
            }
        }

        options.push(buf);
    }
    return options;
}
pub fn validate_yes_no_response(input: String) -> bool {
    return input == "y" || input == "n";
}
pub fn validate_lang_response(input: String) -> bool {
    return input == "c" || input == "cpp" || input == "c++";
}
pub fn validate_stdlib_response(input: String) -> bool {
    return input == "latest" || input.starts_with("-std=");
}
pub fn change_directory_if_needed() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 3 && (args[1] == "--directory" || args[1] == "-d") {
        let path = Path::new(args[2].as_str());

        if !path.exists() {
            println!("the provided directory did not exist: {:?}", path);
            exit(1);
        }

        match std::env::set_current_dir(path) {
            Ok(_) => {}
            Err(error) => {
                println!(
                    "Unable to switch current directory to provided path: {:?}\n Error: {error}",
                    path
                );
                exit(1);
            }
        }
    }
}
