// https://doc.rust-lang.org/1.16.0/book/benchmark-tests.html
#![feature(test)]

mod wrappers;
use wrappers::MyNode;

use serde::Deserialize;
use serde_json::json;
use yaml_front_matter::YamlFrontMatter;

// Debug allows the struct to be printed
#[derive(Deserialize, Debug)]
struct Metadata {
    page_title: String,
    description: String,
}

fn main() -> Result<(), String> {
    use std::env;
    use std::fs;

    let working_dir = env::current_dir().unwrap();
    let file_path = working_dir.join("src/input.mdx");

    // load string contents of MDX
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // split contents into YAML frontmatter and actual content
    let result = YamlFrontMatter::parse::<Metadata>(contents.as_str()).unwrap();

    println!("{:?}", result.metadata);

    // parse into and AST and do some json serialization
    let mdast = markdown::to_mdast(&result.content, &markdown::ParseOptions::default())?;
    let node: MyNode = mdast.into();

    let obj = json!(node);

    let output_file_path = working_dir.join("src/output.json");
    let output_json = serde_json::to_string_pretty(&obj).unwrap();

    // Output to local file
    fs::write(output_file_path, output_json.clone()).unwrap();

    // Output to stdout
    println!("{}", output_json.clone());

    Ok(())
}

// BENCHMARKS
#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use test::Bencher;
    #[bench]
    fn benchmark_test(b: &mut Bencher) {
        b.iter(|| main());
    }
}
