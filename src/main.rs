pub mod gen;
pub mod input;

use crate::gen::*;
use crate::input::*;

use std::{path::Path, process::exit};

fn main() {
    check_bashrc_for_alias();
    let stdin = std::io::stdin();
    // cd to a provided directory if the user passes in a path.
    change_directory_if_needed();

    // throw an error if the makefile already exists.
    if Path::new("Makefile").exists() {
        println!("This directory already contains a makefile. If you want to create a new sub-project, provide the (--d or --directory) flag followed by directory of where to create it.\n Usage bob <--d/--directory> root_of_project");
        exit(1);
    }

    // read cmd line options
    let proj_name = read_option(&stdin, "Enter a project name", None);
    // read the language option.
    // currently, only C and C++ are supported.
    let lang = read_option(&stdin, "Language? [c/c++]", Some(validate_lang_response));

    // read the stdlib option.
    // use 'latest' for the latest release for the chosen compiler.
    let standard = read_option(
        &stdin,
        "Standard library to use? [latest/-std=c++2b/-std=c11 etc]",
        Some(validate_stdlib_response),
    );

    // Use a source directory?
    // inline converted to a bool.
    let use_src = read_option(
        &stdin,
        "Use a 'src' dir? [y/n]",
        Some(validate_yes_no_response),
    )
    .to_lowercase()
        == "y";

    // Use an include directory?
    // inline converted to a bool.
    let use_include = read_option(
        &stdin,
        "Use an 'include' dir? [y/n]",
        Some(validate_yes_no_response),
    )
    .to_lowercase()
        == "y";

    let libraries = read_several_options(&stdin, "Enter any libraries you want to link against  (one at a time, enter to send) and type 'done' when you're finished.");

    // generate the makefile.
    let make_content = generate_makefile(&GeneratorOptions::new(
        proj_name,
        use_src,
        use_include,
        lang.clone(),
        standard,
        libraries,
    ));

    // write out files.
    match std::fs::write("Makefile", make_content) {
        Ok(_) => {}
        Err(error) => {
            println!("Unable to write the makefile.\nError: {error}");
            exit(1);
        }
    }
    if use_include {
        match std::fs::create_dir("include") {
            Ok(_) => {}
            Err(error) => {
                println!("Unable to create the 'include' directory. Error: {error}");
                exit(1);
            }
        }
    }

    let ext = if lang == "cpp" || lang == "c++" {
        "cpp"
    } else {
        "c"
    };
    if use_src {
        match std::fs::create_dir("src") {
            Ok(_) => {}
            Err(error) => {
                println!("Unable to create the 'src' directory. Error: {error}");
                exit(1);
            }
        }
        create_main_c_file(format!("src/main.{}", ext).as_str());
    } else {
        create_main_c_file(format!("main.{}", ext).as_str());
    }
}
