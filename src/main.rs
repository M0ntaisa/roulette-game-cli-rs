mod funcs;

use funcs::{get_user_bet, get_scrap_bet, shuffle, random_number_between, pay_out, play_again};

fn main() {
    println!("Welcome to the roulette table!");

    let mut roulette_wheel: &mut [i32] = &mut [1, 3, 1, 5, 1, 3, 1, 10, 1, 3, 1, 5, 1, 5, 3, 1, 10, 1, 3, 1, 5, 1, 3, 1, 20];
    let mut scrap: i32 = 1000;
    let mut keep_playing = true;

    while keep_playing {
        // print scrap amount
        println!("You have {} Scraps", &scrap);

        // shuffle the numbers in the roulette_wheel array
        shuffle(&mut roulette_wheel);

        // let user put how much scrap for the bet
        let scrap_bet = get_scrap_bet(&mut scrap);
        println!("{:#?}", &scrap_bet);

        // let the user place their bet
        let bet = get_user_bet();

        // spin the roulette wheel
        let spun_number = roulette_wheel[random_number_between(0, roulette_wheel.len() - 1)];
        println!("spun number is {}", &spun_number);
        // check if the user won
        if spun_number == bet {
            // multiply the user's bet by the number they bet on
            match scrap_bet {
                Ok(s) => {
                    let winnings = s * spun_number + s;
                    // pay out the winnings to the user
                    pay_out(winnings, &mut scrap);
                    play_again(&mut keep_playing);
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
            
        } else {
            // the user lost, so don't pay out anything
            println!("Sorry, you lost. Better luck next time!");
            play_again(&mut keep_playing);
        }

    }
}

// TODO : make and option if the game is over, wanna play again or not