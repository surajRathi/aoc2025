use std::io;
use std::io::Read;
use std::ops::{Add, Mul};
use std::slice::Iter;

type OperationFn = fn(u64, u64) -> u64;

struct OperationData {
    func: OperationFn,
    default: u64,
}

fn parse_op_data(val: &str) -> OperationData {
    match val.trim() {
        "+" => OperationData {
            func: u64::add as OperationFn,
            default: 0,
        },
        "*" => OperationData {
            func: u64::mul as OperationFn,
            default: 1,
        },
        _ => panic!("Unknown operation type {val}"),
    }
}

fn try_parse_op_data(val: &str) -> Option<OperationData> {
    // if !val.trim().is_empty() {
    //     println!("Parsing {val}");
    // }
    match val.trim() {
        "+" => Some(OperationData {
            func: u64::add as OperationFn,
            default: 0,
        }),
        "*" => Some(OperationData {
            func: u64::mul as OperationFn,
            default: 1,
        }),
        _ => None,
    }
}

fn parse_op_data_vec(line: &str) -> Vec<OperationData> {
    line.split_whitespace()
        .map(parse_op_data)
        .collect::<Vec<OperationData>>()
}
fn do_problem() -> Result<u64, io::Error> {
    let path = std::path::Path::new("./data/day6.txt");
    println!("Reading {}", path.display());

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = std::fs::File::open(&path)?;
    let mut s = String::new();
    let _ = file.read_to_string(&mut s)?;
    s = s.trim().to_string();

    let mut lines = s.lines().into_iter();

    let x = lines
        .by_ref()
        .take(4)
        .map(|line| {
            line.split_whitespace()
                .map(|s| {
                    s.parse::<u64>()
                        .expect(format!("Failed to parse number {s} in line {line}").as_str())
                })
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();

    let y = parse_op_data_vec(lines.next().unwrap());

    assert!(x.iter().all(|x| x.len() == y.len()));

    let mut sum = 0;
    for (op, args) in std::iter::zip(y, zip::<u64>(&x)) {
        let res = args
            .iter()
            .fold(op.default, |acc: u64, val| (op.func)(acc, **val));
        sum += res;
    }
    Ok(sum)
}

struct Segment {
    start_index: usize,
    operation: OperationData,
    operands: Vec<u64>,
}
fn do_problem_2() -> Result<u64, io::Error> {
    let path = std::path::Path::new("./data/day6.txt");
    println!("Reading {}", path.display());

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = std::fs::File::open(&path)?;
    let mut s = String::new();
    let _ = file.read_to_string(&mut s)?;
    s = s.trim().to_string();

    let mut lines = s.lines().into_iter();

    let number_lines: Vec<&str> = lines.by_ref().take(4).collect();

    let mut segments: Vec<Segment> = lines
        .by_ref()
        .next()
        .unwrap()
        .bytes()
        .enumerate()
        .filter_map(|(i, b)| {
            let data = try_parse_op_data((b as char).to_string().as_str());
            match data {
                None => None,
                Some(data) => Some(Segment {
                    start_index: i,
                    operation: data,
                    operands: vec![],
                }),
            }
        })
        .collect();

    let mut last: Option<usize> = None;

    for data in segments.iter_mut().rev() {
        let slice: Vec<String> = number_lines
            .iter()
            .map(|line| match last {
                None => line[data.start_index..].to_string(),
                Some(val) => line[data.start_index..val].to_string(),
            })
            .collect();

        data.operands = parse_rtl_columns(&slice);

        last = Some(data.start_index);
    }
    let mut sum = 0;
    for s in segments {
        let res = s
            .operands
            .iter()
            .fold(s.operation.default, |acc: u64, val| {
                (s.operation.func)(acc, *val)
            });
        sum += res;
    }
    Ok(sum)
}

fn parse_rtl_columns(string: &Vec<String>) -> Vec<u64> {
    if string.is_empty() {
        return vec![];
    }

    let max_len = string.iter().map(|s| s.len()).max().unwrap();
    // assert!(string.iter().all(|s| s.len() == max_len), "{:?}", string);

    (0..max_len)
        .rev()
        .map(|i| {
            string
                .iter()
                .filter_map(|s| {
                    if i < s.len() {
                        // println!("{i}: {}", s[i..=i].to_string());
                        let val = s[i..=i].to_string().trim().to_string();
                        return if val.is_empty() { None } else { Some(val) };
                    } else {
                        None
                    }
                })
                .collect::<Vec<String>>()
                .join("")
        })
        .filter(|s| !s.is_empty())
        .map(|s| {
            // println!("{s}, {}", s.parse::<u64>().unwrap());
            s
        })
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}

fn zip<T>(vector_vectors: &Vec<Vec<T>>) -> impl Iterator<Item = Vec<&T>> {
    assert!(!vector_vectors.is_empty());
    let len = vector_vectors[0].len();
    assert!(vector_vectors.iter().by_ref().all(|v| v.len() == len));

    let mut iters: Vec<Iter<T>> = vector_vectors.iter().map(|v| v.iter()).collect();

    (0..len).map(move |_| iters.iter_mut().map(|v| v.next().unwrap()).collect())
}

#[allow(dead_code)]
pub fn part1() {
    let res = do_problem();
    println!("Sum: {}", res.unwrap());
    return;
}

#[allow(dead_code)]
pub fn part2() {
    let res = do_problem_2();
    println!("Sum: {}", res.unwrap());
    return;
}
