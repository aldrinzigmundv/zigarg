
#![warn(missing_docs)]
#![forbid(unsafe_code)]

//! zigarg is a very light argument parser written fully in Rust. It's not dependent on any third party libraries other than those that Rust already comes with. It lacks many features, like help generation, but enough to be suitable for a lot of applications. It's also good for educational purposes as it isn't very complex.
//! 
//! I decided to publish the library after using it on several private CLI applications I made. I may add additional features in the future if I find it useful for my own projects as well.
//! 
//! # Quickstart
//! Add `zigarg` to `Cargo.toml` as a dependency
//! ```ignore
//! [dependencies]
//! zigarg = "1.1.0"
//! ```
//! Capture user's arguments by adding the code below, after you have added zigarg to your dependencies:
//! ```ignore
//! use zigarg::Arguments;
//! let arguments = Arguments::new();
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

use std::{env, ops::Deref, usize};

#[derive(Debug)]
///`Struct` that contains the arguments provided by the user
pub struct Arguments(pub Vec<String>);

impl Arguments {

    ///Returns an `Arguments` struct that contains the arguments provided by the user with zigarg's functionalities
    pub fn new() -> Arguments {
        let args = env::args().collect();
        Arguments(args)
    }
    
    ///Returns `true` if the user provided arguments other than the program name
    pub fn has_args(&self) -> bool {
        if &self.0.len() > &1 {
            true
        } else {
            false
        }
    }

    ///Returns the lenght through `usize` of the `Arguments` struct
    pub fn len(&self) -> usize {
        self.0.deref().len()
    }

    ///Returns `true` if the number provided is equals to the number `Arguments` struct has
    pub fn eq(&self, num: usize) -> bool {
        if &self.0.len() == &num {
            return true
        }
        false
    }

    ///Returns a `Some(&String)` from the Arguments struct based on the provided index or `None` if not found
    pub fn get(&self, index: usize) -> core::option::Option<&String> {
        self.0.get(index)
    }

    ///Returns value through `Some('&String)` of provided flag (not case sensitive) or `None` if not found
    pub fn get_value(&self, query: &str) -> core::option::Option<&String> {
        for (i, el) in self.0.iter().enumerate() {
            if query.eq_ignore_ascii_case(&el) {
                return self.0.get(i+1);
            }
        }
        return None;
    }

    ///Returns value through `Some(&String)` of provided flag (case sensitive) or `None` if not found
    pub fn get_value_case_sensitive(&self, query: &str) -> core::option::Option<&String> {
        for (i, el) in self.0.iter().enumerate() {
            if query.eq(&el[..]) {
                return self.0.get(i+1);
            }
        }
        return None;
    }

    ///Returns index through `Some(usize)` of the `&str` slice provided from the `Arguments` struct or `None` if not found (not case sensitive)
    pub fn get_index(&self, query: &str) -> core::option::Option<usize> {
        for (i, el) in self.0.iter().enumerate() {
            if query.eq_ignore_ascii_case(&el) {
                return Some(i);
            }
        }
        return None;
    }

    ///Returns index through `Some(usize)` of the `&str` slice provided from the `Arguments` struct or `None` if not found (case sensitive)
    pub fn get_index_case_sensitive(&self, query: &str) -> core::option::Option<usize> {
        for (i, el) in self.0.iter().enumerate() {
            if query.eq(&el[..]) {
                return Some(i);
            }
        }
        return None;
    }

    ///Returns `true` if provided `&str` slice is found on any of the arguments provided by the user in the `Arguments` struct (not case sensitive)
    pub fn exist(&self, query: &str) -> bool {
        for arg in &self.0 {
            if query.eq_ignore_ascii_case(&arg) {
                return true;
            }
        }
        false
    }

    ///Returns `true` if provided `&str` slice is found on any of the arguments provided by the user in the `Arguments` struct (case sensitive)
    pub fn exist_case_sensitive(&self, query: &str) -> bool {
        for arg in &self.0 {
            if query.eq(&arg[..]) {
                return true;
            }
        }
        false
    }

    ///Returns the rest of the arguments based on provided index
    pub fn get_after_index(&self, query: usize) -> Vec<String> {
        self.0[query..].iter().map(|x| x.to_owned()).collect()
    }
}
