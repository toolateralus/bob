### to install
- you need `make` to use `Makefile`s. to get it:
 
`apt install make`

- simply clone the repo, run 

`cargo b --release`

- then in your `.bashrc` (I don't know how to install this on windows, but there's no platform specific code.)
add: 

`alias bob='path/to/src/target/release/bob'`


then, once you've installed it, restart your terminal to reload your bash configuration.


### to use

make sure you've followed the previous installation steps. you should have a cmd line program called `bob`

- navigate to wherever you want your new project to go. for example, let's say we're in `~/source/c/my_new_project`
- run `bob`
- instructions will come up on screen in the form of several prompts (binary name, use src/include dirs, libraries to link against)

once completed, you'll have a `Makefile`, `main.c` and the optional dirs if specified. running `make` will create `bin` and `objs` directories, and your program will be located in `bin`.

simple as that.
