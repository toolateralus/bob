

type InputValidator = fn(input: String) -> bool;

fn read_option(stdin: &std::io::Stdin, prompt: &str, validator: Option<InputValidator>) -> String {
    println!("{}", prompt);
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end_matches("\n").to_string();
    
    if validator.is_some() {
        let v = validator.unwrap();
        if !v(buf.clone()) {
            println!("Invalid option. Retrying..");
            return read_option(stdin, prompt, validator);
        }
    }
    
    return buf;
}

fn read_several_options(stdin: &std::io::Stdin, init_prompt: &str) -> Vec<String> {
    println!("{}", init_prompt);
    let mut libs: Vec<String> = Vec::new();
    loop {
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();
        buf = buf.trim_end_matches("\n").to_string();

        if buf.is_empty() {
            println!("not adding empty option");
            continue;
        }
        if buf == "done" {
            println!("done fetching options");
            break;
        }

        libs.push(buf);
    }
    return libs;
}

fn generate_makefile(
    proj_name: String,
    use_src_dir: bool,
    use_inc_dir: bool,
    lang : String,
    standard: String,
    libraries: Vec<String>,
) -> String {
    
    
    let mut is_cpp = false;
    let lang = lang.to_lowercase();
    // get c++ or c compiler.
    let compiler = if lang == "c++" || lang == "cpp" {
        is_cpp = true;
        "clang++"
    } else if lang == "c"  {
        "clang"
    } else {
        panic!("unknown: cannot find a compiler for language: {}", lang);
    };
    
    // get the std libarary to use
    let std = if standard == "latest" {
        if is_cpp {
            "-std=c++2b"
        } else {
            "-std=c2x"
        }
    } else {
        standard.as_str()
    };
    
    let include = if use_inc_dir { "-Iinclude" } else { "" };
    
    // join the libraries and prepend the -l prefix.
    // -lm, -lraylib etc.
    let libs_formatted: String = libraries
        .iter()
        .map(|lib| format!("-l{}", lib))
        .collect::<Vec<_>>()
        .join(" ");

    let src_dir = if use_src_dir { "src/" } else { "" };

    let make_vars = format!(
        "COMPILER := {}\n\
        COMPILER_FLAGS := {} {}\n\
        LD_FLAGS := {}\n\
        OBJ_DIR := objs\n\
        BIN_DIR := bin",
        compiler, std,
        include, libs_formatted
    );
    
    let srcs_wildcard = format!("$(wildcard {}*.c)", src_dir);
    
    let objs = objs_str(use_src_dir, &srcs_wildcard);

    let make_file = makefile_str(use_src_dir, srcs_wildcard, objs, proj_name);

    return make_vars + "\n\n" + &make_file;
}

fn objs_str(use_src_dir: bool, wildcard: &String) -> String {
    if use_src_dir {
        format!("$(patsubst src/%.c,$(OBJ_DIR)/%.o,{})", wildcard)
    } else {
        format!("$(patsubst %.c,$(OBJ_DIR)/%.o,{})", wildcard)
    }
}

fn makefile_str(
    use_src_dir: bool,
    wildcard: String,
    objs: String,
    proj_name: String,
) -> String {
    format!(
        "SRCS := {}\n\
        OBJS := {}\n\
        \n\
        all: directories {}\n\
        \n\
        directories:\n\
        \tmkdir -p $(OBJ_DIR) $(BIN_DIR)\n\
        \n\
        {}: $(OBJS)\n\
        \t$(COMPILER) $(COMPILER_FLAGS) -o $(BIN_DIR)/$@ $^ $(LD_FLAGS)\n\
        \n\
        $(OBJ_DIR)/%.o: {}\n\
        \tmkdir -p $(@D)\n\
        \t$(COMPILER) $(COMPILER_FLAGS) -c $< -o $@\n\
        \n\
        clean:\n\
        \trm -rf $(OBJ_DIR) $(BIN_DIR)\n\
        \n\
        run: {}\n\
        \t./$(BIN_DIR)/{}",
        wildcard, objs, proj_name, proj_name, if use_src_dir {"src/%.c"} else { "%.c" },proj_name, proj_name
    )
    
}

fn create_main_c_file(path: &str) {
    let c_code = "#include <stdio.h>\nint main(int argc, char *argv[]) {\n\treturn 0;\n}";
    std::fs::write(path, c_code).unwrap();
}

fn main() {
    let stdin = std::io::stdin();
    
    // read cmd line options
    
    let proj_name = read_option(&stdin, "Enter a project name", None);
    
    let lang = read_option(&stdin, "Language? [c/c++]", Some(|input: String| {
        let input = input.to_lowercase();
        return input == "c" || input == "cpp" || input == "c++";
    }));
    
    let standard = read_option(&stdin, "Standard library to use? [latest/-std=c++2b/-std=c11 etc]", Some(|input: String|{
        if input == "latest" {
            return true;
        }
        if input.starts_with("-std=") {
            return true;
        }
        return false;
    }));
    
    let use_src = read_option(&stdin, "Use a 'src' dir? [y/n]", Some(|input: String| {
        let input = input.to_lowercase();
        return input == "y" || input == "n";
    })).to_lowercase() == "y";
    
    let use_include = read_option(&stdin, "Use an 'include' dir? [y/n]", Some(|input: String| {
        let input = input.to_lowercase();
        return input == "y" || input == "n";
    })).to_lowercase() == "y";
    
    let libraries = read_several_options(&stdin, "Enter any libraries you want to link against  (one at a time, enter to send) and type 'done' when you're finished.");
    
    // generate the makefile.
    let content = generate_makefile(proj_name, use_src, use_include, lang, standard, libraries);
    
    // write out files.
    std::fs::write("Makefile", content).unwrap();
    if use_include {
        std::fs::create_dir("include").unwrap();
    }
    
    if use_src {
        std::fs::create_dir("src").unwrap();
        create_main_c_file("src/main.c");
    } else {
        create_main_c_file("main.c");
    }
}
