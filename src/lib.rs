use std::convert::TryInto;
use std::fs;
use std::io::Seek;
use std::io::Write;
use std::io::{Read, SeekFrom};
use std::time::Instant;

const BUFFER_LEN: usize = 100000;

const FIRST: [u8; 4] = [0xFF, 0xD8, 0xFF, 0xE0];
const END: [u8; 2] = [0xFF, 0xD9];

pub fn extract_images(filename: &str, verbose: bool) {
    let mut src_f = fs::File::open(filename).expect("Something went wrong opening the file!");

    let mut search_strings_idx = 0;
    let search_strings_iter = &mut [FIRST.iter(), END.iter()];
    let mut search_strings_iter_cloned = search_strings_iter.clone();
    let mut search_strings = search_strings_iter_cloned.iter_mut();
    let mut search_str_bytes = search_strings.next();
    let mut search_byte = search_str_bytes.as_mut().unwrap().next();
    let buffer = &mut [0; BUFFER_LEN];
    let mut hits = 0;
    let mut start_addr = 0;
    let mut searching = false;
    let mut middle_searching = false;
    let mut reset_search_byte = false;
    let mut cur_addr: u64 = 0;
    let start_time = Instant::now();

    loop {
        let n = src_f.read(buffer).expect("Failed to read buffer from file");
        if n == 0 {
            break;
        }

        let mut cur_byte: u64 = 0;
        for &byte in buffer.iter() {
            if search_byte.is_none() {
                reset_search_byte = true;
            }

            if search_byte.is_some() && byte == *search_byte.unwrap() {
                if !searching {
                    start_addr = cur_addr;
                    searching = true;
                }
                middle_searching = true;
                search_byte = search_str_bytes.as_mut().unwrap().next(); // set to the next search byte
                if search_byte.is_none() {
                    // use next search string when last one is finished
                    search_str_bytes = search_strings.next();
                    search_strings_idx += 1;
                    middle_searching = false;
                    search_byte = match search_str_bytes {
                        Some(ref mut search_str_bytes) => search_str_bytes.next(),
                        None => {
                            write_image(&src_f, start_addr, cur_addr, hits);
                            if verbose {
                                println!("Found {}. image at {}", hits, start_addr);
                            }
                            src_f
                                .seek(SeekFrom::Start(cur_addr + (BUFFER_LEN as u64 - cur_byte)))
                                .expect("Failed to seek file");
                            hits += 1;
                            search_strings_idx = 0;
                            reset_search_byte = true;
                            searching = false;
                            None
                        }
                    }
                }
            } else if middle_searching {
                reset_search_byte = true;
                middle_searching = false;

                if search_strings_idx == 0 {
                    searching = false;
                }
            }

            if reset_search_byte {
                search_strings_iter_cloned = search_strings_iter.clone();
                search_strings = search_strings_iter_cloned.iter_mut();
                search_str_bytes = search_strings.nth(search_strings_idx);
                search_byte = search_str_bytes.as_mut().unwrap().nth(0);
                reset_search_byte = false;
            }

            cur_addr += 1;
            cur_byte += 1;
        }
    }

    let time = Instant::now() - start_time;
    let mut size = cur_addr / 1000;
    let mut unit = "KB";
    if size > 1000 {
        unit = "MB";
        size = size / 1000;
    } else if size > 1000 {
        unit = "GB";
        size = size / 1000;
    }
    println!(
        "Processed {}{} with {} jpegs in {} ms",
        size,
        unit,
        hits,
        time.as_millis()
    );
}

fn write_image(mut src_f: &std::fs::File, start_addr: u64, end_addr: u64, hits: u32) {
    let mut cur_addr = start_addr;
    let mut dst_f =
        fs::File::create(format!("img-{}.jpg", hits)).expect("Failed to create new image file");
    src_f
        .seek(SeekFrom::Start(start_addr))
        .expect("Failed to seek file");
    while cur_addr < end_addr {
        if cur_addr + BUFFER_LEN as u64 >= end_addr {
            let mut shorter_buff = vec![0; (1 + end_addr - cur_addr).try_into().unwrap()];
            src_f.read(&mut shorter_buff).unwrap();
            dst_f
                .write(&shorter_buff)
                .expect("Failed to write final buffer to image");
        } else {
            let buffer = &mut [0; BUFFER_LEN];
            src_f.read(buffer).expect("Failed to read buffer from file");
            dst_f
                .write(buffer)
                .expect("Failed to write buffer to image");
        }
        cur_addr += BUFFER_LEN as u64;
    }
}
