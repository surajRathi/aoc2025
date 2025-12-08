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
