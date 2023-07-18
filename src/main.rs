#![allow(unused)]

use clap::Parser;
use std::io::Write;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::Read;
use std::env;
use chrono::prelude::*;

#[derive(Parser)]
struct Jot {
    action: String,
    action_text: String
}

fn add(action_text: &String, mut current_file: File) -> () {

    // read if it has todays date
    let mut content = std::fs::read_to_string("today").expect("Unable to read current file");

    // add date if file is empty
    let mut date = Local::now().date_naive();
    if content.is_empty() {
        current_file.write_all(&mut date.format("%A, %-d %B, %C%y \n").to_string().as_bytes()).unwrap()
    };

    // insert task
    let entry = format!("{}\n",action_text);
    match current_file.write_all(entry.as_bytes()) {
        Err(why) => panic!("couldn't write {}" ,why),
        Ok(_) => println!("task successfully written"),
    };
}


fn main() {
    let args = Jot::parse();


    let master_file = OpenOptions::new().create(true).write(true).open("master");
    let mut current_file = match OpenOptions::new().create(true).append(true).open("today") {
        Ok(file) => file,
        Err(e) => panic!("Problem opening the data file: {:?}", e),
    };
    
    if &args.action == "add" {
        add(&args.action_text, current_file)
    }
    // either unwrap or handle the error
    // let mut current_file = OpenOptions::new().create(true).append(true).open("today").unwrap();
  
    println!("action {}, action_text {}", &args.action, &args.action_text)

}
