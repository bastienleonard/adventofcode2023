pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7_part1;
pub mod day7_part2;
pub mod day8;

fn main() {
    assert_eq!(day1::part1(), 54605);
    assert_eq!(day1::part2(), 55429);
    assert_eq!(day2::part1(), 2476);
    assert_eq!(day2::part2(), 54911);
    assert_eq!(day3::part1(), 543867);
    assert_eq!(day3::part2(), 79613331);
    assert_eq!(day4::part1(), 25571);
    assert_eq!(day4::part2(), 8805731);
    assert_eq!(day5::part1(), 318728750);
    assert_eq!(day5::part2(), 37384986);
    assert_eq!(day6::part1(), 170000);
    assert_eq!(day6::part2(), 20537782);
    assert_eq!(day7_part1::part1(), 251806792);
    assert_eq!(day7_part2::part2(), 252113488);
    assert_eq!(day8::part1(), 13207);
}
