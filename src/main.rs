use std::env;
use std::fs;

fn preprocessing(contents:String) {
    let lines:Vec<&str> = contents.split("\n").collect();
    let mut urls = Vec::new();

    for line in lines {
        if line.len() > 0 {
            let items:Vec<&str> = line.split_whitespace().collect();
            if items[4] == "query[A]" {
                if !urls.contains(&items[5]) {
                    urls.push(items[5]);
                }
            }
        }
    }
    println!("{:?}",urls);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    preprocessing(contents);
}