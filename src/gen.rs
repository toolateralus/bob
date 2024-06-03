pub fn generate_makefile(
    proj_name: String,
    use_src_dir: bool,
    use_inc_dir: bool,
    lang: String,
    standard: String,
    libraries: Vec<String>,
) -> String {
    let mut is_cpp = false;
    let lang = lang.to_lowercase();
    // get c++ or c compiler.
    let compiler = if lang == "c++" || lang == "cpp" {
        is_cpp = true;
        "clang++"
    } else if lang == "c" {
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

    // source directory for using a /src
    let src_dir = if use_src_dir { "src/" } else { "" };

    // the variables section of the makefile.
    let make_vars = format!(
        "COMPILER := {}\n\
        COMPILER_FLAGS := {} {}\n\
        LD_FLAGS := {}\n\
        OBJ_DIR := objs\n\
        BIN_DIR := bin",
        compiler, std, include, libs_formatted
    );

    // where to search for source files when making.
    // either match all .c files or only those in /src.
    let srcs_wildcard = format!("$(wildcard {}*.c)", src_dir);

    // create a pattern for finding .o files.
    // again, this depends on if using /src or not.
    let objs = objs_str(use_src_dir, &srcs_wildcard);

    // generate the rest of the makefile, mostly targets.
    let make_file = makefile_str(use_src_dir, srcs_wildcard, objs, proj_name);

    // the full makefile.
    return make_vars + "\n\n" + &make_file;
}

pub fn objs_str(use_src_dir: bool, wildcard: &String) -> String {
    // create a pattern for finding .o files.
    if use_src_dir {
        format!("$(patsubst src/%.c,$(OBJ_DIR)/%.o,{})", wildcard)
    } else {
        format!("$(patsubst %.c,$(OBJ_DIR)/%.o,{})", wildcard)
    }
}

pub fn makefile_str(
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
        wildcard,
        objs,
        proj_name,
        proj_name,
        if use_src_dir { "src/%.c" } else { "%.c" },
        proj_name,
        proj_name
    )
}

pub fn create_main_c_file(path: &str) {
    // write out a basic main.c boilerplate to help the user out.
    let c_code = "#include <stdio.h>\nint main(int argc, char *argv[]) {\n\treturn 0;\n}";
    match std::fs::write(path, c_code) {
        Ok(_) => {}
        Err(error) => {
            println!("Unable to write {path}. Please restart the tool. \nError: {error}");
        }
    }
}
