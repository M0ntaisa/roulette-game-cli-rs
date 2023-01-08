use rand::{seq::SliceRandom, Rng};

fn get_user_bet() -> i32 {
    println!("Enter your bet:");
    let mut bet = String::new();
    std::io::stdin().read_line(&mut bet).expect("Failed to read line");
    let bet: i32 = match bet.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        println!("Invalid input. Please enter a number.");
        get_user_bet()
      }
    };
    bet
}

fn get_scrap_bet() -> i32 {
    println!("Enter scrap:");
    let mut scrap = String::new();
    std::io::stdin().read_line(&mut scrap).expect("Failed to read line");
    let scrap: i32 = match scrap.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a number.");
            get_scrap_bet()
        }
    };
    scrap
}

fn shuffle(roulette_wheel: &mut [i32]) {
    let mut rng = rand::thread_rng();
    roulette_wheel.shuffle(&mut rng);
}
  
fn random_number_between(min: usize, max: usize) -> usize {
    rand::thread_rng().gen_range(min..max)
}

fn pay_out(amount: i32) {
    println!("You won {} Scraps!", amount);
}

fn main() {
    println!("Welcome to the roulette table!");

    let mut roulette_wheel: &mut [i32] = &mut [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 3, 3, 3, 3, 3, 5, 5, 5, 5, 10, 10, 20];
    let mut scrap: i32 = 1000;

    loop {
        // shuffle the numbers in the roulette_wheel array
        shuffle(&mut roulette_wheel);

        // let user put how much scrap for the bet
        let scrap_bet = get_scrap_bet();

        // let the user place their bet
        let bet = get_user_bet();

        // spin the roulette wheel
        let spun_number = roulette_wheel[random_number_between(0, roulette_wheel.len() - 1)];
        println!("{}", &spun_number);
        // check if the user won
        if spun_number == bet {
            // multiply the user's bet by the number they bet on
            let winnings = bet * roulette_wheel[bet as usize];
            // pay out the winnings to the user
            pay_out(winnings);
            break;
        } else {
            // the user lost, so don't pay out anything
            println!("Sorry, you lost. Better luck next time!")
        }
    }
}

// TODO : scrap bet still not working, make it work!