use std::{env, process::exit, ops::Index};
use rand::prelude::*;
use text_io;
use std::fs::File;
use std::io::{prelude::*, BufReader, LineWriter};


use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
   
    #[clap[short,long,value_parser]]
    path: String,


    #[clap[short,long,value_parser]]
    lenght: i32,

    #[clap[short,long,value_parser]]
    times: i32
}


fn main() {
    std::process::Command::new("clear").status();

    let arg = Args::parse();

    let x = ["1", "&2", "3", "4", "5", "6", "7", "8", "9", "10", "a", "b", "C", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "u", "q", "w" ,"v","p","x","y","z", "!", "@", "#" ,"$" ,"%" ,"^"  ,"&" ,"*" ,"(" ,")" ,"-" ,"[", "]" ];
    let mut rng = rand::thread_rng();



    let mut genPasswd = vec![String::new()];
    genPasswd.clear();

    for _ox in 0..arg.times {
    
    let mut generated = String::new(); 
    for _i in 0..arg.lenght{
        let _y = rng.gen_range(0..46);
        generated.push_str(x[_y]);
          
        }
        genPasswd.push(generated.to_string());
    }



 
    let mut f = File::create(arg.path).unwrap();
    let mut f = LineWriter::new(f);
      


    println!("Your Passowrds is:");
    for _w in genPasswd
    {
        println!("{:?}", _w);
        f.write(_w.as_bytes());  
        f.write(b"\n");
    
    }
    



    println!("Succeded to generate passwords");
    exit(0x1);

}

