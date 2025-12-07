use std::io::Read;

fn read_file() -> Vec<String> {
    let path = std::path::Path::new("./data/day3.txt");
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

    s.lines().map(|x| x.to_string()).collect()
}

fn max_jolt(s: &String) -> u128 {
    let max_first = s[..s.len() - 1].chars().into_iter().max().unwrap();
    let max_second = s[s.find(max_first).unwrap() + 1..].chars().into_iter().max().unwrap();

    (max_first.to_string() + &max_second.to_string())
        .parse::<u128>()
        .unwrap()
}

#[allow(dead_code)]
pub fn part1() {
    let sum: u128 = read_file()
        .iter()
        .map(|x| max_jolt(x))
        .map(|x| {
            println!("{}", x);
            x
        })
        .sum();
    println!("Sum: {}", sum);
    return;
}
