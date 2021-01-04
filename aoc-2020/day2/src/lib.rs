pub fn solve_part_1(input: &String) -> String {
    input
        .trim()
        .split('\n')
        .map(parse_password)
        .filter(Password::is_valid)
        .count()
        .to_string()
}

fn parse_password(text: &str) -> Password {
    let text: Vec<&str> = text.split(": ").collect();
    let policy_text: Vec<&str> = text[0].split(|c| c == '-' || c == ' ').collect();
    Password {
        policy: Policy {
            letter: policy_text[2].chars().next().unwrap(),
            min: policy_text[0].parse::<usize>().unwrap(),
            max: policy_text[1].parse::<usize>().unwrap(),
        },
        password: text[1],
    }
}

struct Password<'a> {
    policy: Policy,
    password: &'a str,
}

struct Policy {
    letter: char,
    min: usize,
    max: usize,
}

impl Password<'_> {
    fn is_valid(&self) -> bool {
        let occurences = self.password.chars().filter(|c| *c == self.policy.letter).count();
        occurences >= self.policy.min && occurences <= self.policy.max
    }
}

pub fn solve_part_2(input: &String) -> String {
    String::from("not yet implemented")
}
