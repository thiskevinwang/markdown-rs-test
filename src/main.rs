// https://doc.rust-lang.org/1.16.0/book/benchmark-tests.html
#![feature(test)]

mod wrappers;
use wrappers::MyNode;

use serde_json::json;

fn main() -> Result<(), String> {
    use std::env;
    use std::fs;

    // /Users/kevin/repos/markdown-rs-test
    let working_dir = env::current_dir().unwrap();
    // println!("working_dir: {:?}", working_dir);
    let file_path = working_dir.join("src/input.mdx");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mdast = markdown::to_mdast(&contents.to_string(), &markdown::ParseOptions::default())?;
    let node: MyNode = mdast.into();

    let obj = json!(node);
    // println!("{}", serde_json::to_string_pretty(&obj).unwrap());

    let output_file_path = working_dir.join("src/output.json");
    fs::write(
        output_file_path,
        serde_json::to_string_pretty(&obj).unwrap(),
    )
    .unwrap();

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
