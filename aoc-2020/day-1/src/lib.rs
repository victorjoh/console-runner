pub fn solve_puzzle(input: String) -> String {
    let expenses: Vec<i32> = input
        .trim()
        .split('\n')
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    for (this_index, this) in expenses.iter().enumerate() {
        for other_index in this_index+1..expenses.len() {
            let other = expenses.get(other_index).unwrap();
            if *this + other == 2020 {
                return (*this * other).to_string();
            }
        }
    }
    panic!("no solution found");
}
