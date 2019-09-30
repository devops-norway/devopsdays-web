extern crate serde;
extern crate serde_yaml;
extern crate serde_derive;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read};

// extern crate chrono;
// use chrono::{Duration, Utc};

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
    // let now = Utc::now();
    // println!("{}", now);
    // let thirty_minutes_from_now = now + Duration::minutes(30);
    // println!("{}", thirty_minutes_from_now);

    let mut f = File::open("something.yaml").expect("Unable to open file");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("Unable to read file");

    let _deserialized_program: Result<DurationProgram, _> = serde_yaml::from_str(&s);
    println!("{:?}", _deserialized_program);

    // // // Index access for map & array
    // // assert_eq!(doc["foo"][0].as_str().unwrap(), "list1");
    // // assert_eq!(doc["bar"][1].as_f64().unwrap(), 2.0);

    // // // Chained key/array access is checked and won't panic,
    // // // return BadValue if they are not exist.
    // // assert!(doc["INVALID_KEY"][100].is_badvalue());
}
