extern crate csv;
use std::env;
use std::error::Error;
use std::process;
use std::ffi::OsString;

fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;    
    let mut rdr = csv::Reader::from_path(file_path)?;
    {
        let headers = rdr.headers()?;
        println!("{:?}",headers);
    }
    let mut rowcount = 0;
    for result in rdr.records() {
        // let record = result?;
        // println!("{:?}", record);
        rowcount+=1;
    }

    println!("number of rows: {}",rowcount);

    Ok(())
}

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}


fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }

    
   
}

