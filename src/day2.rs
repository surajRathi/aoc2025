use std::io::Read;

fn read_file() -> Vec<String> {
    let path = std::path::Path::new("./data/day2.txt");
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

    s.split(',').map(|s| s.to_string()).collect()
}

struct Range {
    min: u128,
    max: u128,
}
fn parse_range(line: &str) -> Range {
    let (first, second_minus) = line.split_at(line.find('-').unwrap());
    let second = &second_minus[1..];

    Range {
        min: first
            .trim()
            .parse::<u128>()
            .unwrap_or_else(|e| panic!("First: Couldnt parse {} - {}. Line: {}", first, e, line)),
        max: second.trim().parse::<u128>().unwrap_or_else(|e| {
            panic!("Second: couldn't parse '{}': {}. Line: {}", second, e, line)
        }),
    }
}
#[allow(dead_code)]
pub fn part1() {
    let mut sum: u128 = 0;

    for range in read_file().into_iter().map(|l| parse_range(&l)) {
        for code in range.min..=range.max {
            let s = code.to_string();
            if (s.len() % 2 != 0) {
                continue;
            }

            let (first, second) = s.split_at(s.len() / 2);
            if first == second {
                sum += code;
            }
        }
    }

    println!("Sum: {}", sum);
    return;
}
