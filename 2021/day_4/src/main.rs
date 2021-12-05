use std::env;
use util::input_operations::{read_file_to_string,split_double_newlines,split_lines,split_whitespace};

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

const HORIZONTAL_BINGO_VECTOR: [usize; 5] = [0, 1, 2, 3, 4]; // TODO: create scalar vector
const VERTICAL_BINGO_VECTOR: [usize; 5] = [0, 5, 10, 15, 20];
// const DIAGONAL_BINGO_TOPRIGHT_BOTTOMLEFT: [usize; 5]  = [4, 8, 12, 16, 20];
// const DIAGONAL_BINGO_TOPLEFT_BOTTOMRIGHT: [usize; 5] = [0, 6, 12, 18, 24];

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

    fn check_bingo(&self) -> Option<(usize, usize)> {
        let mut bingo_result: Option<(usize, usize)> = None;
        for (card_index, card) in self.cards().enumerate() {
            bingo_result = match card.check_bingo() {
                Some(bingo) => Some((bingo, card_index)),
                None => bingo_result
            }
        }
        bingo_result
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

    fn check_bingo(&self) -> Option<Vec<usize>> {
        let mut bingo_results: Vec<usize> = vec![];
        for i in 0..self.card_width {
            let mut has_bingo: bool = true;
            HORIZONTAL_BINGO_VECTOR.iter().for_each(|item| has_bingo &= self.crossed_number_positions.contains(&(item + (&i * self.card_width))));
            if has_bingo {
                println!("horizontal bingo!");
                bingo_results.push(self.compute_sum());
            }
        }
        for i in 0..self.card_height {
            let mut has_bingo: bool = true;
            VERTICAL_BINGO_VECTOR.iter().for_each(|item| has_bingo &= self.crossed_number_positions.contains(&(item + &i)));
            if has_bingo {
                println!("vertical bingo!");
                bingo_results.push(self.compute_sum());
            }
        }
        if bingo_results.iter().count() > 0 {
            return Some(bingo_results);
        }
        // let mut has_bingo: bool = true;
        // DIAGONAL_BINGO_TOPLEFT_BOTTOMRIGHT.iter().for_each(|item| has_bingo &= self.crossed_number_positions.contains(item));
        // if has_bingo {
        //     println!("diagonal bingo (topleft bottomright)!");
        //     return has_bingo;
        // }
        // has_bingo = true;
        // DIAGONAL_BINGO_TOPRIGHT_BOTTOMLEFT.iter().for_each(|item| has_bingo &= self.crossed_number_positions.contains(item));
        // if has_bingo {
        //     println!("diagonal bingo (topright bottomleft)!");
        //     return has_bingo;
        // }
        return None
        // TODO: check for bingo
        // If 5 subsequent numbers are crossed
        // If the diagonals are crossed
        // If 5 vertical subsequent numbers are crossed
        
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
    
    println!("{:?}", numbers);
    println!("{:?}", bingo_game.cards);
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
        if let Some((bingo_sum,_)) = bingo_game.check_bingo() {
            println!("bingo! Sum is {}", bingo_sum);
            println!("bingo! bingo is {}", bingo_sum*lucky_number);
            break;
        }
    }
}

fn consolation_price(numbers: &Vec<usize>, bingo_game: &mut BingoGame) {
    let mut last_bingo: usize = 0;
    for lucky_number in numbers {
        println!("Drawing number {}", lucky_number);
        if bingo_game.cards().count() == 0 {
            break;
        }
        bingo_game.cross_number(&lucky_number);
        if let Some((bingo_sum, card_index)) = bingo_game.check_bingo() {
            println!("bingo! Sum is {}", bingo_sum);
            println!("bingo! Bingo is {}", bingo_sum*lucky_number);
            last_bingo = bingo_sum*lucky_number;
            bingo_game.remove_card(card_index);
            println!("{} cards left", bingo_game.cards().count());
        }
    }
    println!("consolation price: {}", last_bingo);
}