extern crate fancy;

use fancy::printcoln;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Operation {
    Add,
    Done,
}

fn main() {
    printcoln!("[s|cyan]Hello world [magenta]!");

    let mut operation = Operation::Add;

    let list_todo = read_from_cmd(&mut operation);
    println!("Operation after = {:?}", operation);

    add_to_file(&list_todo, &operation);
    let loaded_list_todo = read_from_file();

    // println!("Loaded list : {:?}",loaded_list_todo);
    print_list(&loaded_list_todo);
    println!("Length list : {}", loaded_list_todo.len());
}

fn read_from_file() -> Vec<String> {
    let file = File::open("data.txt").expect("Failed to open file");
    let reader = io::BufReader::new(file);

    let mut lines = Vec::new();

    for line in reader.lines() {
        lines.push(line.expect("Failed to read line"));
    }

    lines
}

fn add_to_file(list_todo: &Vec<String>, operation: &Operation) {
    let mut file = OpenOptions::new()
        .append(true)
        .open("data.txt")
        .expect("cannot open file");

    for item in list_todo {
        match operation {
            Operation::Done => {
                file.write_all(("#".to_owned() + item + "\n").as_bytes())
                    .expect("write failed");
            }
            _ => {
                file.write_all((item.to_owned() + "\n").as_bytes())
                    .expect("write failed");
            }
        }
    }

    println!("file append success");
}

fn print_list(list_todo: &Vec<String>) {
    println!("Here is the complete list : {:?}", list_todo);
}

fn read_from_cmd(mut operation: &mut Operation) -> Vec<String> {
    println!("Operation before = {:?}", operation);

    let cmd_line = std::env::args();
    println!("Num of elements to add in todo is : {}", cmd_line.len() - 1);

    let mut list_todo = Vec::<String>::new();
    let mut has_read_first_arg = false;
    let mut has_read_second_arg = false;

    for arg in cmd_line {
        if has_read_first_arg {
            // skip the first argument since it is the exe file name
            if has_read_second_arg {
                list_todo.push(arg);
            } else {
                println!("here is the second thing: {}", arg);
                match arg.as_str() {
                    "add" => *operation = Operation::Add,
                    "done" => *operation = Operation::Done,
                    _ => println!("wake up"),
                }
            }
            has_read_second_arg = true;
        }
        has_read_first_arg = true;
    }
    println!("New Items {:?}", list_todo);
    list_todo
}
