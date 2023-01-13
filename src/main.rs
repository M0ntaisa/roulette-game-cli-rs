use rand::{seq::SliceRandom, Rng};

// TODO :: change this to option input
fn get_user_bet() -> i32 {
    println!("Enter your wheel bet [1, 3, 5, 10, 20]:");
    let mut bet = String::new();
    std::io::stdin().read_line(&mut bet).expect("Failed to read line");
    let bet_option = bet.trim();
    let bet: i32 = match bet_option {
        "1" | "3" | "5" | "10" | "20" => bet_option.parse().unwrap(),
        _ => {
            println!("Invalid input. Please enter a valid option.");
            get_user_bet()
        }
    };
    bet
}

fn get_scrap_bet(scrap: &mut i32) -> Result<i32, String> {
    println!("Enter scrap:");
    let mut scrap_in = String::new();
    std::io::stdin().read_line(&mut scrap_in).expect("Failed to read line");
    let scrap_in: i32 = match scrap_in.trim().parse() {
        Ok(num) => num,
        Err(_) => return Err("Invalid input, please enter a number.".to_string())
    };
    
    // check if player have enough scrap to bet
    if scrap_in > *scrap {
        return Err("You don't have enought scrap to bet!".to_string());
    }
    *scrap = *scrap - scrap_in;
    println!("Your current scrap is: {}", scrap);
    println!("You bet {} scrap", scrap_in);
    Ok(scrap_in)
}

fn shuffle(roulette_wheel: &mut [i32]) {
    let mut rng = rand::thread_rng();
    roulette_wheel.shuffle(&mut rng);
}
  
fn random_number_between(min: usize, max: usize) -> usize {
    rand::thread_rng().gen_range(min..max)
}

fn pay_out(amount: i32, scrap: &mut i32) {
    println!("You won {} Scraps!", &amount);
    *scrap = amount + *scrap;
    println!("Your current scrap is: {}", scrap);
}

fn play_again(keep_playing: &mut bool) {
    println!("Do you want to continue playaing? (y/n)");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to readline");

    match input.trim() {
        "n" => {
            println!("Thanks for playing");
            *keep_playing = false
        },
        "y" => *keep_playing = true,
        _ => {
            println!("Invalid input, Please enter 'y' or 'n'.");
            *keep_playing = true;
        }
    }
}

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