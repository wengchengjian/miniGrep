use std::{fs,error::Error};



  pub fn run(filename:&str)->Result<(),Box<dyn Error>>{
    let contents = fs::read_to_string(filename)?;
  
    println!("With text:\n{}",contents);
  
    Ok(())
  }
  
  pub struct Config {
    pub query:String,

    pub filename:String,
  }
  
  impl Config {
    pub fn new(args:&[String])-> Result<Config,&'static str>{
  
      if args.len() < 3{
        return Err("not enough arguments.At lease 2 arg");
      }
  
      let query = args[1].clone();
  
      let filename = args[2].clone();
      return Ok(Config{
        query,
        filename
      })
    }
  }



