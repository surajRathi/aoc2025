use std::fmt::Formatter;
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
            inv.ids.push(line.parse::<usize>().unwrap());
        } else {
            let (start, end) = line.split_once("-").expect(
                format!(
                    "Line cannot be split at index {}: `{}` id_section={}",
                    line_id, line, id_section
                )
                .as_str(),
            );
            inv.fresh.push(FreshRange::new(
                start.to_string().parse::<usize>().unwrap(),
                end.to_string().parse::<usize>().unwrap(),
            ));
        }
    }

    inv
}

pub(crate) struct FreshRange {
    min: usize,
    max: usize,
}

impl FreshRange {
    pub(crate) fn new(min: usize, max: usize) -> FreshRange {
        assert!(min <= max);
        FreshRange { min, max }
    }

    // TODO: should i always accept elements as references?
    fn contains(&self, val: &usize) -> bool {
        self.min <= *val && *val <= self.max
    }
}

impl std::fmt::Debug for FreshRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Range")
            .field("min", &self.min)
            .field("max", &self.max)
            .finish()
    }
}

struct Inventory {
    ids: Vec<usize>,
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

    let count = get_total_possible_fresh(&mut fresh);

    println!("Maximum possible number of fresh ingredients: {}", count);

    return;
}

pub fn get_total_possible_fresh(fresh: &mut Vec<FreshRange>) -> usize {
    fresh.sort_by_key(|range| range.min);

    let mut count = 0usize;
    let mut unprocessed_ranges: &[FreshRange] = fresh;
    let mut current = usize::MIN;

    while !unprocessed_ranges.is_empty() {
        // If current would not be in any sets, fast-forward it to one where it would be in a set.
        current = std::cmp::max(current, unprocessed_ranges.first().unwrap().min);

        println!("Current: {}", current);
        println!("Count: {}", count);

        // Extract the active ranges from the
        let index = unprocessed_ranges.partition_point(|range| range.min <= current);
        let active: &[FreshRange];
        (active, unprocessed_ranges) = (unprocessed_ranges).split_at(index);

        println!("Active size: {}\t{:?}", active.len(), active);
        println!(
            "Left size: {}\t{:?}",
            unprocessed_ranges.len(),
            unprocessed_ranges
        );

        assert!(!active.is_empty());

        // increment current to one past the largest max value.
        // update count
        // Get max value in active.
        let active_max = active.iter().max_by_key(|range| range.max).unwrap().max;
        if current <= active_max {
            println!("Increment: {}", active_max - current + 1);
            count += active_max - current + 1;
            current = active_max + 1;
        } else {
            current += 1;
        }

        println!();
    }

    count
}
