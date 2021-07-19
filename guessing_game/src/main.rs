
fn main() {
    let secret_number = guessing_game::generate_random_number(1, 101);
    loop {
        let guess = guessing_game::prompt_user_for_guess();
        let user_won_game = guessing_game::user_won_game(&guess, &secret_number);
        if user_won_game {
            break;
        }
    }
}
