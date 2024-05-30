use std::fs::File;
use std::io::Read;

fn err(value: Result<usize, std::io::Error>) {
    match value {
        Ok(_) => {}
        Err(err) => panic!("{:?}", err),
    }
}

fn read_option(stdin: &std::io::Stdin, prompt: &str) -> String {
    println!("{}", prompt);
    let mut buf = String::new();
    err(stdin.read_line(&mut buf));
    buf = buf.trim_end_matches("\n").to_string();
    return buf;
}

fn read_several_options(stdin: &std::io::Stdin, init_prompt: &str) -> Vec<String> {
    println!("{}", init_prompt);
    let mut libs: Vec<String> = Vec::new();
    loop {
        let mut buf = String::new();
        err(stdin.read_line(&mut buf));
        buf = buf.trim_end_matches("\n").to_string();
        if buf == "done" {
            break;
        }

        libs.push(buf);
    }
    return libs;
}
fn create_makefile(
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
        "CC := clang\nCFLAGS := -std=c2x {}\nLD_FLAGS := {}\nOBJ_DIR := objs\nBIN_DIR := bin",
        include, libs_formatted
    );

    let wildcard = format!("$(wildcard {}*.c)", src_dir);
    let objs = if use_src_dir {
        format!("$(patsubst src/%.c,$(OBJ_DIR)/%.o,{})", wildcard)
    } else {
        format!("$(patsubst %.c,$(OBJ_DIR)/%.o,{})", wildcard)
    };

   let make_target_all = if use_src_dir {
    format!("SRCS := {}\nOBJS := {}\n\nall: directories {}\n\ndirectories:\n\tmkdir -p $(OBJ_DIR) $(BIN_DIR)\n\n{}: $(OBJS)\n\t$(CC) $(CFLAGS) $(LD_FLAGS) -o $(BIN_DIR)/$@ $^\n\n$(OBJ_DIR)/%.o: src/%.c\n\tmkdir -p $(@D)\n\t$(CC) $(CFLAGS) -c $< -o $@\n\nclean:\n\trm -rf $(OBJ_DIR) $(BIN_DIR)", wildcard, objs, proj_name, proj_name)
} else {
    format!("SRCS := {}\nOBJS := {}\n\nall: directories {}\n\ndirectories:\n\tmkdir -p $(OBJ_DIR) $(BIN_DIR)\n\n{}: $(OBJS)\n\t$(CC) $(CFLAGS) $(LD_FLAGS) -o $(BIN_DIR)/$@ $^\n\n$(OBJ_DIR)/%.o: %.c\n\tmkdir -p $(@D)\n\t$(CC) $(CFLAGS) -c $< -o $@\n\nclean:\n\trm -rf $(OBJ_DIR) $(BIN_DIR)", wildcard, objs, proj_name, proj_name)
};

let makefile_content = format!("{}\n\n{}", make_vars, make_target_all);

return makefile_content;}
fn main() {
    let args: Vec<String> = std::env::args().collect();

    let _pwd = &args[0];

    let stdin = std::io::stdin();

    let proj_name = read_option(&stdin, "Enter a project name");
    let use_src_dir = read_option(&stdin, "Use a 'src' dir? [y/n]");
    let use_inc_dir = read_option(&stdin, "Use a 'include' dir? [y/n]");
    let libraries = read_several_options(&stdin, "Enter any libraries you want to link against (omit the -l) and type 'done' when you're finished.");

    let content = create_makefile(proj_name, use_src_dir == "y", use_inc_dir == "y", libraries);

    println!("{}", content);

    std::fs::write("Makefile", content).unwrap();
    let c_boiler_plate =  "#include <stdio.h>\nint main(int argc, char *argv[]) {\nreturn 0;\n}";
    if use_src_dir.to_lowercase() == "y" {
        std::fs::create_dir("src").unwrap();
        std::fs::write("src/main.c", c_boiler_plate).unwrap();
    } else {
        std::fs::write("main.c", c_boiler_plate).unwrap();
    }
    if use_inc_dir.to_lowercase() == "y" {
        std::fs::create_dir("include").unwrap();
    }
}
