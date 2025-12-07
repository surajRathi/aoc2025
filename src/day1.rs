use std::io::Read;

fn read_file() -> String {
    let path = std::path::Path::new("./data/day1.txt");
    println!("Reading {}", path.display());

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match std::fs::File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", path.display(), why),
        Ok(_) => {}
    }
    s
}

fn parse_line(line: &str) -> (i32, i32) {
    let (dir, count) = line.split_at(1);

    (
        match dir {
            "R" => 1,
            "L" => -1,
            _ => panic!("Invalid direction: {}", dir),
        },
        match count.parse::<i32>() {
            Err(why) => panic!("Invalid count: {}. Because: {}", count, why),
            Ok(value) => value,
        },
    )
}
#[allow(dead_code)]
pub fn part1() {
    let mut idx: i32 = 50;
    let max = 99;

    let mut password: i32 = 0;

    for line in read_file().lines() {
        if line.len() < 2 {
            continue;
        }

        let (dir, count) = parse_line(line);

        idx += dir * count;
        idx = idx.rem_euclid(max + 1);

        if idx == 0 {
            password += 1;
        }
    }

    println!("Final index: {}", idx);

    println!("Password: {}", password);
    return;
}

#[allow(dead_code)]

pub fn part2() {
    let mut idx: i32 = 50;
    let max = 99;

    let mut password: i32 = 0;

    for line in read_file().lines() {
        if line.len() < 2 {
            continue;
        }

        let (dir, count) = parse_line(line);

        // Brute force!
        for _ in 0..count {
            idx += dir;
            idx = idx.rem_euclid(max + 1);

            if idx == 0 {
                password += 1;
            }
        }
    }

    println!("Password: {}", password);
    return;
}
