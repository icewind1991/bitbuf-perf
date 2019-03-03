extern crate bitreader;
extern crate bitstream_io;
extern crate bitstream_reader;

use std::collections::HashMap;
use std::time::{Duration, Instant};

use bitstream_reader::BitStream;

type TestFn = fn(Vec<u8>, usize) -> u32;

fn main() {
    let vec = vec![1u8; 1024 * 1024];
    let sizes = [
        (1, 1048576),
        (5, 6501165),
        (8, 1048576),
        (10, 71512815),
        (20, 1775225187)
    ];
    let mut methods: HashMap<String, TestFn> = HashMap::new();
    methods.insert("bitstream_reader".to_owned(), test_bitstream_reader);
    methods.insert("bitstream_reader_stream".to_owned(), test_bitstream_reader_stream);
    methods.insert("bitstream_io".to_owned(), test_bitstream_io);
    methods.insert("bitreader".to_owned(), test_bitreader);

    for (name, method) in methods.iter() {
        for (size, expected) in sizes.iter() {
            let speed = time_fn(&vec, *size, *expected, *method);

            println!("{} at size {}: {}", name, size, format_speed(speed));
        }
    }

//    let mut padded = vec.clone();
//    padded.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0]);
//
//    for (size, expected) in sizes.iter() {
//        let speed = time_fn(&padded, *size, *expected, test_bitstream_reader_padded);
//
//        println!("bitstream_reader(padded) at size {}: {}", size, format_speed(speed));
//    }
}

fn format_speed(size: f64) -> String {
    let units = ["B", "KB", "MB", "GB"];
    let mut in_unit = size;
    for unit in units.iter() {
        if in_unit < 1024.0 {
            return format!("{:.2}{}/s", in_unit, unit);
        }
        in_unit = in_unit / 1024.0;
    }
    return format!("{:.2}TB/s", in_unit);
}

fn time_fn(bytes: &Vec<u8>, size: usize, expected: u32, f: TestFn) -> f64 {
    let len = bytes.len();

    let mut duration = Duration::new(0, 0);

    for _ in 0..9 {
        let clone = bytes.clone();
        let start = Instant::now();
        let result = f(clone, size);
        duration += start.elapsed();

        // prevent the compiler from optimizing away our result
        if result != expected {
            panic!("invalid result, got {}, expected {} when reading with size {}", result, expected, size);
        }
    }

    let duration_float = (duration.as_secs() as f64) + (duration.subsec_nanos() as f64) / 1_000_000_000 as f64;

    (len * 10) as f64 / duration_float
}

fn test_bitstream_reader(vec: Vec<u8>, size: usize) -> u32 {
    let buffer = bitstream_reader::BitBuffer::new(vec, bitstream_reader::BigEndian);
    let mut acc = 0u32;
    let mut pos = 0;
    let len = buffer.bit_len();
    loop {
        if pos + size > len {
            return acc;
        }
        let data = buffer.read_int::<u32>(pos, size).unwrap();
        acc = acc.wrapping_add(data);
        pos += size;
    }
}

fn test_bitstream_reader_stream(vec: Vec<u8>, size: usize) -> u32 {
    let buffer = bitstream_reader::BitBuffer::new(vec, bitstream_reader::BigEndian);
    let len = buffer.bit_len();
    let mut stream = BitStream::new(buffer);
    let mut acc = 0u32;
    let mut pos = 0;
    loop {
        if pos + size > len {
            return acc;
        }
        let data = stream.read_sized::<u32>(size).unwrap();
        acc = acc.wrapping_add(data);
        pos += size;
    }
}

fn test_bitstream_io(vec: Vec<u8>, size: usize) -> u32 {
    let mut cursor = std::io::Cursor::new(&vec);
    let mut reader = bitstream_io::BitReader::endian(&mut cursor, bitstream_io::BigEndian);

    let size_32 = size as u32;
    let mut acc = 0u32;
    let len = vec.len() * 8;
    let mut pos = 0;
    loop {
        if pos + size > len {
            return acc;
        }
        let data = reader.read::<u32>(size_32).unwrap();
        acc = acc.wrapping_add(data);
        pos += size;
    }
}

fn test_bitreader(vec: Vec<u8>, size: usize) -> u32 {
    let mut reader = bitreader::BitReader::new(vec.as_slice());

    let size_8 = size as u8;
    let mut acc = 0u32;
    let len = vec.len() * 8;
    let mut pos = 0;
    loop {
        if pos + size > len {
            return acc;
        }
        let data = reader.read_u32(size_8).unwrap();
        acc = acc.wrapping_add(data);
        pos += size;
    }
}