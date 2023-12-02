set dotenv-load

day_template := '''
#![deny(clippy::all, clippy::pedantic, rust_2018_idioms)]

// TODO: Set correct day
const INPUT: &str = include_str!("../../input/day{{day}}.txt");

pub fn main() {
    let input = input_generator(INPUT);

    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

pub fn input_generator(input: &str) -> Vec<usize> {
  todo!()
}

fn part1(input: &[usize]) -> usize {
  todo!()
}

fn part2(input: &[usize]) -> usize {
  todo!()
}

#[cfg(test)]
mod tests {
}
'''

default:
  just --list

# Create a new workspace for `day`. Generates a basic main.rs as well.
new day:
  #!/bin/sh
  if [ ! -d day{{day}} ]; then
    cargo new --vcs none day{{day}}
    echo '{{ day_template }}' | rustfmt > day{{day}}/src/main.rs
  else
    echo 'nothing to do'
  fi

# Print the `main.rs` template for testing.
print:
  @echo '{{ day_template }}' | rustfmt

# Download input for `day`.
input day:
  curl \
    --cookie "session=$AOC_SESSION" \
    --output ./input/day{{day}}.txt \
    https://adventofcode.com/2023/day/{{ day }}/input

run day:
  cargo r -p day{{day}}
