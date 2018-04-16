#!/usr/bin/env run-cargo-script
// cargo-deps: xml-rs = "0.7", xml5ever = "0.1.3", tendril = "0.1.3", walkdir = "2", clap = "~2.31"

extern crate clap;
extern crate walkdir;
extern crate xml;

use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};
use walkdir::WalkDir;
use std::env;
use clap::{App, Arg};

struct MavenProject {
    artifact_id: String,
    group_id: String,
    #[allow(dead_code)]
    parent_group_id: Option<String>,
    version: Option<String>,
    dependencies: Vec<MavenProject>,
}

impl MavenProject {
    fn new(artifact_id: String, group_id: String) -> Self {
        MavenProject {
            artifact_id,
            group_id,
            parent_group_id: None::<String>,
            dependencies: Vec::new(),
            version: None::<String>,
        }
    }

    fn is_same_project_version(&self) -> bool {
        if let Some(ref version_value) = self.version {
            String::from("${project.version}").eq(version_value)
        } else {
            false
        }
    }

    #[allow(dead_code)]
    fn get_parent_id(&self) -> String {
        if !self.group_id.is_empty() {
            return self.group_id.to_string()
        } else if let Some(ref parent_group_id) = self.parent_group_id {
            return parent_group_id.to_string()
        }
        String::new()
    }
}

fn get_project(file_path: &str) -> MavenProject {
    let file = File::open(file_path).unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    let mut artifact_id: String = String::new();
    let mut group_id: String = String::new();
    let mut dependency_artifact_id: String = String::new();
    let mut dependency_group_id: String = String::new();
    let mut parent_group_id: Option<String> = Option::None;
    let mut dependencies: Vec<MavenProject> = Vec::new();
    let mut tag_hierarchy: Vec<String> = Vec::new();
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                tag_hierarchy.push(name.local_name);
            }
            Ok(XmlEvent::EndElement { .. }) => {
                let hierarchy_as_str = to_string(&tag_hierarchy);
                if "/project/dependencies/dependency".eq_ignore_ascii_case(&hierarchy_as_str) {
                    dependencies.push(MavenProject::new(
                        dependency_artifact_id.clone(),
                        dependency_group_id.clone(),
                    ))
                }
                tag_hierarchy.pop();
            }
            Ok(XmlEvent::Characters(data)) => match to_string(&tag_hierarchy).as_ref() {
                "/project/artifactId" => artifact_id = data,
                "/project/groupId" => group_id = data,
                "/project/parent/groupId" => parent_group_id = Option::from(data),
                "/project/dependencies/dependency/artifactId" => dependency_artifact_id = data,
                "/project/dependencies/dependency/groupId" => dependency_group_id = data,
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
        version: None::<String>,
    }
}

fn to_string(vector: &[String]) -> String {
    let mut result: String = String::new();
    vector.into_iter().for_each(|item| {
        result.push_str("/");
        result.push_str(item);
    });
    result
}

fn get_all_pom_files_from_cwd() -> Vec<String> {
    let mut pom_files: Vec<String> = Vec::new();
    let cwd = env::current_dir().unwrap();
    for entry in WalkDir::new(cwd).into_iter().filter_map(|e| e.ok()) {
        let entry_path: &Path = entry.path();
        let path = entry_path.to_str().unwrap();
        if entry_path.is_file() && path.ends_with("/pom.xml") {
            pom_files.push(path.to_string());
        }
    }
    pom_files.sort();
    pom_files
}

fn get_pom_file_from_artifact(project_to_find: &str) -> Result<String, String> {
    let pom_files = get_all_pom_files_from_cwd();
    for pom_file in &pom_files {
        let project = get_project(&pom_file);
        let project_full_name = format!("{}:{}", project.group_id, &project.artifact_id);
        if project_full_name.eq(&project_to_find) {
            return Ok(pom_file.to_string());
        }
    }
    Err(format!("Project not found {} in files {} \n", project_to_find, &pom_files.join("\n")))
}

#[allow(dead_code)]
fn gets_nested_dependencies(project: &MavenProject) -> String {
    let mut dependencies: String = String::new();
    dependencies.push_str(project.group_id.as_str());
    dependencies.push_str(":");
    dependencies.push_str(project.artifact_id.as_str());
    for dependency in &project.dependencies {
        if dependency.is_same_project_version() {
            dependencies.push_str(",");
            dependencies.push_str(&gets_nested_dependencies(dependency));
        }
    }

    dependencies
}

fn main() {
    let matches = App::new("Maven Sidekick")
        .version("v1.0-beta")
        .author("Andrei Petcu <andrei@ceata.org>")
        .about("This tool helps you with using maven on large projects")
        .arg(
            Arg::with_name("project")
                .short("p")
                .long("project")
                .value_name("org.apache.camel:camel-core")
                .help("A project name as an groupId:artifactId")
                .takes_value(true),
        )
        .get_matches();

    let project = matches
        .value_of("project")
        .unwrap_or("org.apache.camel:camel-core");

    let project_list = get_pom_file_from_artifact(project).unwrap();
    println!("{}", project_list);
}

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

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
    fn to_string_prints_xml_path() {
        let vec = &vec![String::from("project"), String::from("artifactId")];
        let vec_to_string = to_string(vec);
        assert_eq!("/project/artifactId", vec_to_string);
    }

    #[test]
    fn it_gets_all_pom_files_from_cwd() {
        let pom_files = get_all_pom_files_from_cwd();
        assert_eq!(2, pom_files.len());
        assert!(pom_files[0].ends_with("test_data/apache-camel/pom.xml"));
        assert!(pom_files[1].ends_with("test_data/pom.xml"));
    }

    #[test]
    fn it_gets_pom_file_from_artifact() {
        let pom_file = get_pom_file_from_artifact("org.apache.camel:camel-core").unwrap();
        assert!(pom_file.ends_with("test_data/pom.xml"));
    }

    #[test]
    fn it_gets_nested_dependencies() {
        let child1 = MavenProject {
            artifact_id: String::from("switzerland"),
            group_id: String::from("com.geography"),
            parent_group_id: None::<String>,
            dependencies: Vec::new(),
            version: Some(String::from("${project.version}")),
        };
        let parent = MavenProject {
            artifact_id: String::from("europe"),
            group_id: String::from("com.geography"),
            parent_group_id: None::<String>,
            dependencies: vec![child1],
            version: None::<String>,
        };

        let dependencies = gets_nested_dependencies(&parent);

        assert_eq!(
            "com.geography:europe,com.geography:switzerland",
            dependencies
        );
    }

    #[test]
    fn it_checks_same_project_version() {
        let project = build_project_with_version(Some(String::from("${project.version}")));
        assert_eq!(true, project.is_same_project_version());

        let project = build_project_with_version(Some(String::from("1.2.3")));
        assert_eq!(false, project.is_same_project_version());
    }

    #[test]
    fn it_gets_parent_id_from_group_id() {
        let only_group_id = MavenProject {
            artifact_id: String::new(),
            group_id: String::from("group_id"),
            parent_group_id: None::<String>,
            dependencies: Vec::new(),
            version: Some(String::new()),
        };

        assert_eq!("group_id", only_group_id.get_parent_id());
    }

    #[test]
    fn it_gets_parent_id_from_parent_group_id() {
        let only_parent_id = MavenProject {
            artifact_id: String::new(),
            group_id: String::new(),
            parent_group_id: Some(String::from("parent_group_id")),
            dependencies: Vec::new(),
            version: Some(String::new()),
        };

        assert_eq!("parent_group_id", only_parent_id.get_parent_id());
    }

    #[test]
    fn it_gets_parent_id_from_most_specific() {
        let group_and_parent_id = MavenProject {
            artifact_id: String::new(),
            group_id: String::from("group_id"),
            parent_group_id: Some(String::from("parent_group_id")),
            dependencies: Vec::new(),
            version: Some(String::new()),
        };

        assert_eq!("group_id", group_and_parent_id.get_parent_id());
    }

    #[allow(dead_code)]
    fn build_project_with_version(version: Option<String>) -> MavenProject {
        MavenProject {
            artifact_id: String::new(),
            group_id: String::new(),
            parent_group_id: None::<String>,
            dependencies: Vec::new(),
            version,
        }
    }
}
