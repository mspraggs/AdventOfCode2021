use std::collections::HashMap;
use std::env;
use std::error;
use std::fs;
use std::hash::Hash;

use aoc2021::error::Error;

fn parse_input(data: &str) -> Result<Vec<Player>, Box<dyn error::Error>> {
    data.lines()
        .map(|l| {
            l.split(": ")
                .nth(1)
                .and_then(|s| s.parse::<usize>().ok())
                .map(Player::new)
                .ok_or_else(|| -> Box<dyn error::Error> {
                    Box::new(Error("Unable to parse integer.".to_string()))
                })
        })
        .collect::<Result<Vec<_>, _>>()
}

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Player {
    position: usize,
    score: usize,
}

impl Player {
    fn new(position: usize) -> Self {
        Self { position, score: 0 }
    }

    fn add_to_position(&mut self, increase: usize) {
        self.position += increase;
        while self.position > 10 {
            self.position -= 10;
        }
        self.score += self.position;
    }
}

trait Dice {
    fn roll(&mut self) -> Vec<usize>;
}

#[derive(Debug, Clone, Copy)]
struct PracticeDice {
    current: usize,
}

impl PracticeDice {
    fn new() -> Self {
        Default::default()
    }
}

impl Dice for PracticeDice {
    fn roll(&mut self) -> Vec<usize> {
        let mut sum = 0;
        for _ in 0..3 {
            sum += self.current;
            self.current += 1;
            if self.current > 100 {
                self.current = 1;
            }
        }
        vec![sum]
    }
}

impl Default for PracticeDice {
    fn default() -> Self {
        Self { current: 1 }
    }
}

#[derive(Debug, Default, Clone)]
struct DiracDice {
    rolls: Vec<usize>,
}

impl DiracDice {
    fn new() -> Self {
        let mut rolls = Vec::with_capacity(27);
        for i in 1..4 {
            for j in 1..4 {
                for k in 1..4 {
                    rolls.push(i + j + k);
                }
            }
        }
        Self { rolls }
    }
}

impl Dice for DiracDice {
    fn roll(&mut self) -> Vec<usize> {
        self.rolls.clone()
    }
}

trait Game {
    fn play(&mut self, players: &mut [Player]);
}

#[derive(Debug, Default, Clone, Copy)]
struct PracticeGame {
    dice: PracticeDice,
    roll_count: usize,
}

impl PracticeGame {
    fn new(dice: PracticeDice) -> Self {
        Self {
            dice,
            roll_count: 0,
        }
    }
}

impl Game for PracticeGame {
    fn play(&mut self, players: &mut [Player]) {
        'game_loop: loop {
            for player in players.iter_mut() {
                let dice_rolls = self.dice.roll();
                self.roll_count += 3;
                let roll: usize = dice_rolls[0];
                player.add_to_position(roll);

                if player.score >= 1000 {
                    break 'game_loop;
                }
            }
        }
    }
}

fn practice(players: &mut [Player]) -> Result<usize, Box<dyn error::Error>> {
    if players.is_empty() {
        return Err(Box::new(Error(
            "Unable to play without players.".to_owned(),
        )));
    }

    let mut game = PracticeGame::new(PracticeDice::new());
    game.play(players);

    let min_score = players.iter().map(|p| p.score).min().unwrap();

    Ok(min_score * game.roll_count)
}

#[derive(Debug, Default, Clone)]
struct DiracGame {
    rolls: Vec<usize>,
    win_counts: Vec<usize>,
    result_cache: HashMap<(Vec<Player>, usize), Vec<usize>>,
    game_count: usize,
}

impl DiracGame {
    fn new(mut dice: DiracDice, num_players: usize) -> Self {
        Self {
            rolls: dice.roll(),
            win_counts: vec![0; num_players],
            result_cache: HashMap::new(),
            game_count: 0,
        }
    }

    fn play_recursive(&mut self, players: &[Player], turn: usize) -> Vec<usize> {
        let turn = if turn >= players.len() { 0 } else { turn };
        let cache_key = (players.to_owned(), turn);
        if let Some(result) = self.result_cache.get(&cache_key) {
            return result.clone();
        }

        let mut win_counts = vec![0; players.len()];

        for i in 0..self.rolls.len() {
            let mut new_players = players.to_owned();

            let current_player = &mut new_players[turn];
            current_player.add_to_position(self.rolls[i]);

            if current_player.score >= 21 {
                win_counts[turn] += 1;
            } else {
                let counts = self.play_recursive(&new_players, turn + 1);
                for (i, c) in counts.iter().enumerate() {
                    win_counts[i] += *c;
                }
            }
        }

        self.result_cache.insert(cache_key, win_counts.clone());

        win_counts
    }
}

impl Game for DiracGame {
    fn play(&mut self, players: &mut [Player]) {
        self.win_counts = self.play_recursive(players, 0);
    }
}

fn play(players: &mut [Player]) -> Result<usize, Box<dyn error::Error>> {
    if players.is_empty() {
        return Err(Box::new(Error(
            "Unable to play without players.".to_owned(),
        )));
    }

    let mut game = DiracGame::new(DiracDice::new(), players.len());
    game.play(players);

    Ok(game.win_counts.iter().max().copied().unwrap())
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(Box::new(Error(format!(
            "Usage: {} <input data path>",
            args[0]
        ))));
    }

    let path = &args[1];
    let file_contents =
        fs::read_to_string(path).map_err(|e| format!("Error reading input data: {}.", e))?;

    let mut players = parse_input(&file_contents)?;

    let result = practice(&mut players.clone())?;
    println!("Part one: {}", result);

    let result = play(&mut players)?;
    println!("Part two: {}", result);

    Ok(())
}
