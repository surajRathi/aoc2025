use std::io::Read;

fn read_file() -> Result<String, std::io::Error> {
    let path = std::path::Path::new("./data/day7.txt");
    println!("Reading {}", path.display());

    let mut s = String::new();
    let _ = std::fs::File::open(&path)?.read_to_string(&mut s);

    Ok(s)
}

fn parse_file(data: &String) -> Result<World, String> {
    if data.is_empty() {
        return Err("Data is empty".to_string());
    }

    let mut lines = data.lines();

    let source_line = lines
        .next()
        .ok_or_else(|| "Data has no lines.".to_string())?;

    let mut world = World::new(
        source_line.len(),
        source_line.chars().position(|c| c == 'S').ok_or_else(|| {
            format!(
                "First line of the file does not contain a 'S' - {}",
                source_line
            )
        })?,
    );

    for line in data.lines() {
        if line.len() != world.cols {
            return Err(format!(
                "Line has invalid length {} instead of {}. Line: '{}'",
                line.len(),
                world.cols,
                line
            ));
        }
        world.world.push(
            line.chars()
                .enumerate()
                .filter(|(_, c)| c == &'^')
                .map(|(idx, _)| idx)
                .collect(),
        );
    }
    Ok(world)
}

struct World {
    world: Vec<std::collections::HashSet<usize>>,
    cols: usize,
    source_col: usize,
}

impl World {
    fn new(cols: usize, source_col: usize) -> World {
        assert!(source_col < cols);

        World {
            world: vec![],
            cols,
            source_col,
        }
    }
}

pub fn part1() {
    let world = read_file()
        .map_err(|e| e.to_string())
        .and_then(|s| parse_file(&s))
        .expect("File parsing failed");

    let mut active_rays = std::collections::HashSet::new();
    active_rays.insert(world.source_col);

    let mut num_splits = 0u128;

    for splitter_row in &world.world {
        let mut next_active_rays = std::collections::HashSet::new();

        for ray in &active_rays {
            if splitter_row.contains(ray) {
                num_splits += 1;
                if *ray > 0 {
                    next_active_rays.insert(ray - 1);
                }

                if *ray + 1 < world.cols {
                    next_active_rays.insert(ray + 1);
                }
            } else {
                next_active_rays.insert(*ray);
            }
        }

        // TODO: Print row + activepar
        active_rays = next_active_rays;
    }
    println!("Number of splits: {}", num_splits);
}
