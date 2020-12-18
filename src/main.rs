use aoc2020::day1::{day1a, day1b};
use aoc2020::day10::{day10a, day10b};
use aoc2020::day11::{day11a, day11b};
use aoc2020::day2::{day2a, day2b};
use aoc2020::day3::{day3a, day3b};
use aoc2020::day4::{day4a, day4b};
use aoc2020::day5::{day5a, day5b};
use aoc2020::day6::{day6a, day6b};
use aoc2020::day7::{day7a, day7b};
use aoc2020::day8::{day8a, day8b};
use aoc2020::day9::{day9a, day9b};
use aoc2020::day12::{day12a, day12b};
use aoc2020::day13::{day13a, day13b};
use aoc2020::day14::{day14a, day14b};
use aoc2020::day15::{day15a, day15b};
use aoc2020::day16::{day16a, day16b};
use aoc2020::day17::{day17a, day17b};
use aoc2020::day18::{day18a, day18b};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let problem = args.get(1).map(|s| s.as_str()).unwrap_or("None");
    let result = match problem {
        "day1a" => day1a(),
        "day1b" => day1b(),
        "day2a" => day2a(),
        "day2b" => day2b(),
        "day3a" => day3a(),
        "day3b" => day3b(),
        "day4a" => day4a(),
        "day4b" => day4b(),
        "day5a" => day5a(),
        "day5b" => day5b(),
        "day6a" => day6a(),
        "day6b" => day6b(),
        "day7a" => day7a(),
        "day7b" => day7b(),
        "day8a" => day8a(),
        "day8b" => day8b(),
        "day9a" => day9a(),
        "day9b" => day9b(),
        "day10a" => day10a(),
        "day10b" => day10b(),
        "day11a" => day11a(),
        "day11b" => day11b(),
        "day12a" => day12a(),
        "day12b" => day12b(),
        "day13a" => day13a(),
        "day13b" => day13b(),
        "day14a" => day14a(),
        "day14b" => day14b(),
        "day15a" => day15a(),
        "day15b" => day15b(),
        "day16a" => day16a(),
        "day16b" => day16b(),
        "day17a" => day17a(),
        "day17b" => day17b(),
        "day18a" => day18a(),
        "day18b" => day18b(),
        _ => "We haven't solved that yet".to_string(),
    };
    println!("{}", result);
}
