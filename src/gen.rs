pub enum Language {
    C,
    Cpp,
}

impl Language {
    fn get_compiler(&self) -> &str {
        match self {
            Language::C => "clang",
            Language::Cpp => "clang++",
        }
    }
    fn get_extension(&self) -> &str {
        match self {
            Language::C => "c",
            Language::Cpp => "cpp",
        }
    }
}
impl From<String> for Language {
    fn from(value: String) -> Self {
        match value.as_str() {
            "c" => Language::C,
            "c++" |
            "cpp" => Language::Cpp,
            _ => {
                panic!("invalid language option");
            }
        }
    }
}

pub struct StdLibOption {
    pub value: String,
}

impl From<(bool, String)> for StdLibOption {
    fn from(options: (bool, String)) -> Self {
        let (is_cpp, standard) = options;
        let std = if standard == "latest" {
            if is_cpp {
                "-std=c++2b"
            } else {
                "-std=c2x"
            }
        } else {
            standard.as_str()
        };

        return Self {
            value: std.to_string(),
        };
    }
}

#[derive(PartialEq, Debug)]
pub enum DirectoryOptions {
    UseNeither,
    UseSource,
    UseInclude,
    UseBoth,
}
impl DirectoryOptions {
    pub fn get_paths(&self, include: &mut &str, src: &mut &str) {
        match self {
            DirectoryOptions::UseNeither => {
                *include = "";
                *src = "";
            }
            DirectoryOptions::UseSource => *src = "src",
            DirectoryOptions::UseInclude => *include = "-Iinclude",
            DirectoryOptions::UseBoth => {
                *src = "src";
                *include = "-Iinclude"
            }
        }
    }
}

impl From<(bool, bool)> for DirectoryOptions {
    fn from(src_include: (bool, bool)) -> Self {
        match src_include {
            (false, false) => DirectoryOptions::UseNeither,
            (true, false) => DirectoryOptions::UseSource,
            (false, true) => DirectoryOptions::UseInclude,
            (true, true) => DirectoryOptions::UseBoth,
        }
    }
}

pub struct GeneratorOptions {
    pub stdlib: StdLibOption,
    pub lang: Language,
    pub dirs: DirectoryOptions,
    pub libraries: Vec<String>,
    pub name: String,
}

impl GeneratorOptions {
    pub fn new(
        proj_name: String,
        use_src: bool,
        use_include: bool,
        lang: String,
        standard: String,
        libraries: Vec<String>,
    ) -> Self {
        return Self {
            dirs: DirectoryOptions::from((use_src, use_include)),
            stdlib: StdLibOption::from((lang == "cpp" || lang == "c++", standard)),
            libraries,
            lang: Language::from(lang),
            name: proj_name,
        };
    }
    pub fn get_libraries(&self) -> String {
        self.libraries
            .iter()
            .map(|lib| format!("-l{}", lib))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

pub fn generate_makefile(options: &GeneratorOptions) -> String {
    // get c++ or c compiler.
    let compiler = options.lang.get_compiler();

    let ext = options.lang.get_extension();

    let mut include: &str = "";
    let mut src_dir: &str = "";
    options.dirs.get_paths(&mut include, &mut src_dir);

    let libs_formatted = options.get_libraries();

    // the variables section of the makefile.
    let make_vars = format!(
        "COMPILER := {}\n\
        COMPILER_FLAGS := {} {}\n\
        LD_FLAGS := {}\n\
        OBJ_DIR := objs\n\
        BIN_DIR := bin",
        compiler, options.stdlib.value, include, libs_formatted
    );

    let src_ext_pattern = if options.dirs == DirectoryOptions::UseBoth
        || options.dirs == DirectoryOptions::UseSource
    {
        format!("src/%.{ext}")
    } else {
        format!("%.{ext}")
    };

    // where to search for source files when making.
    // either match all .c files or only those in /src.
    let srcs_wildcard = format!("$(wildcard {}*.{})", src_dir, ext,);

    // create a pattern for finding .o files.
    // again, this depends on if using /src or not.
    let objs = objs_str(src_ext_pattern.clone(), &srcs_wildcard);

    // generate the rest of the makefile, mostly targets.
    let make_file = makefile_str(
        src_ext_pattern.clone(),
        srcs_wildcard,
        objs,
        options.name.clone(),
    );

    // the full makefile.
    return make_vars + "\n\n" + &make_file;
}

pub fn objs_str(src_ext_pattern: String, wildcard: &String) -> String {
    format!(
        "$(patsubst {},$(OBJ_DIR)/%.o,{})",
        src_ext_pattern, wildcard
    )
}

pub fn makefile_str(
    src_ext_pattern: String,
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
        wildcard, objs, proj_name, proj_name, src_ext_pattern, proj_name, proj_name
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
