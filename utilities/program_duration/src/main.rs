extern crate serde;
extern crate serde_yaml;
extern crate serde_derive;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read};

extern crate chrono;
// use chrono::{Duration};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct DurationEntry {
    title: String,
    date: String,
    duration: u8,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct DurationProgram {
    program_start_time: String,
    program: Vec<DurationEntry>,
}


fn main()  {
    let mut f = File::open("something.yaml").expect("Unable to open file");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("Unable to read file");

    let program: DurationProgram  = match serde_yaml::from_str(&s) {
        Ok(program) => program,
        Err(_) => panic!("could not parse yaml"),
    };

    println!("{:?}", program);


    let start_time = program.program_start_time;
    println!("{}", start_time);

    // let thirty_minutes_from_now = now + Duration::minutes(30);

}
