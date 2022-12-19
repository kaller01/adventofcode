use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("File should exists");

    let hands_for_rounds = parse_input::<Hand>(&input,parse_hand);
    // println!("{:?}",hands_for_rounds);
    let points = calculate_score(hands_for_rounds);
    println!("{points}");

    let rounds = parse_input::<Round>(&input,parse_outcome);
    let mut hands_for_rounds: Vec<(Hand,Hand)> = Vec::new();

    for (opponent_hand, outcome) in rounds {
        let my_hand = cheat_round(&opponent_hand,&outcome);
        hands_for_rounds.push((opponent_hand, my_hand)); 
    }
    // println!("{:?}",hands_for_rounds);
    let points = calculate_score(hands_for_rounds);
    println!("{points}");
    

}

#[derive(Debug)]
enum Hand {
    Rock,
    Paper,
    Scissor
}

#[derive(Debug)]
enum Round {
    Draw,
    Win,
    Loose
}

fn cheat_round(opponent: &Hand, outcome: &Round) -> Hand {
    return match outcome {
        Round::Draw => match opponent {
            Hand::Rock => Hand::Rock,
            Hand::Paper => Hand::Paper,
            Hand::Scissor => Hand::Scissor
        },
        Round::Win => match opponent {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissor,
            Hand::Scissor => Hand::Rock
        },
        Round::Loose => match opponent {
            Hand::Rock => Hand::Scissor,
            Hand::Paper => Hand::Rock,
            Hand::Scissor => Hand::Paper
        }
    }
}

fn calculate_score(rounds: Vec<(Hand,Hand)>) -> u32 {
    let mut points = 0;
    for (my_hand, opponent_hand) in rounds {
        let round = play_round(&opponent_hand,&my_hand);
        points += match my_hand {
            Hand::Rock=>1,
            Hand::Paper=>2,
            Hand::Scissor=>3
        };
        points += match round {
            Round::Loose=>0,
            Round::Draw=>3,
            Round::Win=>6
        };
    }
    return points;
}

fn play_round(opponent: &Hand, me: &Hand) -> Round{
    return match opponent {
        Hand::Rock=> match me {
            Hand::Rock=>Round::Draw,
            Hand::Paper=>Round::Win,
            Hand::Scissor=>Round::Loose
        },
        Hand::Paper=> match me {
            Hand::Rock=>Round::Loose,
            Hand::Paper=>Round::Draw,
            Hand::Scissor=>Round::Win
        },
        Hand::Scissor=> match me {
            Hand::Rock=>Round::Win,
            Hand::Paper=>Round::Loose,
            Hand::Scissor=>Round::Draw
        },
    };
}

fn parse_input<T>(input: &str, parse_action: fn(&str) -> T) -> Vec<(Hand,T)> {
    let rounds = input.split("\n");
    let mut rounds_hands: Vec<(Hand,T)> = Vec::new();

    for round in rounds {
        let hands_input: Vec<&str> = round.split(" ").collect(); // Split returns an iterable, it has to be collected or loop through (better for performance if big values)
        let opponent_hand = parse_hand(hands_input[0]);
        let my_action = parse_action(hands_input[1]);
        rounds_hands.push((opponent_hand,my_action));
    }
    return rounds_hands;
}

fn parse_outcome(input_action: &str) -> Round {
    return match input_action.trim() {
        "X"=>Round::Loose,
        "Y"=>Round::Draw,
        "Z"=>Round::Win,
        _=>unreachable!("Invalid input")
    }
}

fn parse_hand(input_hand: &str) -> Hand {
    return match input_hand.trim() {
        "A"=>Hand::Rock,
        "B"=>Hand::Paper,
        "C"=>Hand::Scissor,
        "X"=>Hand::Rock,
        "Y"=>Hand::Paper,
        "Z"=>Hand::Scissor,
        _=>unreachable!("Invalid input")
    };
}