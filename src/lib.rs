
#![warn(missing_docs)]
#![forbid(unsafe_code)]

//! zigarg is a very light argument parser written fully in Rust. It's composed only of a small number but very helpful lines and lacks a lot of features but suitable for many applications, especially personal or small projects. It's also good for educational purposes as it isn't very complex.
//! 
//! I decided to publish the library after using it on several private CLI applications I made. I may add additional features in the future if I find it useful for my own projects as well.
//! 
//! # Quickstart
//! Add `zigarg` to `Cargo.toml` as a dependency
//! ```ignore
//! [dependencies]
//! zigarg = "0.1.0"
//! ```
//! Capture user's arguments by adding the code below, after you have added zigarg to your dependencies:
//! ```ignore
//! let arguments = zigarg::new();
//! ```
//! Use the struct returned from `zigarg::new()` to perform different actions like the examples below
//! ```ignore
//! //Check if there are arguments provided by the user other than your program's name
//! let has_arguments = arguments.has_args();
//! //Check if the arguments provided by the user has a certain flag
//! let exist = arguments.exist("-q");
//! //Measure the number of arguments provided the user
//! let arguments_number = arguments.len();
//! //Get an argument from the arguments provided by the user via index number
//! let argument = arguments.get(1);
//! ```
//! Check documentation of the Arguments struct below for more...

use std::{env, ops::Deref};

#[derive(Debug)]
///Struct that contains the arguments provided by the user
pub struct Arguments(pub Vec<String>);

impl Arguments {

    ///Returns an Arguments struct that contains the arguments provided by the user with zigarg's functionalities
    pub fn new() -> Arguments {
        let args = env::args().collect();
        Arguments(args)
    }
    
    ///Returns true if the user provided arguments other than the program name
    pub fn has_args(&self) -> bool {
        if &self.0.len() > &1 {
            true
        } else {
            false
        }
    }

    ///Returns the lenght of the Arguments struct
    pub fn len(&self) -> usize {
        self.0.deref().len()
    }

    ///Returns true if the number provided is equals to the number Arguments struct has
    pub fn eq(&self, num: usize) -> bool {
        if &self.0.len() == &num {
            return true
        }
        false
    }

    ///Returns an argument from Arguments struct based on the provided index
    pub fn get(&self, index: usize) -> &str {
        &self.0[index]
    }

    ///Returns true if provided string slice is found on any of the Arguments provided by the user (not case sensitive)
    pub fn exist(&self, query: &str) -> bool {
        for arg in &self.0 {
            if query.eq_ignore_ascii_case(&arg) {
                return true;
            }
        }
        false
    }

    ///Returns true if provided string slice is found on any of the Arguments provided by the user (not case sensitive)
    pub fn exist_case_sensitive(&self, query: &str) -> bool {
        for arg in &self.0 {
            if query.eq(&arg[..]) {
                return true;
            }
        }
        false
    }
}