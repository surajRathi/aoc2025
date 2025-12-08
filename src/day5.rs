use std::cmp::Ordering;
use std::io::Read;

fn read_file() -> Inventory {
    let path = std::path::Path::new("./data/day5.txt");
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

    let mut inv = Inventory::default();

    let mut id_section = false;
    for (line_id, line) in s.lines().enumerate() {
        if line.trim().is_empty() {
            println!("Section completed at {}", line_id);
            id_section = true;
            continue;
        }

        if id_section {
            inv.ids.push(line.parse::<i64>().unwrap());
        } else {
            let (start, end) = line.split_once("-").expect(
                format!(
                    "Line cannot be split at index {}: `{}` id_section={}",
                    line_id, line, id_section
                )
                .as_str(),
            );
            inv.fresh.push(FreshRange::new(
                start.to_string().parse::<i64>().unwrap(),
                end.to_string().parse::<i64>().unwrap(),
            ));
        }
    }

    inv
}

struct FreshRange {
    min: i64,
    max: i64,
}

impl FreshRange {
    fn new(min: i64, max: i64) -> FreshRange {
        assert!(min <= max);
        FreshRange { min, max }
    }

    // TODO: should i always accept elements as references?
    fn contains(&self, val: &i64) -> bool {
        self.min <= *val && *val <= self.max
    }
}

struct Inventory {
    ids: Vec<i64>,
    fresh: Vec<FreshRange>,
}

impl Default for Inventory {
    fn default() -> Self {
        Inventory {
            ids: vec![],
            fresh: vec![],
        }
    }
}

impl Inventory {
    fn remove_stale(&mut self) {
        self.ids
            .retain(|id| self.fresh.iter().any(|range| range.contains(id)));
    }
}

#[allow(dead_code)]
pub fn part1() {
    let mut inv = read_file();

    println!("Inventory id size: {}", inv.ids.len());

    inv.remove_stale();

    println!("Inventory fresh id size: {}", inv.ids.len());

    return;
}

#[allow(dead_code)]
pub fn part2() {
    let mut fresh = read_file().fresh;

    let max_fresh_id = fresh.iter().max_by_key(|range| range.max).unwrap().max;
    println!("Maximum fresh id size: {}", max_fresh_id);

    // TODO: Making a struct sortable seems very annoying, need to implement 4 traits.
    // fresh.sort_by(|lhs, rhs| {
    //     if lhs.min.eq(&rhs.min) {
    //         // Not required, but why not?
    //         return lhs.max.cmp(&rhs.max);
    //     }
    //     return lhs.min.cmp(&rhs.min);
    // });

    fresh.sort_by_key(|range| range.min);

    let mut count = 0usize;
    let mut next_range = 0;

    let mut active_ids: Vec<&FreshRange> = vec![];

    let mut i = 0;
    loop {
        while next_range < fresh.len() && {
            assert!(fresh[next_range].min >= i);
            fresh[next_range].min == i
        } {
            // TODO insert sorted by the end?
            active_ids.push(&fresh[next_range]);
            next_range += 1;
        }

        active_ids.retain(|r: &&FreshRange| r.contains(&i));

        if !active_ids.is_empty() {
            count += 1;
        } else {
            i += 1; // TODO use the max in the active range?
        }
    }

    println!("Maximum possible number of fresh ingredients: {}", count);

    return;
}
