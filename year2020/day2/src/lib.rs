pub fn solve_part_1(input: &String) -> String {
    get_nbr_of_valid_passwords(input, |p| p.is_valid_by_occurences())
}

pub fn solve_part_2(input: &String) -> String {
    get_nbr_of_valid_passwords(input, |p| p.is_valid_by_locations())
}

fn get_nbr_of_valid_passwords<P>(input: &String, policy_interpretation: P) -> String
where
    P: Fn(&Password) -> bool,
{
    input
        .trim()
        .split('\n')
        .map(parse_password)
        .filter(policy_interpretation)
        .count()
        .to_string()
}

fn parse_password(text: &str) -> Password {
    let text: Vec<&str> = text.split(": ").collect();
    let policy_text: Vec<&str> = text[0].split(|c| c == '-' || c == ' ').collect();
    Password {
        policy: Policy {
            first: policy_text[0].parse::<usize>().unwrap(),
            second: policy_text[1].parse::<usize>().unwrap(),
            letter: policy_text[2].chars().next().unwrap(),
        },
        password: text[1],
    }
}

struct Password<'a> {
    policy: Policy,
    password: &'a str,
}

struct Policy {
    first: usize,
    second: usize,
    letter: char,
}

impl Password<'_> {
    fn is_valid_by_occurences(&self) -> bool {
        let occurences = self
            .password
            .chars()
            .filter(|c| *c == self.policy.letter)
            .count();
        occurences >= self.policy.first && occurences <= self.policy.second
    }

    fn is_valid_by_locations(&self) -> bool {
        let characters: Vec<char> = self.password.chars().collect();
        return contains_character(&characters, self.policy.first - 1, self.policy.letter)
            ^ contains_character(&characters, self.policy.second - 1, self.policy.letter);
    }
}

fn contains_character(characters: &Vec<char>, index: usize, letter: char) -> bool {
    characters.get(index).map(|c| *c == letter).unwrap_or(false)
}
