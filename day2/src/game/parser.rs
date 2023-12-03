use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{
        self,
        complete::{digit1, space1},
    },
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult,
};

use crate::game::{Cube, Game, Subset};

pub fn game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tuple((tag("Game"), space1))(input)?;
    let (input, id) = map_res(digit1, str::parse)(input)?;
    let (input, _) = tuple((character::complete::char(':'), space1))(input)?;

    let mut game = Game::new(id);

    let (input, subsets) = subsets(input)?;

    game.set_subsets(subsets);

    Ok((input, game))
}

fn subsets(input: &str) -> IResult<&str, Vec<Subset>> {
    separated_list1(tag("; "), subset)(input)
}

fn subset(input: &str) -> IResult<&str, Subset> {
    let color = alt((red, green, blue));
    let (input, colors) = separated_list1(tag(", "), color)(input)?;

    let rgb = colors.iter().fold([0, 0, 0], |[r, g, b], cube| match cube {
        Cube::Red(n) => [r + n, g, b],
        Cube::Green(n) => [r, g + n, b],
        Cube::Blue(n) => [r, g, b + n],
    });

    Ok((input, Subset::from(rgb)))
}

fn red(input: &str) -> IResult<&str, Cube> {
    map(color("red"), Cube::Red)(input)
}

fn green(input: &str) -> IResult<&str, Cube> {
    map(color("green"), Cube::Green)(input)
}

fn blue(input: &str) -> IResult<&str, Cube> {
    map(color("blue"), Cube::Blue)(input)
}

fn color(color: &str) -> impl FnMut(&str) -> IResult<&str, usize> + '_ {
    move |input: &str| -> IResult<&str, usize> {
        let (input, n) = map_res(
            separated_pair(digit1, space1, tag(color)),
            |(d, _): (&str, _)| d.parse::<usize>(),
        )(input)?;

        Ok((input, n))
    }
}

impl From<[usize; 3]> for Subset {
    fn from([r, g, b]: [usize; 3]) -> Self {
        Self { r, g, b }
    }
}

#[cfg(test)]
mod tests {
    use crate::game::{Cube, Game};

    #[test]
    fn parse_game() {
        let (rem, game) =
            super::game("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue")
                .unwrap();
        assert_eq!(
            game,
            Game::new(2).with_subsets(vec![[0, 2, 1], [1, 3, 4], [0, 1, 1]])
        );
        assert!(rem.is_empty());
    }

    #[test]
    fn parse_subset() {
        let (rem, subset) = super::subset("3 green, 4 blue, 1 red").unwrap();
        assert!(rem.is_empty());
        assert_eq!(subset.r, 1, "red does not match");
        assert_eq!(subset.g, 3);
        assert_eq!(subset.b, 4);
    }

    #[test]
    fn parse_colors() {
        let (_, c) = super::red("1 red").unwrap();
        assert_eq!(c, Cube::Red(1));

        let (_, c) = super::green("2 green").unwrap();
        assert_eq!(c, Cube::Green(2));

        let (_, c) = super::blue("3 blue").unwrap();
        assert_eq!(c, Cube::Blue(3));
    }
}
