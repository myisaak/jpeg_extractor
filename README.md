# jpeg_extractor

> Extract JPEGs out of binary files from the command-line

## Install

```bash
cargo install jpeg_extractor
```

## Usage

```bash
Usage: jpeg_extractor <binary_file_containing_jpegs> [--verbose]
```

## How it works?

It uses signatures common to all JPEG types. A start signature `0xFF, 0xD8, 0xFF, 0xE0` and an end signature `0xFF, 0xD9`.

## Limitations

Currently only supports JPEGs with EXIF data.