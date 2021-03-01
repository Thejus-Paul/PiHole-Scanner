use std::fs;
use std::collections::HashSet;
use regex::Regex;

const PIHOLE_LOG:&str = "./src/today_list.txt";
const EXACT_WHITELIST:&str = "./src/exact_whitelist.txt";
const PATTERN_BLACKLIST:&str = "./src/pattern_blacklist.txt";

fn read_file_contents(filename:&str) -> String {
	let contents = fs::read_to_string(filename)
		.expect("Something went wrong reading the file");
	return contents;
}

fn preprocessing() {
	let contents = read_file_contents(EXACT_WHITELIST);
	let mut exact_whitelist_urls = HashSet::new();
	for line in contents.split("\n") {
		exact_whitelist_urls.insert(line);
	}
	let pattern_blacklist = read_file_contents(PATTERN_BLACKLIST);

	let mut urls = HashSet::new();

	let contents = read_file_contents(PIHOLE_LOG);
	let lines:Vec<&str> = contents.split("\n").collect();
	for line in lines {
		let mut pattern_match:bool = false;
		if line.len() > 0 {
			let items:Vec<&str> = line.split_whitespace().collect();
			if items[4] == "query[A]" && !exact_whitelist_urls.contains(&items[5]) {
				for pattern in pattern_blacklist.split("\n") {
					let re = Regex::new(pattern).unwrap();
					if re.is_match(items[5]) {
						pattern_match = true;
						break;
					}
				}
				if !pattern_match {
					urls.insert(items[5]);
				}
			}
		}
	}
	println!("{:?}",urls);
}

fn main() {
	preprocessing();
}