use std::env;
use util::input_operations::{read_file_to_string,split_double_newlines,split_lines};

#[derive(Debug,Clone)]
struct BingoCard {
    card: Vec<usize>,
    crossed_number_positions: Vec<usize>,
    card_width: usize,
    card_height: usize
}

#[derive(Debug,Clone)]
struct BingoGame {
    cards: Vec<BingoCard>
}

const HORIZONTAL_BINGO_VECTOR: [usize; 5] = [0, 1, 2, 3, 4];
const VERTICAL_BINGO_VECTOR: [usize; 5] = [0, 5, 10, 15, 20];

impl BingoGame {
    fn cards(&self) -> std::slice::Iter<'_, BingoCard> {
        self.cards.iter()
    }

    fn remove_card(&mut self, card_index: usize) {
        &mut self.cards.remove(card_index);
    }


    fn cross_number(&mut self, number: &usize) {
        for card in &mut self.cards {
            card.cross_number(number);
        }
    }

    fn check_bingo(&self) -> Vec<(usize, usize)> {
        let mut bingo_result: Vec<(usize, usize)> = vec![];
        for (card_index, card) in self.cards().enumerate() {
            if let Some(bingo) = card.check_bingo() {
                bingo_result.push((bingo, card_index));
            }
        }
        bingo_result.sort_by(|a, b| b.1.cmp(&a.1));
        return bingo_result;
    }
}

impl BingoCard {
    fn card(&self) -> std::slice::Iter<'_, usize> {
        self.card.iter()
    }

    fn crossed_number_positions(&self) -> std::slice::Iter<'_, usize> {
        self.crossed_number_positions.iter()
    }

    fn cross_number(&mut self, number: &usize) {
        let position = self.card().position(|x| x == number);
        if let Some(hit) = position {
            self.crossed_number_positions.push(hit); 
        }
    }

    fn compute_sum(&self) -> usize {
        let mut sum: usize = 0;
        for i in self.crossed_number_positions() {
            sum += self.card[*i];
        }
        sum = self.card().sum::<usize>() - sum;
        return sum;
    }

    fn check_bingo(&self) -> Option<usize> {
        for i in 0..self.card_width {
            let mut has_bingo: bool = true;
            HORIZONTAL_BINGO_VECTOR.iter().for_each(|item| has_bingo &= self.crossed_number_positions.contains(&(item + (&i * self.card_width))));
            if has_bingo {
                return Some(self.compute_sum());
            }
        }
        for i in 0..self.card_height {
            let mut has_bingo: bool = true;
            VERTICAL_BINGO_VECTOR.iter().for_each(|item| has_bingo &= self.crossed_number_positions.contains(&(item + &i)));
            if has_bingo {
                return Some(self.compute_sum());
            }
        }
        return None
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Expected a filename as argument");
    }
    let input = str::replace(&read_file_to_string(&args[1]), "  ", " ");
    let input = split_double_newlines(&input);

    let (mut bingo_game, numbers) = collect_bingo_cards(input);
    
    let mut bingo_copy = bingo_game.clone();
    play_bingo(&numbers, &mut bingo_game);
    consolation_price(&numbers, &mut bingo_copy);
}

fn collect_bingo_cards(mut input: std::iter::Peekable<std::str::Split<&str>>) -> (BingoGame, Vec<usize>) {
    let numbers: Vec<usize> = input.next().unwrap().split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let mut bingo_cards: Vec<BingoCard> = vec![];
    while input.peek().is_some() {
        let bingo_card = input.next().unwrap();
        let bingo_card: Vec<usize> = split_lines(&bingo_card.to_string())
            .map(|x: &str| x.trim().split_whitespace().map(|y| y.to_string()).collect::<Vec<String>>())
            .flat_map(|x| x.iter().map(|y| y.parse::<usize>().unwrap()).collect::<Vec<usize>>())
            .collect();
        bingo_cards.push(BingoCard {
            card: bingo_card,
            crossed_number_positions: vec![],
            card_width: 5,
            card_height: 5
        });
    }
    (BingoGame {
        cards: bingo_cards
    }, numbers)
}


fn play_bingo(numbers: &Vec<usize>, bingo_game: &mut BingoGame) {
    for lucky_number in numbers {
        bingo_game.cross_number(&lucky_number);
        for correct_bingo in bingo_game.check_bingo() {
            println!("bingo! bingo is {}", correct_bingo.0*lucky_number);
            return;
        }
    }
}

fn consolation_price(numbers: &Vec<usize>, bingo_game: &mut BingoGame) {
    let mut last_bingo: usize = 0;
    for lucky_number in numbers {
        if bingo_game.cards().count() == 0 {
            break;
        }
        bingo_game.cross_number(&lucky_number);
        for correct_bingo in bingo_game.check_bingo() {
            let bingo_sum = correct_bingo.0;
            last_bingo = bingo_sum*lucky_number;
            bingo_game.remove_card(correct_bingo.1);
        }
    }
    println!("consolation price: {}", last_bingo);
}