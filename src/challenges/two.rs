use crate::utils::utils::*;

pub fn compute_rock_paper_scissors_score() {
    let file_path = "src/challenges/two_input.txt";
    let contents = read_file(file_path);
    let rounds: Vec<&str> = contents.split("\n").collect();

    let mut score = 0;
    let mut alternative_score = 0;

    for round in rounds {
        let opponent = &round[0..1];
        let response = &round[2..];
        score += resolve_rock_paper_scissors(opponent, response);
        alternative_score += resolve_rock_paper_scissors_alternative(opponent, response);
    }

    println!(
        "The total score is = {}, and the alternative score is {}",
        score, alternative_score
    );
}
