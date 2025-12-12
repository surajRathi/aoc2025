use std::fmt::Formatter;
use std::io;
use std::io::Read;

fn read_file() -> Result<usize, io::Error> {
    let path = std::path::Path::new("./data/day6.txt");
    println!("Reading {}", path.display());

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = std::fs::File::open(&path)?;
    let mut s = String::new();
    let _ = file.read_to_string(&mut s)?;

    let mut lines = s.lines().into_iter();

    // if lines.count() != 4 {
    //     return Err(io::Error::new(
    //         std::io::ErrorKind::InvalidData,
    //         "File had {lines.count()} lines",
    //     ));
    // }

    let x = lines
        .by_ref()
        .take(3)
        .map(|line| {
            line.split("\\S+")
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let y = lines
        .skip(3)
        .take(1)
        .next()
        .unwrap()
        .split("\\S+")
        .map(|ch| match ch {
            "+" => std::ops::Add::add,
            "*" => std::ops::Mul::mul,
            _ => panic!("Unknown operation type {ch}"),
        })
        .collect();

    Ok(3usize)
}

#[allow(dead_code)]
pub fn part1() {
    let lines = read_file();

    return;
}
