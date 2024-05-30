## to install
dependencies :
`git`
`make`
`rust`

Note: (this is from [rust's website](https://www.rust-lang.org/learn/get-started) and may be outdated.)
to install rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` 

to get make and git: `sudo apt install make git`

- clone the repo: `git clone https://github.com/toolateralus/bob.git`
- build the release: `cargo b --release`

#### adding a cmd line alias for the binary
in your `.bashrc` (I don't know how to install this on windows, but there's no platform specific code.)
add: 

`alias bob='path/to/src/target/release/bob'`

for example, if you've downloaded the source at `~/source/rust/bob`

`alias bob='~/source/rust/bob/target/release/bob`


then, once you've installed it, restart your terminal to reload your bash configuration.

## to use

make sure you've followed the previous installation steps. you should have a cmd line program called `bob`

- navigate to wherever you want your new project to go. for example, let's say we're in `~/source/c/my_new_project`
- run `bob`
- instructions will come up on screen in the form of several prompts (binary name, use src/include dirs, libraries to link against)

once completed, you'll have a `Makefile`, `main.c` and the optional dirs if specified. running `make` will create `bin` and `objs` directories, and your program will be located in `bin`.

simple as that.
