pub fn solve_part_1(input: &String) -> String {
    let expenses = parse_expenses(input);

    multiply_two_entries_that_sum_to(2020, &expenses[..])
        .unwrap()
        .to_string()
}

fn parse_expenses(input: &String) -> Vec<i32> {
    input
        .trim()
        .split('\n')
        .map(|line| line.parse::<i32>().unwrap())
        .collect()
}

fn multiply_two_entries_that_sum_to(sum: i32, expenses: &[i32]) -> Result<i32, &'static str> {
    for (first_index, first) in expenses.iter().enumerate() {
        for second in &expenses[first_index + 1..] {
            if first + second == sum {
                return Ok(first * second);
            }
        }
    }
    return Err("no solution found");
}

pub fn solve_part_2(input: &String) -> String {
    let expenses = parse_expenses(input);

    for (first_index, first) in expenses.iter().enumerate() {
        match multiply_two_entries_that_sum_to(2020 - first, &expenses[first_index + 1..]) {
            Ok(product_of_the_other_two) => return (first * product_of_the_other_two).to_string(),
            _ => (),
        }
    }
    panic!("no solution found");
}
