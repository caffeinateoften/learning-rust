#[test]
fn generate_random_number_returns_different_numbers() {
    let a = guessing_game::generate_random_number(1, 999999);
    let b = guessing_game::generate_random_number(1, 999999);
    assert_ne!(a, b);
}

#[test]
fn user_wins_game_if_guesses_exact_number() {
    let result = guessing_game::user_won_game(&3, &3);
    assert!(result);
}

#[test]
fn user_doesnt_win_if_guess_is_low() {
    let result = guessing_game::user_won_game(&2, &3);
    assert!(!result);
}

#[test]
fn user_doesnt_win_if_guess_is_high() {
    let result = guessing_game::user_won_game(&5, &3);
    assert!(!result);
}
