use std::str::FromStr;

struct PasswordRule {
    letter: char,
    should_appear: (usize, usize),
}

impl PasswordRule {
    // First star
    // fn check(&self, s: &str) -> bool {
    //     let count = s.chars().filter(|c| *c == self.letter).count();

    //     self.should_appear.0 <= count && count <= self.should_appear.1
    // }

    // Second star
    fn check(&self, s: &str) -> bool {
        let first = if s.chars().nth(self.should_appear.0 - 1).unwrap_or_default() == self.letter {
            1
        } else {
            0
        };
        let second = if s.chars().nth(self.should_appear.1 - 1).unwrap_or_default() == self.letter {
            1
        } else {
            0
        };

        first ^ second == 1
    }
}

struct ParsePasswordRuleError {
    message: &'static str,
}

static PASSWORD_RULE_MISSING_SPACE: &ParsePasswordRuleError = &ParsePasswordRuleError {
    message: "missing ' ' delimiter",
};
static PASSWORD_RULE_MISSING_HYPHEN: &ParsePasswordRuleError = &ParsePasswordRuleError {
    message: "missing '-' delimiter",
};
static PASSWORD_RULE_MISSING_LETTER: &ParsePasswordRuleError = &ParsePasswordRuleError {
    message: "missing rule letter",
};
static PASSWORD_RULE_INVALID_RANGE: &ParsePasswordRuleError = &ParsePasswordRuleError {
    message: "range borders should be numbers",
};

impl FromStr for PasswordRule {
    type Err = &'static ParsePasswordRuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let delimiter_index = s.find(" ").ok_or(PASSWORD_RULE_MISSING_SPACE)?;

        let (part1, part2) = s.split_at(delimiter_index);

        let delimiter_index = s.find("-").ok_or(PASSWORD_RULE_MISSING_HYPHEN)?;

        let (fromstr, tostr) = part1.split_at(delimiter_index);

        let letter = part2.chars().nth(1).ok_or(PASSWORD_RULE_MISSING_LETTER)?;
        let from = match fromstr.parse() {
            Ok(from) => from,
            Err(_) => return Err(PASSWORD_RULE_INVALID_RANGE),
        };
        let to = match tostr[1..].parse() {
            Ok(to) => to,
            Err(_) => return Err(PASSWORD_RULE_INVALID_RANGE),
        };

        Ok(PasswordRule {
            letter,
            should_appear: (from, to),
        })
    }
}

fn main() {
    let input = include_str!("task2.txt").lines();

    let mut valid_password_count = 0;
    for line in input {
        let delimiter_index = line.find(": ").expect("invalid input line");
        let (rulestr, passwordstr) = line.split_at(delimiter_index);

        let rule = match rulestr.parse::<PasswordRule>() {
            Ok(rule) => rule,
            Err(err) => panic!(err.message),
        };

        if rule.check(&passwordstr[2..]) {
            valid_password_count += 1;
        }
    }

    println!("{}", valid_password_count);
}
