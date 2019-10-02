extern crate serde;
extern crate serde_yaml;
extern crate serde_derive;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read};

extern crate chrono;
use chrono::{Duration, NaiveTime};

// use structopt::StructOpt;
// #[derive(StructOpt)]
// struct Cli {
//     pattern: String,
//     #[structopt(parse(from_os_str))]
//     path: std::path::PathBuf,
// }

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct DurationEntry {
    title: String,
    #[serde(rename = "type")]
    type_field: String,
    date: String,
    duration: i64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct DurationProgram {
    program_start_time: String,
    program: Vec<DurationEntry>,
}


fn main()  {
    // let args = Cli::from_args();

    let mut f = File::open("day2.yml").expect("Unable to open file");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("Unable to read file");

    let program: DurationProgram = match serde_yaml::from_str(&s) {
        Ok(program) => program,
        Err(_)      => panic!("could not parse yaml"),
    };


    let mut prev_time: NaiveTime = match NaiveTime::parse_from_str(&program.program_start_time, "%H:%M") {
        Ok(prev_time) => prev_time,
        Err(_)        => panic!("could not parse time"),
    };

    println!("program:");

    for entry in program.program {
        println!("  - title: \"{}\"", entry.title);
        println!("    type: {}", entry.type_field);
        println!("    date: {}", entry.date);
        println!("    start_time: \"{}\"", prev_time.format("%H:%M"));
        prev_time = prev_time + Duration::minutes(entry.duration);
        println!("    end_time: \"{}\"", prev_time.format("%H:%M"));
    }
}
