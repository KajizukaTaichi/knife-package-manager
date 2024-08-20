#![allow(warnings)]
use colored::*;
use dirs::home_dir;
use git2::Repository;
use std::env;
use std::fs;
use std::io;
mod gitl;
mod info;
mod install;

fn main() {
    let version = info::VERSION;
    let _args: Vec<String> = env::args().collect();
    if _args.len() > 1 {
        if _args[1] == "update" {
            gitl::Gitl::update_package_list();
            std::process::exit(0);
        } else if _args[1] == "upgrade" {
            gitl::Gitl::upgrade_knife(version);
        } else if _args[1] == "install" && _args.len() == 3 {
            install::Get::search_program(_args[2].to_string());
        } else if _args[1] == "install" {
            eprintln!("{} Specify the package to install", "Error:".red(),);
        }
    } else {
        println!(
            "{}{}",
            "Error: option needed.".red(),
            " Please run --help option and check your options"
        );
        std::process::exit(1);
    }
}
