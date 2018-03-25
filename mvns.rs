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

pub struct MavenProject {
    artifact_id: String
}

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size).map(|_| INDENT)
             .fold(String::with_capacity(size*INDENT.len()), |r, s| r + s)
}

pub fn get_project_artifact_id(file_path: &str) -> MavenProject {
    print!("{0}", file_path.to_string());

    let file = File::open(file_path).unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    let mut depth = 0;
    // let mut artifact_id :String;
    let mut current_tag :String = "".to_string();
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                println!("{}+{}", indent(depth), name);
                current_tag = name.local_name.to_string();
                depth += 1;
            }
            Ok(XmlEvent::EndElement { .. }) => {
                depth -= 1;
            }
            Ok(XmlEvent::CData(data)) => {
                println!("{}={}", indent(depth), data );
            }
            Ok(XmlEvent::Characters(data)) => {
                println!("{}={}", indent(depth), data);
                if depth == 2 && "artifactId".eq_ignore_ascii_case(&current_tag) {
                    return MavenProject { artifact_id: data }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
    MavenProject { artifact_id: String::from("test") }
}

fn main() {
    println!("{}", get_project_artifact_id("test_data/pom.xml").artifact_id);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_gets_project_artifact_id() {
        assert_eq!("camel-core", get_project_artifact_id("test_data/pom.xml").artifact_id);
    }

}