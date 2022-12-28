use rand::Rng;

fn main() {
    println!("Welcome to the roulette table!");

    // Generate a random number between 0 and 36
    let winning_number = rand::thread_rng().gen_range(0..37);

    println!("The winning number is: {}", winning_number);

    println!("Enter your bet (1-36 for a specific number, 37 for red, 38 for black):");

    let mut bet = String::new();
    std::io::stdin().read_line(&mut bet).expect("Failed to read line");

    let bet: u32 = bet.trim().parse().expect("Please enter a valid number");

    // Check if the bet is a specific number, red, or black
    if bet == winning_number {
        println!("Congratulations, you win!");
    } else if bet == 37 {
        if winning_number % 2 == 0 {
            println!("Sorry, you lose. The winning number was black.");
        } else {
            println!("Congratulations, you win! The winning number was red.");
        }
    } else if bet == 38 {
        if winning_number % 2 == 0 {
            println!("Congratulations, you win! The winning number was black.");
        } else {
            println!("Sorry, you lose. The winning number was red.");
        }
    } else {
        println!("Sorry, you lose. The winning number was {}.", winning_number);
    }
}

// TODO : Add scrap bet feature