use std::{env,fs,process,error::Error};

mod minigrep;

use minigrep::{Config,run};

fn main() {
    let args:Vec<String> = env::args().collect();
    
    let config = Config::new(&args).unwrap_or_else(|err|{
      println!("Problem parsing arguments : {}",err);
      process::exit(1);
    });

    let query = &config.query;

    let filename = &config.filename;

    println!("Searching for {}",query);

    println!("In file {}",filename);

    if let Err(e) = run(filename) {
      println!("Application error: {}",e);
      process::exit(1);
    }


}
