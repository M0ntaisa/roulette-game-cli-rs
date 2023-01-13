
use rand::{seq::SliceRandom, Rng};

// TODO :: change this to option input
pub fn get_user_bet() -> i32 {
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

pub fn get_scrap_bet(scrap: &mut i32) -> Result<i32, String> {
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

pub fn shuffle(roulette_wheel: &mut [i32]) {
  let mut rng = rand::thread_rng();
  roulette_wheel.shuffle(&mut rng);
}

pub fn random_number_between(min: usize, max: usize) -> usize {
  rand::thread_rng().gen_range(min..max)
}

pub fn pay_out(amount: i32, scrap: &mut i32) {
  println!("You won {} Scraps!", &amount);
  *scrap = amount + *scrap;
  println!("Your current scrap is: {}", scrap);
}

pub fn play_again(keep_playing: &mut bool) {
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