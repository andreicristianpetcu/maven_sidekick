#!/usr/bin/env run-cargo-script
// cargo-deps: time="0.1.25"
// You can also leave off the version number, in which case, it's assumed
// to be "*".  Also, the `cargo-deps` comment *must* be a single-line
// comment, and it *must* be the first thing in the file, after the
// hashbang.
extern crate time;

pub fn get_project_artifact_id(file_path: &str) -> String {
    print!("{0}", file_path.to_string());
    String::from("test")
}

fn main() {
    println!("{}", time::now().rfc822z());
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