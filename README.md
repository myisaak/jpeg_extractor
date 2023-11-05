[![Crates.io](https://img.shields.io/crates/v/jpeg_extractor.svg)](https://crates.io/crates/jpeg_extractor)
[![Rust](https://github.com/MyIsaak/jpeg_extractor/actions/workflows/rust.yml/badge.svg)](https://github.com/MyIsaak/jpeg_extractor/actions/workflows/rust.yml)

# jpeg_extractor

> Extract JPEGs out of binary files from the command-line

## Install

```bash
cargo install jpeg_extractor
```

## Usage

To use the tool, run the following command:

```bash
Usage: jpeg_extractor <binary_file_containing_jpegs> [--verbose]
```

### Options
- <binary_file_containing_jpegs>: Specify the binary file from which you want to extract JPEG images.
- --verbose (optional): Use this flag to enable verbose output.

## How it works?

**jpeg_extractor** identifies JPEG images within binary files by looking for specific start and end signatures that are common to all JPEG types. The start signature is **'0xFF, 0xD8, 0xFF, 0xE0'**, and the end signature is **'0xFF, 0xD9'**.

## Limitations

Currently **jpeg_extractor** only supports JPEGs with EXIF data.

## Contributing

If you would like to contribute to the development of jpeg_extractor, please check out [CONTRIBUTING.md](https://github.com/MyIsaak/jpeg_extractor/blob/main/CONTRIBUTING.md)

