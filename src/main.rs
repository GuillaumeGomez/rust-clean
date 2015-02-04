/*
* rust-clean - Copyright (c) 2014 Gomez Guillaume.
*
* This software is provided 'as-is', without any express or implied warranty.
* In no event will the authors be held liable for any damages arising from
* the use of this software.
*
* Permission is granted to anyone to use this software for any purpose,
* including commercial applications, and to alter it and redistribute it
* freely, subject to the following restrictions:
*
* 1. The origin of this software must not be misrepresented; you must not claim
*    that you wrote the original software. If you use this software in a product,
*    an acknowledgment in the product documentation would be appreciated but is
*    not required.
*
* 2. Altered source versions must be plainly marked as such, and must not be
*    misrepresented as being the original software.
*
* 3. This notice may not be removed or altered from any source distribution.
*/

#![feature(io, core, path, os, collections)]

use std::old_io::fs;
use std::os;
use std::path::posix::Path;
use std::old_io::fs::PathExtensions;

struct CleanOptions {
    recursive: bool,
    verbose: bool,
    confirmation: bool
}

fn ask_confirmation(file: &Path) -> bool {
    loop {
        print!("clean: remove \x1b[37;1m'{}'\x1b[0m (y/n) ? ", file.as_str().unwrap());
        match std::old_io::stdio::stdin().read_line() {
            Ok(s) => {
                let tmp_s = s.replace("\r\n", "").replace("\n", "");

                if tmp_s.as_slice() == "y" || tmp_s.as_slice() == "yes" {
                    return true;
                } else if tmp_s.as_slice() == "n" || tmp_s.as_slice() == "no" {
                    return false;
                }
            }
            Err(_) => {}
        }
    }
}

fn start_clean(options: &CleanOptions, entry: &Path) {
    if entry.exists() {
        if entry.is_dir() {
            if options.recursive || entry.as_str().unwrap() == "." {
                match fs::readdir(entry) {
                    Ok(res) => {
                        if options.verbose {
                            println!("\x1b[36;1m-> Entering {}\x1b[0m", entry.as_str().unwrap());
                        }
                        for tmp in res.iter() {
                            start_clean(options, tmp);
                        }
                        if options.verbose {
                            println!("\x1b[34;1m<- Leaving {}\x1b[0m", entry.as_str().unwrap());
                        }
                    }
                    Err(e) => {
                        println!("\x1b[31;1mProblem with this directory: {} -> {}\x1b[0m", entry.as_str().unwrap(), e);
                    }
                }
            }
        } else {
            match entry.filename() {
                Some(s) => {
                    if s.last().unwrap() == &('~' as u8) {
                        if !options.confirmation || ask_confirmation(&Path::new(s)) {
                            match fs::unlink(entry) {
                                Ok(_) => {
                                    if options.verbose {
                                        println!("\x1b[32;1m{} deleted\x1b[0m", entry.as_str().unwrap());
                                    }
                                }
                                Err(e) => {
                                    println!("\x1b[31;1mProblem with this file: {} -> {}\x1b[0m", entry.as_str().unwrap(), e);
                                }
                            }
                        }
                    }
                }
                None => {
                    println!("\x1b[31;1mProblem with this file: {}\x1b[0m", entry.as_str().unwrap());
                }
            }
        }
    } else {
        println!("\x1b[31;1mProblem with this entry: {}\x1b[0m", entry.as_str().unwrap());
    }
}

fn print_help() {
    println!("./clean [options] [files | dirs]");
    println!("    -r : recursive mode");
    println!("    -v : verbose mode");
    println!("    -i : prompt before every removal");
    println!("--help : print this help");
}

fn main() {
    let mut args = os::args().clone();
    let mut options = CleanOptions{recursive: false, verbose: false, confirmation: false};
    let mut files = Vec::new();

    args.remove(0);
    for tmp in args.iter() {
        if tmp.clone().into_bytes()[0] == '-' as u8 {
            let mut tmp_arg = String::from_str(tmp.as_slice());

            tmp_arg.remove(0);
            if tmp_arg.len() > 0 {
                for character in tmp_arg.into_bytes().iter() {
                    match *character as char {
                        '-' => {
                            if tmp.as_slice() == "--help" {
                                print_help();
                                return;
                            }
                        }
                        'r' => {
                            options.recursive = true;
                        }
                        'v' => {
                            options.verbose = true;
                        }
                        'i' => {
                            options.confirmation = true;
                        }
                        _ => {
                            panic!("Unknown option: '{}', to have the options list, please launch with '-h' option\n", character);
                        }
                    }
                }
            } else {
                files.push(Path::new(tmp.as_slice()));
            }
        }
    }
    if files.len() == 0 {
        files.push(Path::new(String::from_str(".").as_slice()));
    }
    if options.verbose {
        println!("\x1b[33;1m=== VERBOSE MODE ===\x1b[0m");
    }
    for tmp in files.iter() {
        start_clean(&options, tmp);
    }
    if options.verbose {
        println!("\x1b[33;1mEnd of execution\x1b[0m");
    }
}