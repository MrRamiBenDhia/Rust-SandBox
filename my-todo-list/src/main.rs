extern crate fancy;

use fancy::printcoln;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::{self, BufRead};



#[derive(Debug)]
enum Operation{
    Add, Done
}

fn main() {
    printcoln!("[s|cyan]Hello world [magenta]!");
    
    let mut operation = Operation::Add;

    let list_todo = read_from_cmd();

    println!("Operation= {:?}",operation);

    add_to_file(&list_todo);
    let loaded_list_todo = read_from_file();

    // println!("Loaded list : {:?}",loaded_list_todo);
    print_list(&loaded_list_todo);
    println!("Length list : {}", loaded_list_todo.len());
}

// fn write_to_file() {
//     let mut file = File::create("Data.txt").expect("Opps");
//     file.write_all("Hello there \n".as_bytes())
//         .expect("Fama mochkla");
// }
fn read_from_file() -> Vec<String> {
    let file = File::open("data.txt").expect("Failed to open file");
    let reader = io::BufReader::new(file);

    let mut lines = Vec::new();

    for line in reader.lines() {
        lines.push(line.expect("Failed to read line"));
    }

    lines
}
fn add_to_file(list_todo: &Vec<String>) {
    let mut file = OpenOptions::new()
        .append(true)
        .open("data.txt")
        .expect("cannot open file");

    for item in list_todo {
        file.write_all((item.to_owned() + "\n").as_bytes())
            .expect("write failed");
    }

    println!("file append success");
}

fn print_list(list_todo: &Vec<String>) {
    println!("Here is the complete list : {:?}", list_todo);
}

fn read_from_cmd() -> Vec<String> {

  
    let cmd_line = std::env::args();
    println!("Num of elements to add in todo is :{}", cmd_line.len() - 1);

    let mut list_todo = Vec::<String>::new();
    let mut has_read_first_arg = false;

    for arg in cmd_line {
        if has_read_first_arg {
            //skip the first argument since it is the exe file name
            list_todo.push(arg)
        }
        has_read_first_arg = true;
    }
    println!("New Items {:?}", list_todo);
    return list_todo;
}
