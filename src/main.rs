use std::fs;
use std::io;
use std::thread;
use std::sync::mpsc::sync_channel;
use std::time::{Instant};

fn main() -> io::Result<()> {

    let answer_words_file = String::from(".\\data\\words_answers.txt");
    let answer_words_string =
        fs::read_to_string(answer_words_file).expect("Should have been able to read the file");
    let answer_words_iter = answer_words_string.split("\n");
    let answer_words_vec: Vec<&str> = answer_words_iter.collect();
    let mut answer_words_arr: [[char; 5]; 2315] = [['7'; 5]; 2315];
    let mut counter: usize = 0;
    for x in answer_words_vec {
        if x.chars().count() == 6 {
            answer_words_arr[counter][0] = x.chars().nth(0).unwrap();
            answer_words_arr[counter][1] = x.chars().nth(1).unwrap();
            answer_words_arr[counter][2] = x.chars().nth(2).unwrap();
            answer_words_arr[counter][3] = x.chars().nth(3).unwrap();
            answer_words_arr[counter][4] = x.chars().nth(4).unwrap();
        }
        counter += 1;
    }
    run_game(answer_words_arr);
    Ok(())
}
fn print_instrunctions(){
    println!("\nInstructions: Enter Wordle's response to guess in 10 characters, where before");
    println!("each letter you indicate the color of the square, ' ' for blank '*' for yellow,");
    println!("and '=' for green (e.g.' t=e*s t s') All remaining possible words will appear. ");
    println!("Other options: 'r' gets best possible word. 'c' clears guesses. 'w' generates ");
    println!("Wordles response to a guess. 'b' removes most recent guess from guess list.");
    println!("'q' quits the program \n");

}
fn run_game(answer_words_arr: [[char; 5]; 2315]) {
    const TOTAL_THREADS:usize = 15;
    let mut keep_looping = true;
    let mut guess_list: Vec<[char; 10]> = Vec::new();
    let mut answer_words_vec: Vec<[char; 5]> = Vec::new();
    let (tx, rx) = sync_channel::<[u32; 2315]>(0);
    for x in answer_words_arr {
        answer_words_vec.push(x);
    }
    print_instrunctions();
    while keep_looping {
        print_guesses(&guess_list);
        println!("Enter word with tile colors or option (enter 'h' for help and options):");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();
        let mut guess_char: [char; 10] = ['7'; 10];
        let mut all_scores:[u32; 2315] = [0; 2315];
        if guess.len() > 9 {
            for n in 0..10 {
                guess_char[n] = guess.chars().nth(n).unwrap();
            }
            guess_list.push(guess_char);
            print_char5_array(&possible_words(&guess_list, answer_words_vec.clone()));
        } else if guess.chars().nth(0).unwrap() == 'r' {
            let start = Instant::now();
            let guess_list2 = guess_list.clone();
            let valid_words = possible_words(&guess_list, answer_words_vec.clone());

            let word_num = possible_words(&guess_list, answer_words_vec.clone()).len();
            if word_num > 40 {
                for x in 0..TOTAL_THREADS {
                    let guess_list_temp: Vec<[char; 10]> = guess_list2.clone();
                    let tx2 = tx.clone();
                    thread::spawn(move || {
                        let temp = get_best_word(
                            &guess_list_temp.clone(),
                            answer_words_arr,
                            x,
                            TOTAL_THREADS
                        );
                        tx2.send(temp).unwrap();
                    });
                }
                
                for _n in 0..TOTAL_THREADS {
                    let current_thread_array = rx.recv().unwrap();
                    for x in 0..2315{
                        all_scores[x] += current_thread_array[x];
                    }  
                }    
            } else {
                let guess_list_temp: Vec<[char; 10]> = guess_list2.clone();
                all_scores = get_best_word(
                    &guess_list_temp.clone(),
                    answer_words_arr,
                    0,
                    1
                );            
            }
            let mut lowest_number:u32 = 10000000;
            let mut lowest_number_valid:u32 = 10000000;
            for x in 0..2315{
                if all_scores[x] < lowest_number{
                    lowest_number = all_scores[x];
                }
                if valid_words.contains(&answer_words_arr[x]){
                    if all_scores[x] < lowest_number_valid{
                        lowest_number_valid = all_scores[x];
                    }
                }
            }
            let duration = start.elapsed();
            println!("Time elapsed\t{:?}", duration);
            for x in 0..2315{
                if all_scores[x] == lowest_number{
                    let lowest_word:String = answer_words_arr[x].iter().collect();
                    println!("Best Overall Word:\t{} ({})",lowest_word,all_scores[x]);
                }
                if valid_words.contains(&answer_words_arr[x]){
                    if all_scores[x] == lowest_number_valid{
                        let lowest_word:String = answer_words_arr[x].iter().collect();
                        println!("Best Valid Word:\t{} ({})",lowest_word,all_scores[x]);
                        lowest_number_valid = all_scores[x];
                    }
                }
            }
        } else if guess.chars().nth(0).unwrap() == 'q' {
            keep_looping = false;
        } else if guess.chars().nth(0).unwrap() == 'h' {
            print_instrunctions();
        } else if guess.chars().nth(0).unwrap() == 'b' {
            guess_list.pop();
            print_char5_array(&possible_words(&guess_list, answer_words_vec.clone()));
            print_guesses(&guess_list);
        } else if guess.chars().nth(0).unwrap() == 'c' {
            guess_list = Vec::new();} else if guess.chars().nth(0).unwrap() == 'w' {
            let mut guess = String::new();
            let mut answer = String::new();
            println!("See how Wordle would color tiles of a guess given an answer:\nEnter Guess: ");
            io::stdin().read_line(&mut guess).unwrap();
            println!("Enter Solution: ");
            io::stdin().read_line(&mut answer).unwrap();

            let mut guess_char: [char; 5] = ['7'; 5];
            let mut answer_char: [char; 5] = ['7'; 5];
            for n in 0..5 {
                guess_char[n] = guess.chars().nth(n).unwrap();
                answer_char[n] = answer.chars().nth(n).unwrap();
    
            }
            let char_response = get_game_response(guess_char, answer_char);
            let to_print: String = char_response.iter().collect();
            println!("{}", to_print);
        }
    }
}

fn get_best_word(
    
    guesses_vec: &Vec<[char; 10]>,
    answer_words_arr: [[char; 5]; 2315],
    this_part: usize,
    total_parts: usize,
) -> [u32; 2315] {
    let mut word_score_array: [u32; 2315] = [0; 2315];
    let mut word_index: usize = 0;
    let mut lowest_score_score: u32 = 429496700;
    let mut answer_words_vec: Vec<[char; 5]> = Vec::new();
    for x in answer_words_arr {
        answer_words_vec.push(x);
    }
    let possible_answers = possible_words(&guesses_vec, answer_words_vec);
    let thread_total_size: usize = possible_answers.len() as usize;
    let mut last_index: usize = (thread_total_size / total_parts) * (this_part + 1);
    if this_part == total_parts - 1 {
        last_index = thread_total_size;
    }
    let first_index: usize = (thread_total_size / total_parts) * this_part;
    let mut thread_possible_answers: Vec<[char; 5]> = Vec::new();
    for x in first_index..last_index {
        thread_possible_answers.push(possible_answers[x]);
    }
        for possible_guess in answer_words_arr{
            let mut guess_total: u32 = 0;

            for possible_answer in thread_possible_answers.clone() {
                let if_is_answer: [char; 10] = get_game_response(possible_guess, possible_answer);
                let mut the_guesses = guesses_vec.clone();
                the_guesses.push(if_is_answer);
                let total_words: i32 =
                    possible_words(&the_guesses, possible_answers.clone()).len() as i32;
                guess_total = guess_total + (total_words as u32);
            }
            if guess_total < lowest_score_score {
                lowest_score_score = guess_total;
            }
            word_score_array[word_index] = guess_total;
            word_index = word_index + 1;
        }

    return word_score_array;
} 

fn print_guesses(guesses_vec: &Vec<[char; 10]>) {
    if guesses_vec.len() > 0 {
        println!("Current guess list:");
        for a in guesses_vec {
            for b in a {
                print!("{}", b);
            }
            println!();
        }
        println!();
    }
}

fn get_game_response(guess: [char; 5], answer: [char; 5]) -> [char; 10] {
    let mut char_ret: [char; 10] = [' '; 10];
    let mut unfound_letters: [char; 5] = [' '; 5];
    //1. In char where guess and answer match add = in front of letter in ret
    //2. Put the char of guess in answer spot at 2*index+1
    for place in 0..5 {
        let current_char = guess[place];
        if guess[place] == answer[place] {
            char_ret[place * 2] = '=';
        } else {
            unfound_letters[place] = answer[place];
        }
        char_ret[(place * 2) + 1] = current_char;
    }
    for outi in 0..5 {
        let guess_char = guess[outi];
        for ini in 0..5 {
            let unfound_char = unfound_letters[ini];
            if guess_char == unfound_char {
                unfound_letters[ini] = ' ';
                if char_ret[outi * 2] != '=' {
                    char_ret[outi * 2] = '*';
                }
            }
        }
    }

    return char_ret;
}

fn print_char5_array(to_print: &Vec<[char; 5]>) {
    let mut counter = 0;
    println!("");
    for x in to_print {
        let s2: String = x.iter().collect();
        if counter % 10 == 0 {
            print!("{}", s2);
        } else {
            print!(", {}", s2);
        }

        if counter % 10 == 9 {
            println!()
        }
        counter += 1;
    }
    println!("\nTotal words: {}", to_print.len());
    println!("");
}

fn possible_words(
    guesses_vec: &Vec<[char; 10]>,
    answer_words_arr: Vec<[char; 5]>,
) -> Vec<[char; 5]> {
    let mut ret_vec: Vec<[char; 5]> = Vec::new();
    let mut equals_spots: [char; 5] = [' '; 5];
    let mut in_somewhere: Vec<char> = Vec::new();
    let mut in_somewhere_not_found: Vec<char> = Vec::new();
    let mut def_not_in: Vec<char> = Vec::new();

    //Make sure a yellow char isn't in same position later
    let mut not_in_spot_0:Vec<char> = Vec::new();
    let mut not_in_spot_1:Vec<char> = Vec::new();
    let mut not_in_spot_2:Vec<char> = Vec::new();
    let mut not_in_spot_3:Vec<char> = Vec::new();
    let mut not_in_spot_4:Vec<char> = Vec::new();


    //Make an array where each = char has real letter in space
    for line in guesses_vec.clone() {
        for place in 0..5 {
            if line[place * 2] == '=' {
                equals_spots[place] = line[(place * 2) + 1];
                in_somewhere.push(line[(place * 2) + 1]);
            }
            if line[place * 2] == ' ' {
                def_not_in.push(line[(place * 2) + 1]);
            }
            if line[place * 2] == '*' {
                in_somewhere_not_found.push(line[(place * 2) + 1]);
            }
            if line[0] == '*'{
                not_in_spot_0.push(line[1]);
            }
            if line[2] == '*'{
                not_in_spot_1.push(line[3]);
            }
            if line[4] == '*'{
                not_in_spot_2.push(line[5]);
            }
            if line[6] == '*'{
                not_in_spot_3.push(line[7]);
            }
            if line[8] == '*'{
                not_in_spot_4.push(line[9]);
            }
        }
    }

    for i in 0..answer_words_arr.len() {
        let mut add_word = true;
        let current_word = answer_words_arr[i];
        if not_in_spot_0.contains(&current_word[0]){
            add_word = false;
        }
        if not_in_spot_1.contains(&current_word[1]){
            add_word = false;
        }
        if not_in_spot_2.contains(&current_word[2]){
            add_word = false;
        }
        if not_in_spot_3.contains(&current_word[3]){
            add_word = false;
        }
        if not_in_spot_4.contains(&current_word[4]){
            add_word = false;
        }
        'done: for find_equal in 0..5 {
            if equals_spots[find_equal] != ' ' {
                if equals_spots[find_equal] != current_word[find_equal] {
                    add_word = false;
                    break 'done;
                }
            }


            if def_not_in.contains(&current_word[find_equal]) {
                if !in_somewhere.contains(&current_word[find_equal]) {
                    if !in_somewhere_not_found.contains(&current_word[find_equal]) {
                        add_word = false;
                        break 'done;
                    }
                }
            }
        }
        if add_word {
            for x in in_somewhere_not_found.clone() {
                if !current_word.contains(&x) {
                    add_word = false;
                }
            }
            if add_word {
                ret_vec.push(current_word);
            }
        }
    }
    return ret_vec;
}