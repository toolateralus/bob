fn handle_io_err(value: Result<usize, std::io::Error>) {
    match value {
        Ok(_) => {}
        Err(err) => panic!("{:?}", err),
    }
}

fn read_option(stdin: &std::io::Stdin, prompt: &str) -> String {
    println!("{}", prompt);
    let mut buf = String::new();
    handle_io_err(stdin.read_line(&mut buf));
    buf = buf.trim_end_matches("\n").to_string();
    return buf;
}

fn read_several_options(stdin: &std::io::Stdin, init_prompt: &str) -> Vec<String> {
    println!("{}", init_prompt);
    let mut libs: Vec<String> = Vec::new();
    loop {
        let mut buf = String::new();
        handle_io_err(stdin.read_line(&mut buf));
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
    libraries: Vec<String>,
) -> String {
    let include = if use_inc_dir { "-Iinclude" } else { "" };

    let libs_formatted: String = libraries
        .iter()
        .map(|lib| format!("-l{}", lib))
        .collect::<Vec<_>>()
        .join(" ");

    let src_dir = if use_src_dir { "src/" } else { "" };

    let make_vars = format!(
        "CC := clang\n\
        CFLAGS := -std=c2x {}\n\
        LD_FLAGS := {}\n\
        OBJ_DIR := objs\n\
        BIN_DIR := bin",
        include, libs_formatted
    );

    let srcs_wildcard = format!("$(wildcard {}*.c)", src_dir);

    let objs = generate_objs(use_src_dir, &srcs_wildcard);

    let make_file = generate_make_file(use_src_dir, srcs_wildcard, objs, proj_name);

    return make_vars + "\n\n" + &make_file;
}

fn generate_objs(use_src_dir: bool, wildcard: &String) -> String {
    if use_src_dir {
        format!("$(patsubst src/%.c,$(OBJ_DIR)/%.o,{})", wildcard)
    } else {
        format!("$(patsubst %.c,$(OBJ_DIR)/%.o,{})", wildcard)
    }
}

fn generate_make_file(
    use_src_dir: bool,
    wildcard: String,
    objs: String,
    proj_name: String,
) -> String {
    if use_src_dir {
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
            \t$(CC) $(CFLAGS) -o $(BIN_DIR)/$@ $^ $(LD_FLAGS)\n\
            \n\
            $(OBJ_DIR)/%.o: src/%.c\n\
            \tmkdir -p $(@D)\n\
            \t$(CC) $(CFLAGS) -c $< -o $@\n\
            \n\
            clean:\n\
            \trm -rf $(OBJ_DIR) $(BIN_DIR)\n\
            \n\
            run: {}\n\
            \t./$(BIN_DIR)/{}",
            wildcard, objs, proj_name, proj_name, proj_name, proj_name
        )
    } else {
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
            \t$(CC) $(CFLAGS) -o $(BIN_DIR)/$@ $^ $(LD_FLAGS)\n\
            \n\
            $(OBJ_DIR)/%.o: %.c\n\
            \tmkdir -p $(@D)\n\
            \t$(CC) $(CFLAGS) -c $< -o $@\n\
            \n\
            clean:\n\
            \trm -rf $(OBJ_DIR) $(BIN_DIR)\n\
            \n\
            run: {}\n\
            \t./$(BIN_DIR)/{}",
            wildcard, objs, proj_name, proj_name, proj_name, proj_name
        )
    }
}

fn create_main_c(path: &str) {
    let c_boiler_plate = "#include <stdio.h>\nint main(int argc, char *argv[]) {\n\treturn 0;\n}";
    std::fs::write(path, c_boiler_plate).unwrap();
}

fn main() {
    let stdin = std::io::stdin();

    let proj_name = read_option(&stdin, "Enter a project name");
    let use_src = read_option(&stdin, "Use a 'src' dir? [y/n]").to_lowercase() == "y";
    let use_include = read_option(&stdin, "Use a 'include' dir? [y/n]").to_lowercase() == "y";
    let libraries = read_several_options(&stdin, "Enter any libraries you want to link against (omit the -l) and type 'done' when you're finished.");

    // generate the makefile.
    let content = generate_makefile(proj_name, use_src, use_include, libraries);

    // write out files.
    std::fs::write("Makefile", content).unwrap();
    if use_include {
        std::fs::create_dir("include").unwrap();
    }
    
    if use_src {
        std::fs::create_dir("src").unwrap();
        create_main_c("src/main.c");
    } else {
        create_main_c("main.c");
    }
}
