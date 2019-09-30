extern crate serde;
extern crate serde_yaml;
extern crate serde_derive;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read};

extern crate chrono;
use chrono::{Duration, NaiveTime};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct DurationEntry {
    title: String,
    date: String,
    duration: i64,
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

    let program: DurationProgram = match serde_yaml::from_str(&s) {
        Ok(program) => program,
        Err(_)      => panic!("could not parse yaml"),
    };

    println!("{:?}", program);

    let mut prev_time: NaiveTime = match NaiveTime::parse_from_str(&program.program_start_time, "%H:%M") {
        Ok(prev_time) => prev_time,
        Err(_)        => panic!("could not parse time"),
    };

    println!("{}", prev_time);

    for entry in program.program {
        println!("title: {}", entry.title);
        println!("start_time: {}", prev_time.format("%H:%M"));
        prev_time = prev_time + Duration::minutes(entry.duration);
        println!("end_time: {}", prev_time.format("%H:%M"));
    }
}
