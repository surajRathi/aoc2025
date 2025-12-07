use std::io::Read;

fn read_file() -> Vec<Vec<bool>> {
    let path = std::path::Path::new("./data/day4.txt");
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

    s.lines()
        .map(|x| x.chars().map(|y| y == '@').collect())
        .collect()
}

struct Map {
    map: Vec<Vec<bool>>,
    rows: usize,
    cols: usize,
    // TODO: can we indicate that inner is fixed size?
}

impl Map {
    fn new(map: Vec<Vec<bool>>) -> Result<Map, String> {
        if map.len() == 0 {
            return Err("Map has no rows.".to_string());
        }

        let rows = map.len();

        let cols = map[0].len();
        for (x, row) in map.iter().enumerate() {
            if row.len() != cols {
                return Err(format!(
                    "At row {} map has {} columns instead of {}.",
                    x,
                    row.len(),
                    cols
                ));
            }
        }

        Ok(Map { map, rows, cols })
    }

    fn valid(&self, i: i32, j: i32) -> bool {
        0 <= i && i < self.rows as i32 && 0 <= j && j < self.cols as i32
    }

    fn neighbors(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        let mut list: Vec<(usize, usize)> = vec![];

        for di in [-1i32, 0, 1] {
            for dj in [-1i32, 0, 1] {
                if di == 0 && dj == 0 {
                    continue;
                }

                let ni = i as i32 + di;
                let nj = j as i32 + dj;

                if self.valid(ni, nj) && self.map[ni as usize][nj as usize] {
                    list.push((ni as usize, nj as usize));
                }
            }
        }

        assert!(list.len() < 9, "({}, {}) -> {:?}", i, j, list);

        list
    }

    fn accessible(&self, i: usize, j: usize) -> bool {
        self.neighbors(i, j).iter().count() < 4
    }
}

trait GridPrint {
    fn print(&self) -> String;
}

impl GridPrint for bool {
    fn print(&self) -> String {
        if *self {
            "&".to_string()
        } else {
            ".".to_string()
        }
    }
}
impl GridPrint for i32 {
    fn print(&self) -> String {
        if *self == -1 {
            ".".to_string()
        } else {
            self.to_string()
        }
    }
}
fn print_grid<T: GridPrint>(map: &Vec<Vec<T>>) {
    for row in map.iter() {
        for val in row.iter() {
            print!("{}", GridPrint::print(val));
        }
        println!();
    }
}

fn get_neighbor_map(map: &Map) -> Vec<Vec<i32>> {
    map.map
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, val)| {
                    if *val {
                        map.neighbors(i, j).iter().count() as i32
                    } else {
                        -1
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>()
}

#[allow(dead_code)]
pub fn part1() {
    let map: Map = Map::new(read_file()).expect("Input data is not square.");

    // print_grid(&map.map);
    // println!();

    let sum = (0..map.rows)
        .flat_map(|i| (0..map.cols).map(move |j| (i, j)))
        .filter(|&(i, j)| map.map[i][j] && map.accessible(i, j))
        .count();

    print_grid(&get_neighbor_map(&map));
    println!();

    println!("Sum: {}", sum);
    return;
}

#[allow(dead_code)]
pub fn part2() {
    let mut map: Map = Map::new(read_file()).expect("Input data is not square.");

    print_grid(&get_neighbor_map(&map));
    println!();

    let mut total_removed = 0;
    loop {
        let cells_to_remove: Vec<(usize, usize)> = (0..map.rows)
            .flat_map(|i| (0..map.cols).map(move |j| (i, j)))
            .filter(|&(i, j)| map.map[i][j] && map.accessible(i, j))
            .collect();

        let removed = cells_to_remove.len();

        // 2. Iterate over the collected coordinates and apply the mutation.
        // The map is now borrowed mutably and exclusively.
        for (i, j) in cells_to_remove {
            map.map[i][j] = false;
        }

        print_grid(&get_neighbor_map(&map));
        println!();

        total_removed += removed;
        if removed == 0 {
            break;
        }
    }

    println!("Total removed: {}", total_removed);
    return;
}
