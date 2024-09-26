use std::env;
use std::fs;
use std::io;

fn main() {

	let arguments: Vec<String> = env::args().collect(); //collect command line argument
	let size = &arguments[1]; //pick only the second arguments since the first is the program name
	let size: u64 = size.trim().parse().expect("please type a number!"); // convert to unsigned integer
	println!("Creating file of size {size}"); // output picked file size
	create_file{size}; //create file function call
	
} 

fn create_file (size: u64) -> io::Result<()> { // create_file function definition, takes in size as a unsigne integer, result type of Result

	fs::File::create("{size}.dat")?.set_len(size)?; //create file of name {size}.dat and fill it with zeroes to the correct size
	Ok(())	
							
}
	

