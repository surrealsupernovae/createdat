//! Creates .dat files of different sizes based on arguments given in the command line
//! The first argument is the number of a certain unit that you would like, it is supposed to be a unsigned integer.
//! The second argument corresponds to the unit, there are only three right now: KB, MB and GB.
//! The file will be named "size_unit.dat"
//! # Example
//! ```
//! cargo run -- 20 KB
//! ```

use std::env;
use std::fs;

fn main() {

	let arguments: Vec<String> = env::args().collect(); //collect command line argument
	let size = &arguments[1]; //pick only the second arguments since the first is the program name
	let bytes: &String = &arguments[2].clone();
	let bytes = &bytes[..];
	let name: &String = &arguments[1].clone(); // name for the size	let name_mut = name.clone();
	let size: u64 = size.trim().parse().expect("please type a  u64 number!"); // convert to unsigned integer
	println!("Creating file of size {size} {bytes}"); // output picked 
	size_picker(size, name, bytes); 
	
} 


fn size_picker (mut size: u64, name: & String, unit: &str) {
	
	match unit {

		"KB" => { 
			size *= 1024;
			create_file(size, name, unit)
		},
		"MB" => {
			size *= 1048576;
			create_file(size, name, unit)
		},
		"GB" => {
			size *= 1073741824;
			create_file(size, name, unit)
		},
		_ => println!("Type a correct unit : KB, MB, GB"),
    }
}

fn create_file (size: u64, name :& String, unit: &str) { // create_file function definition, takes in size as a unsigne integer, result type of Result
	let x;
	x = name.clone();
	let f = fs::File::create(namer(unit, x)).expect("file creation issue");
	f.set_len(size).expect("Issue with filling up the file"); //create file of name {size}.dat and fill it with zeroes to the correct size
						
}
	
fn namer <'a> (unit: &'a str, mut x: String) -> String {
	x.push_str("_");
	x.push_str(unit);
	x.push_str(".dat");
	x 
}