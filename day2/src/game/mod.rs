mod parser;

#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Debug)]
pub struct Game {
    pub id: usize,
    subsets: Vec<Subset>,
}

#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Debug)]
pub enum Cube {
    Red(usize),
    Green(usize),
    Blue(usize),
}

#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Debug)]
pub struct Subset {
    pub r: usize,
    pub g: usize,
    pub b: usize,
}

impl Game {
    const MAX_RED: usize = 12;
    const MAX_GREEN: usize = 13;
    const MAX_BLUE: usize = 14;

    pub fn new(id: usize) -> Self {
        Self {
            id,
            subsets: vec![],
        }
    }

    pub fn set_subsets(&mut self, subsets: Vec<Subset>) {
        self.subsets = subsets;
    }

    pub fn is_valid(&self) -> bool {
        for subset in &self.subsets {
            let &Subset { r, g, b } = subset;
            if r > Self::MAX_RED || g > Self::MAX_GREEN || b > Self::MAX_BLUE {
                return false;
            }
        }
        true
    }

    pub fn power(&self) -> usize {
        let [r, g, b]: [usize; 3] = self.subsets.iter().fold([0, 0, 0], |[r, g, b], subset| {
            [r.max(subset.r), g.max(subset.g), b.max(subset.b)]
        });
        r * g * b
    }

    #[cfg(test)]
    fn with_subsets(mut self, subsets: Vec<[usize; 3]>) -> Self {
        self.subsets = subsets
            .into_iter()
            .map(|[r, g, b]| Subset { r, g, b })
            .collect();
        self
    }
}

impl<'a> TryFrom<&'a str> for Game {
    type Error = nom::Err<nom::error::Error<&'a str>>;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        let (_, game) = parser::game(input)?;
        Ok(game)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn power() {
        let game = super::Game::new(1).with_subsets(vec![[4, 0, 3], [1, 2, 6], [0, 2, 0]]);
        assert_eq!(game.power(), 48);
    }
}
