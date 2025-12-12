mod day5;

fn main() {
    let mut ranges = vec![day5::FreshRange::new(0, 2),day5::FreshRange::new(0, 1), day5::FreshRange::new(1, 2)];
    let count = day5::get_total_possible_fresh(&mut ranges);
    println!("Total possible fresh range: {}", count);
    // day5::part2();
}
