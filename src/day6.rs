use std::io;
use std::io::Read;
use std::ops::{Add, Mul};
use std::slice::Iter;

type OperationFn = fn(u64, u64) -> u64;

struct OperationData {
    func: OperationFn,
    default: u64,
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

    type OperationFn = fn(u64, u64) -> u64;

    let y = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|ch| match ch {
            "+" => OperationData {
                func: (u64::add as OperationFn),
                default: 0,
            },
            "*" => OperationData {
                func: u64::mul as OperationFn,
                default: 1,
            },
            _ => panic!("Unknown operation type {ch}"),
        })
        .collect::<Vec<OperationData>>();

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

fn do_problem_2() -> Result<u64, io::Error> {
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
                .map(|s| s.trim())
                .collect::<Vec<&str>>()
        })
        .collect::<Vec<Vec<&str>>>();

    type OperationFn = fn(u64, u64) -> u64;

    let y = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|ch| match ch {
            "+" => OperationData {
                func: (u64::add as OperationFn),
                default: 0,
            },
            "*" => OperationData {
                func: u64::mul as OperationFn,
                default: 1,
            },
            _ => panic!("Unknown operation type {ch}"),
        })
        .collect::<Vec<OperationData>>();

    assert!(x.iter().all(|x| x.len() == y.len()));

    let mut sum = 0;
    for (op, args) in std::iter::zip(y, zip::<&str>(&x)) {
        let res = parse_rtl_columns(args)
            .into_iter()
            .fold(op.default, |acc: u64, val| (op.func)(acc, val));
        sum += res;
    }
    Ok(sum)
}

fn parse_rtl_columns(string: Vec<&&str>) -> Vec<u64> {
    if string.is_empty() {
        return vec![];
    }

    let max_len = string.iter().map(|s| s.len()).max().unwrap();

    (max_len - 1..=0)
        .map(|i| {
            string
                .iter()
                .filter_map(|s| {
                    if i < s.len() {
                        Some(s[i..=i].to_string())
                    } else {
                        None
                    }
                })
                .collect::<Vec<String>>()
                .join("")
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
