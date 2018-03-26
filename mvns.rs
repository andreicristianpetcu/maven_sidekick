#!/usr/bin/env run-cargo-script
// cargo-deps: time="0.1.25", xml-rs = "0.7", xml5ever = "0.1.3", tendril = "0.1.3"
// You can also leave off the version number, in which case, it's assumed
// to be "*".  Also, the `cargo-deps` comment *must* be a single-line
// comment, and it *must* be the first thing in the file, after the
// hashbang.
extern crate time;

extern crate xml;

use std::fs::File;
use std::io::BufReader;

use xml::reader::{EventReader, XmlEvent};

#[allow(dead_code)]
struct MavenProject {
    artifact_id: String,
    group_id: String,
    parent_group_id: Option<String>,
    dependencies: Vec<MavenProject>,
}

impl MavenProject {
    fn new(artifact_id: &str, group_id: &str) -> Self {
        MavenProject {
            artifact_id: artifact_id.to_string(),
            group_id: group_id.to_string(),
            parent_group_id: None::<String>,
            dependencies: Vec::new(),
        }
    }
}

fn get_project(file_path: &str) -> MavenProject {
    print!("{0}", file_path.to_string());

    let file = File::open(file_path).unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    let mut artifact_id: String = "".to_string();
    let mut group_id: String = "".to_string();
    let mut dependency_artifact_id: String = "".to_string();
    let mut dependency_group_id: String = "".to_string();
    let mut parent_group_id: Option<String> = Option::None;
    let mut dependencies: Vec<MavenProject> = Vec::new();
    let mut tag_hierarchy: Vec<String> = Vec::new();
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                tag_hierarchy.push(name.local_name.to_string());
            }
            Ok(XmlEvent::EndElement { .. }) => {
                let hierarchy_as_str = to_string(&tag_hierarchy);
                if "/project/dependencies/dependency".eq_ignore_ascii_case(&hierarchy_as_str) {
                    dependencies.push(MavenProject::new(
                        &dependency_artifact_id,
                        &dependency_group_id,
                    ))
                }
                tag_hierarchy.pop();
            }
            Ok(XmlEvent::Characters(data)) => match to_string(&tag_hierarchy).as_ref() {
                "/project/artifactId" => artifact_id = data.to_string(),
                "/project/groupId" => group_id = data.to_string(),
                "/project/parent/groupId" => parent_group_id = Option::from(data.to_string()),
                "/project/dependencies/dependency/artifactId" => {
                    dependency_artifact_id = data.to_string()
                }
                "/project/dependencies/dependency/groupId" => {
                    dependency_group_id = data.to_string()
                }
                _ => {}
            },
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
    MavenProject {
        artifact_id,
        group_id,
        parent_group_id,
        dependencies,
    }
}

fn to_string(vector: &[String]) -> String {
    let mut result: String = "".to_string();
    vector.into_iter().for_each(|item| {
        result.push_str("/");
        result.push_str(item);
    });
    result
}

fn main() {
    let project = get_project("test_data/pom.xml");
    println!(
        "group id is {} and artifact id is {} and the parent group id is {}",
        project.artifact_id,
        project.group_id,
        project
            .parent_group_id
            .unwrap_or_else(|| "none".to_string())
    );
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

    #[test]
    fn it_gets_parent_project_group_id() {
        let project = get_project("test_data/pom.xml");
        assert_eq!("org.apache.camel", project.parent_group_id.unwrap());
    }

    #[test]
    fn it_gets_dependencies_from_dependency_section() {
        let project = get_project("test_data/pom.xml");
        let first_dependency = project.dependencies.get(0).unwrap();

        assert!(project.dependencies.len() >= 15);
        assert_eq!("spi-annotations", first_dependency.artifact_id);
        assert_eq!("org.apache.camel", first_dependency.group_id);
    }

    #[test]
    #[ignore]
    fn it_gets_dependencies_from_plugin_section() {
        let project = get_project("test_data/pom.xml");

        assert_eq!(23, project.dependencies.len());
    }

    #[test]
    fn to_string_prints_xml_path() {
        let vec = &vec!["project".to_string(), "artifactId".to_string()];
        let vec_to_string = to_string(vec);
        assert_eq!("/project/artifactId", vec_to_string);
    }

    // #[test]
    // #[ignore]
    // fn it_gets_all_pom_files_from_cwd() {
    //     let pom_files = get_all_pom_files_from_cwd();
    //     assert_eq!(1, pom_files.len());
    //     assert!(pom_files.item(0).ends_with("test_data/pom.xml"));
    // }

    // #[test]
    // #[ignore]
    // fn it_gets_pom_file_from_artifact() {
    //     let pom_file = get_pom_file_from_artifact("org.apache.camel:camel-core");
    //     assert!(pom_file.ends_with("test_data/pom.xml"));
    // }

}
