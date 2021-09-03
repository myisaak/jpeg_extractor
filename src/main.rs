use jpeg_extractor::extract_images;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: jpeg_extractor <binary_file_containing_jpegs> [--verbose]");
        return;
    }
    let file = &args[1];
    let verbose = args.iter().find(|&arg| arg == "--verbose").is_some();
    extract_images(file, verbose);
}

#[test]
fn test1() {
    let path = "./test_images/1-dummy.jpg";
    extract_images(path, false);
    let img = std::fs::read(path).unwrap();
    let extracted = std::fs::read("img-0.jpg").unwrap();
    assert_eq!(img, extracted);
}
