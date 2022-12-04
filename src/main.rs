mod challenges;
mod utils;

fn main() {
    challenges::one::find_elf_with_most_calories();
    challenges::one::find_top_three_with_most_calories();

    challenges::two::compute_rock_paper_scissors_score();
    challenges::three::compute_sum_of_priority_of_repeated_items();

    challenges::four::find_pairs_that_overlap();
}
