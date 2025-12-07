use std::io::Read;

pub fn day1() {
    println!("Hello, world!");

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

    let mut idx: i32 = 50;
    let max = 99;

    let mut password: i32 = 0;

    for line in s.lines() {
        if line.len() < 2 {
            continue;
        }

        let (dir, count) = line.split_at(1);

        let count_int = match count.parse::<i32>() {
            Err(why) => panic!("couldn't read {}: {}", path.display(), why),
            Ok(value) => value,
        };

        let dir_int = match dir {
            "R" => 1,
            "L" => -1,
            _ => panic!("Invalid direction: {}", dir),
        };

        println!(
            "To {} Applying {} * {}. From {}",
            idx, dir_int, count_int, line
        );

        idx += dir_int * count_int;
        idx = idx.rem_euclid(max + 1);

        if idx == 0 {
            password += 1;
        }
    }

    println!("Final index: {}", idx);

    println!("Password: {}", password);
    return;
}
