# zigarg

[![Build status](https://travis-ci.com/aldrinzigmundv/zigarg.svg?branch=master)](https://app.travis-ci.com/github/aldrinzigmundv/zigarg) [![](https://img.shields.io/crates/v/zigarg.svg)](https://crates.io/crates/zigarg) [![](https://docs.rs/zigarg/badge.svg)](https://docs.rs/zigarg)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

zigarg is a very light argument parser written fully in Rust. It's composed only of a small number but very helpful lines and lacks a lot of features but suitable for many applications, especially personal or small projects. It's also good for educational purposes as it isn't very complex.

I decided to publish the library after using it on several private CLI applications I made. I may add additional features in the future if I find it useful for my own projects as well.

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
//Measure the number of arguments provided the user
let arguments_number = arguments.len();
//Get an argument from the arguments provided by the user via index number
let argument = arguments.get(1);
```
Check documentation of the Arguments struct for more...