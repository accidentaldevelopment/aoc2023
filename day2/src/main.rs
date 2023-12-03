use game::Game;

const INPUT: &str = include_str!("../../input/day2.txt");

mod game;

pub fn main() {
    let input = input_generator(INPUT).collect::<Vec<_>>();

    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

pub fn input_generator(input: &str) -> impl Iterator<Item = &str> {
    input.lines()
}

fn part1(input: &[&str]) -> usize {
    input
        .iter()
        .filter_map(|&line| {
            let game = Game::try_from(line).ok()?;
            if game.is_valid() {
                Some(game.id)
            } else {
                None
            }
        })
        .sum()
}

#[allow(dead_code)]
fn part2(input: &[&str]) -> usize {
    input
        .iter()
        .map(|&line| {
            // SAFETY: once again assuming we always have valid input
            let game = Game::try_from(line).unwrap();
            game.power()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    const TXT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn part1() {
        let input = super::input_generator(TXT);
        assert_eq!(super::part1(&input.collect::<Vec<_>>()), 8);
    }

    #[test]
    fn part2() {
        let input = super::input_generator(TXT);
        assert_eq!(super::part2(&input.collect::<Vec<_>>()), 2286);
    }
}
