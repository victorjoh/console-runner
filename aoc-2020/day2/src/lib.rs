pub fn solve_part_1(input: &String) -> String {
    let passwords: Vec<Password> = input.trim().split('\n').map(parse_password).collect();
    return passwords.iter().filter(|p| p.is_valid()).count().to_string();
}

fn parse_password(text: &str) -> Password {
    Password {
        policy: Policy {
            letter: 'a',
            min: 1,
            max: 9,
        },
        password: String::from("awd"),
    }
}

struct Password {
    policy: Policy,
    password: String,
}

struct Policy {
    letter: char,
    min: i32,
    max: i32,
}

impl Password {
    fn is_valid(&self) -> bool {return true}
}

pub fn solve_part_2(input: &String) -> String {
    String::from("not yet implemented")
}
