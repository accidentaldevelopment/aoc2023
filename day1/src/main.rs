const INPUT: &str = include_str!("../../input/day1.txt");

pub fn main() {
    // TODO: There shouldn't be any need to allocate anything in this whole block.
    let input = input_generator(INPUT).collect::<Vec<_>>();

    println!("part1: {}", part1(input.clone().into_iter()));
    println!("part2: {}", part2(input.into_iter()));
}

pub fn input_generator(input: &str) -> impl Iterator<Item = &str> {
    input.lines()
}

fn part1<'a>(input: impl Iterator<Item = &'a str>) -> u32 {
    input.fold(0, |sum, line| {
        let mut chars = line.chars();

        // SAFETY: We're going assume to aoc isn't giving us shit input, and there will always be
        // at least one digit.
        let first = chars.find_map(|c| c.to_digit(10)).unwrap();
        let last = chars.rev().find_map(|c| c.to_digit(10)).unwrap_or(first);

        sum + (first * 10) + last
    })
}

fn part2<'a>(input: impl Iterator<Item = &'a str>) -> u32 {
    static NUMS: [&[u8]; 10] = [
        b"zero", b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
    ];

    // SAFETY: oof, where to start? We're assuming ascii for the whole thing.
    //   Anywhere we're using [] to index, we could runtime panic.
    //   The `as u8` below _could_ panic, but won't because it's a static array and we know there
    //   are never more than 10 indexes. And `10` can fit in a u8.
    fn find_digit(line: &[u8]) -> impl Fn(usize) -> Option<u8> + '_ {
        |i: usize| {
            if line[i].is_ascii_digit() {
                return Some(line[i] - b'0');
            }

            #[allow(clippy::cast_possible_truncation)]
            NUMS.iter()
                .enumerate()
                .find_map(|(ni, num)| line[i..].starts_with(num).then_some(ni as u8))
        }
    }

    input.fold(0, |sum, line| {
        let line = line.as_bytes();
        let first = (0..line.len()).find_map(find_digit(line)).unwrap();

        let last = (0..line.len())
            .rev()
            .find_map(find_digit(line))
            .unwrap_or(first);

        sum + u32::from((first * 10) + last)
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let lines = super::input_generator(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );

        assert_eq!(super::part1(lines), 142);
    }

    #[test]
    fn part2() {
        let lines = super::input_generator(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );

        assert_eq!(super::part2(lines), 281);
    }
}
