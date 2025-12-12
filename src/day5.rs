use std::cmp::Ordering;
use std::io::Read;
use std::thread::current;

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
    let max_fresh_id = fresh.iter().max_by_key(|range| range.max).unwrap().max;
    // println!("Maximum fresh id size: {}", max_fresh_id);

    fresh.sort_by_key(|range| range.min);

    let mut count = 0usize;

    let mut fresh_left: &[FreshRange] = fresh;
    let mut active: Vec<&FreshRange> = vec![];

    let mut current = fresh_left[0].min;

    while !fresh_left.is_empty() {
        // Add all valid sets that contain current
        // Sort them my max value.
        match fresh_left.binary_search_by_key(&(current + 1), |x| x.min) {
            Ok(index) | Err(index) => {
                for element in fresh_left[..index].iter() {
                    match active.binary_search_by_key(&element.max, |x| x.max) {
                        Ok(pos) | Err(pos) => active.insert(pos, element),
                    }
                }

                fresh_left = &fresh_left[index..];
            }
        };

        assert!(!active.is_empty());

        // increment current to one past the smallest max value.
        // update count
        if (active[0].max >= current) {
            count += active[0].max - current + 1;
            current = active[0].max + 1;
        }

        // remove any sets outside of this
        match active.binary_search_by_key(&(current), |x| x.max) {
            Ok(index) | Err(index) => active = active[index..].to_vec(),
        };
    }

    count
}
