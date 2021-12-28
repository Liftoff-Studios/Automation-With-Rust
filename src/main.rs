use chrono::prelude::*;
use std::process;


fn main() {
    open_browser();
}

fn time_in_range()->bool{

    let time = 7;

    // This is the code for local time
    let local: String = Local::now().to_string();

    let local1: Vec<&str> = local.split(" ").collect();
    let local2: Vec<&str> = local1[1].split(":").collect();

    let time_local: i32 = local2[0].parse().expect("Error parsing");


    // Checks if difference is 1 or -1 and decides accordingly
    if time-time_local == 1 || time-time_local==(-1) ||time-time_local==0
        {
            return true;
        }

    false
}

fn open_browser(){
    let time_checker: bool = time_in_range();
    if time_checker{
        let res = open::that("https://entrar.in/login/login");

        if let Err(e) = res{
            println!("Error opening browser: {}",e);
        }

        process::exit(1);
    }

    process::exit(1)
}
