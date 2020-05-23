extern crate csv;
use std::io;
use std::process;

fn main() {
    let mut rdr = csv::Reader::from_reader(io::stdin());

    //loop over each record.
    for result in rdr.records() {
        // An error may occur, so abort the program in an unfriendly way.
        // We will make this more friendly later!
        match result {
            Ok(record) => println!("{:?}", record),
            Err(err) => {
                println!("error reading csv from <stdin>: {}", err);
                process::exit(1);
            }
        }
        
    }
}
