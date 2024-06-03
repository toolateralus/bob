# Bob the makefile builder
A simple rust command line tool to streamline creating boilerplate for C++ and C projects using `make`.


## Prerequisites
- `rust` compiler.
- the `clang` and `clang++` compilers are what the makefile will default to using.
- you'll also need `make` to run the makefiles :P


## Installing

To add a command line alias to your linux bash shell, you can run 

`cargo b --release ; cargo r`

and follow the prompt. Simply answer yes to the prompt asking if you want to add the alias.

Note: You must restart your terminal session to be able to use this new alias.

## Usage

_TLDR_: run `bob` or `bob --d <target_directory>` and follow the prompt.

make sure you've followed the previous installation steps. you should have a command line program called `bob`

there are 2 modes for running `bob`

- No arguments, run it in the directory you want your project to be created.

- with the `--d` or `--directory` argument:
  - The tool will run at the specified directory, only if it exists.
  
  
once you run `bob` in your desired mode, it's as simple as answering the prompts. 

A few notes about the available options:

### _Languages_
  Right now only C and C++ are supported.
  `clang` and `clang++` are the compilers that are supported.
  
  _if you'd like `gcc` support, please make an issue requesting this feature_.
  
### _Standard Libraries_
  you can use the `latest` option for the latest known stdlib for your chosen compiler, or you can enter the exact library you want.
  
  You must provide the entire library option, such as `-std=c++11` or whatever you want to use.

### _Libraries_
  at the end of the creation, there will be a prompt to add any libraries you want to link to. simply typing `done` will exit this prompt,
  otherwise it will loop and add as many libraries you want.
  
  You should omit the `-l` prefix for any library you want. If you wanted to link against `raylib`,
  you'd type `raylib`, press enter, then type done and it will finish the project creation. There is no way for us to check if you misspelled a library name, so be careful!