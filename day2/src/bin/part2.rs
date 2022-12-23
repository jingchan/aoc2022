use std::{io, ops::Index};

#[derive(Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

fn main() -> io::Result<()> {
    let mut my_score = 0;
    for line in io::stdin().lines() {
        let line = line?;
        let mut moves = line.split_ascii_whitespace();
        let opponent_move = parse_move(moves.next().unwrap());
        let intended_result = moves.next().unwrap();
        // println!("{:?} {:?}", opponent_move, my_move);

        let my_move = match intended_result {
            "X" => move_for_lose(&opponent_move),
            "Y" => move_for_draw(&opponent_move),
            "Z" => move_for_win(&opponent_move),
            _ => panic!(),
        };

        match my_move {
            Move::Rock => my_score += 1,
            Move::Paper => my_score += 2,
            Move::Scissors => my_score += 3,
        }

        match opponent_move {
            Move::Rock => match my_move {
                Move::Rock => my_score += 3,
                Move::Paper => my_score += 6,
                Move::Scissors => my_score += 0,
            },
            Move::Paper => match my_move {
                Move::Rock => my_score += 0,
                Move::Paper => my_score += 3,
                Move::Scissors => my_score += 6,
            },
            Move::Scissors => match my_move {
                Move::Rock => my_score += 6,
                Move::Paper => my_score += 0,
                Move::Scissors => my_score += 3,
            },
        }
    }
    println!("{}", my_score);

    Ok(())
}

fn parse_move(move_str: &str) -> Move {
    match move_str {
        "A" | "X" => Move::Rock,
        "B" | "Y" => Move::Paper,
        "C" | "Z" => Move::Scissors,
        _ => panic!(),
    }
}

fn move_for_draw(m: &Move) -> Move {
    match m {
        Move::Rock => Move::Rock,
        Move::Paper => Move::Paper,
        Move::Scissors => Move::Scissors,
    }
}

fn move_for_win(m: &Move) -> Move {
    match m {
        Move::Rock => Move::Paper,
        Move::Paper => Move::Scissors,
        Move::Scissors => Move::Rock,
    }
}

fn move_for_lose(m: &Move) -> Move {
    match m {
        Move::Rock => Move::Scissors,
        Move::Paper => Move::Rock,
        Move::Scissors => Move::Paper,
    }
}
