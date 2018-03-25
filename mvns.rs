#!/usr/bin/env run-cargo-script
// cargo-deps: time="0.1.25"
// You can also leave off the version number, in which case, it's assumed
// to be "*".  Also, the `cargo-deps` comment *must* be a single-line
// comment, and it *must* be the first thing in the file, after the
// hashbang.
extern crate time;

extern crate xml;

use std::fs::File;
use std::io::BufReader;

use xml::reader::{EventReader, XmlEvent};

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size).map(|_| INDENT)
             .fold(String::with_capacity(size*INDENT.len()), |r, s| r + s)
}

pub fn get_project_artifact_id(file_path: &str) -> String {
    print!("{0}", file_path.to_string());

    let file = File::open(file_path).unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    let mut depth = 0;
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                println!("{}+{}", indent(depth), name);
                depth += 1;
            }
            Ok(XmlEvent::EndElement { name }) => {
                depth -= 1;
                println!("{}-{}", indent(depth), name);
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
    String::from("test")
}

fn main() {
    println!("{}", get_project_artifact_id("test_data/pom.xml"));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn it_gets_project_artifact_id() {
        assert_eq!("camel-core", get_project_artifact_id("test_data/pom.xml"));
    }

}