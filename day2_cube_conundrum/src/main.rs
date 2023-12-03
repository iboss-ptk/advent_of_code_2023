use cube_set::BAG;
use game::Game;

mod cube_set;
mod game;

fn games() -> impl Iterator<Item = Game> {
    include_str!("input.txt")
        .lines()
        .map(|line| Game::try_from_str(line).unwrap())
}

fn solve_part1(games: impl Iterator<Item = Game>) -> u64 {
    games
        .filter(|game| game.is_possible(&BAG))
        .map(|game| game.game_id)
        .sum()
}

fn solve_part2(games: impl Iterator<Item = Game>) -> u64 {
    games.map(|game| game.min_cube_set().power()).sum()
}

fn main() {
    println!("part 1: {}", solve_part1(games()));
    println!("part 2: {}", solve_part2(games()));
}
