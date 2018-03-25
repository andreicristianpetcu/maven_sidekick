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
    artifact_id: String,
    group_id: String
}

pub fn get_project(file_path: &str) -> MavenProject {
    print!("{0}", file_path.to_string());

    let file = File::open(file_path).unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    let mut depth = 0;
    let mut artifact_id :String = "".to_string();
    let mut group_id :String = "".to_string();
    let mut current_tag :String = "".to_string();
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                current_tag = name.local_name.to_string();
                depth += 1;
            }
            Ok(XmlEvent::EndElement { .. }) => {
                depth -= 1;
            }
            Ok(XmlEvent::Characters(data)) => {
                if depth == 2 && "artifactId".eq_ignore_ascii_case(&current_tag) {
                    artifact_id = data.to_string()
                }
                if depth == 2 && "groupId".eq_ignore_ascii_case(&current_tag) {
                    group_id = data.to_string()
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
    MavenProject { artifact_id, group_id }
}

fn main() {
    let project = get_project("test_data/pom.xml");
    println!("group id is {} and artifact id is {}", project.artifact_id, project.group_id);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_gets_project_artifact_id() {
        let project = get_project("test_data/pom.xml");
        assert_eq!("camel-core", project.artifact_id);
    }

    #[test]
    fn it_gets_project_group_id() {
        let project = get_project("test_data/pom.xml");
        assert_eq!("org.apache.camel", project.group_id);
    }

}