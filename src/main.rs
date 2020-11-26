use std::env;

fn main() {
    let command:Vec<String> = env::args().collect();

    let exp:&String = &command[1];
    let file_name : &String = &command[2];

    println!("Searching {} in File {}",exp,file_name);

}
