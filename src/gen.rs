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
            "c++" | "cpp" => Language::Cpp,
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
                "-std=c++26"
            } else {
                "-std=c23"
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
            DirectoryOptions::UseSource => *src = "src/",
            DirectoryOptions::UseInclude => *include = "-Iinclude",
            DirectoryOptions::UseBoth => {
                *src = "src/";
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

    let src_ext_pattern = if options.dirs == DirectoryOptions::UseBoth
        || options.dirs == DirectoryOptions::UseSource
    {
        format!("src/%.{ext}")
    } else {
        format!("%.{ext}")
    };

    let name = &options.name;
    let stdlib = &options.stdlib.value;
    // the variables section of the makefile.
    let make_vars = format!(
        "PRJ_NAME := {name}\n\
        COMPILER := {compiler}\n\
        COMPILER_FLAGS := {stdlib} {include}\n\
        LD_FLAGS := {libs_formatted}\n\
        SRC_EXT_PAT := {src_ext_pattern}\n\
        OBJ_DIR := obj\n\
        REL_OBJ_DIR := obj/release\n\
        DBG_OBJ_DIR := obj/debug\n\
        REL_BIN_DIR := bin/release\n\
        DBG_BIN_DIR := bin/debug\n\
        BIN_DIR := bin\n\
        SRCS := $(wildcard {src_dir}*.{ext})"
    );

    // the full makefile.
    return make_vars + "\n\n" + makefile_str();
}

pub fn makefile_str() -> &'static str {
    "REL_OBJS := $(patsubst $(SRC_EXT_PAT), $(REL_OBJ_DIR)/%.o, $(SRCS))\n\
        DBG_OBJS := $(patsubst $(SRC_EXT_PAT), $(DBG_OBJ_DIR)/%.o, $(SRCS))\n\
        \n\
        all: directories $(PRJ_NAME)\n\
        \n\
        directories:\n\
        \tmkdir -p $(REL_OBJ_DIR) $(DBG_OBJ_DIR) $(DBG_BIN_DIR) $(REL_BIN_DIR)\n\
        \n\
        $(PRJ_NAME): $(DBG_OBJS)\n\
        \t$(COMPILER) $(COMPILER_FLAGS) -g -o $(DBG_BIN_DIR)/$(PRJ_NAME) $^ $(LD_FLAGS)\n\
        \n\
        release: $(REL_OBJS)\n\
        \t$(COMPILER) $(COMPILER_FLAGS) -O3 -o $(REL_BIN_DIR)/$(PRJ_NAME) $^ $(LD_FLAGS)\n\
        \n\
        $(REL_OBJ_DIR)/%.o: $(SRC_EXT_PAT)\n\
        \t$(COMPILER) $(COMPILER_FLAGS) -c $< -o $@\n\
        \n\
        $(DBG_OBJ_DIR)/%.o: $(SRC_EXT_PAT)\n\
        \t$(COMPILER) $(COMPILER_FLAGS) -c $< -o $@\n\
        \n\
        clean:\n\
        \trm -rf $(OBJ_DIR) $(BIN_DIR)\n\
        \n\
        run: all $(PRJ_NAME)\n\
        \t./$(DBG_BIN_DIR)/$(PRJ_NAME)\n\
        \n\
        run-release: directories release\n\
        \t./$(REL_BIN_DIR)/$(PRJ_NAME)"
}

pub fn create_main_c_file(path: &str) {
    // write out a basic main.c boilerplate to help the user out.
    let c_code = "#include <stdio.h>\nint main(int argc, char *argv[]) {\n\tprintf(\"hello squirreld\\n\");\n\treturn 0;\n}";
    match std::fs::write(path, c_code) {
        Ok(_) => {}
        Err(error) => {
            println!("Unable to write {path}. Please restart the tool. \nError: {error}");
        }
    }
}
