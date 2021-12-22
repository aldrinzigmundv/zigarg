# zigarg

[![Build status](https://travis-ci.com/aldrinzigmundv/zigarg.svg?branch=master)](https://app.travis-ci.com/github/aldrinzigmundv/zigarg) [![](https://img.shields.io/crates/v/zigarg.svg)](https://crates.io/crates/zigarg) [![](https://docs.rs/zigarg/badge.svg)](https://docs.rs/zigarg)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

zigarg is a very light argument parser written fully in Rust. It's not dependent on any third party libraries other than those that Rust already comes with. It lacks many features, like help generation, but enough to be suitable for a lot of applications. It's also good for educational purposes as it isn't very complex.

I decided to publish the library after using it on several private CLI applications I made. I may add additional features in the future, if I find it necessary, but I'm trying to keep it as simple as possible, while still being very useful. Feel free to file an issue or suggest features.

## Quickstart
Add `zigarg` to `Cargo.toml` as a dependency
```
[dependencies]
zigarg = "0.1.0"
```
Capture user's arguments by adding the code below, after you have added zigarg to your dependencies:
```ignore
let arguments = zigarg::new();
```
Use the struct returned from `zigarg::new()` to perform different actions like the examples below
```ignore
//Check if there are arguments provided by the user other than your program's name
let has_arguments = arguments.has_args();
//Check if the arguments provided by the user has a certain flag
let exist = arguments.exist("-q");
//Get the value of an argument
let file_name = arguments.get_value(-f);
```
Check documentation of the Arguments struct for more...