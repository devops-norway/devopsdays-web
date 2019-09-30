extern crate yaml_rust;
use yaml_rust::{YamlLoader, YamlEmitter};
use std::{fs::File, io::Read};
// extern crate serde_yaml;
// extern crate chrono;
// use chrono::{Duration, Utc};

fn main()  {
    // let now = Utc::now();
    // println!("{}", now);
    // let thirty_minutes_from_now = now + Duration::minutes(30);
    // println!("{}", thirty_minutes_from_now);

    // let f = std::fs::File::open("something.yaml")?;
    // let d: String = serde_yaml::from_reader(f)?;
    // println!("Read YAML string: {}", d);
    // Ok(())

    let mut f = File::open("something.yaml").expect("Unable to open file");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("Unable to read file"); 

    let docs = YamlLoader::load_from_str(&s).unwrap();

    // Multi document support, doc is a yaml::Yaml
    let doc = &docs[0];

    // Debug support
    println!("{:?}", doc);

    // // Index access for map & array
    // assert_eq!(doc["foo"][0].as_str().unwrap(), "list1");
    // assert_eq!(doc["bar"][1].as_f64().unwrap(), 2.0);

    // // Chained key/array access is checked and won't panic,
    // // return BadValue if they are not exist.
    // assert!(doc["INVALID_KEY"][100].is_badvalue());

    // Dump the YAML object
    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(doc).unwrap(); // dump the YAML object to a String
    }
    println!("{}", out_str);
}


