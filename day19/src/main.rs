use std::collections::HashMap;

// way too sick to do part 2

#[derive(Debug, Clone)]
struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

impl Part {
    fn get_value(&self, c: char) -> i64 {
        match c {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => panic!("invalid char"),
        }
    }
}

#[derive(Debug)]
enum Action {
    Redirect(String),
    Reject,
    Approve,
}

#[derive(Debug)]
enum Condtion {
    Gt(char, i64),
    Lt(char, i64),
}

#[derive(Debug)]
struct Rule {
    condition: Option<Condtion>,
    action: Action,
}

fn create_rule_map(rules: &str) -> HashMap<String, Vec<Rule>> {
    rules
        .lines()
        .map(|line| {
            let (id, rule) = line.split_once('{').unwrap();
            let rule = &rule[0..rule.len() - 1];

            let rules = rule.split(',');

            let rules: Vec<Rule> = rules
                .map(|rule| {
                    let has_condition = rule.contains(':');

                    if has_condition {
                        // first char is the char, second char is > or <, numbers until : are amount
                        let (condition, action) = rule.split_once(':').unwrap();
                        println!("condition: {}, action: {}", condition, action);

                        let index = condition.find('<').or(condition.find('>')).unwrap();
                        let (char, condition_and_amount) = condition.split_at(index);
                        let char = char.chars().nth(0).unwrap();
                        let (condition, amount) = condition_and_amount.split_at(1);
                        let condition = condition.chars().nth(0).unwrap();
                        
                        let amount = amount.parse::<i64>().unwrap();
                        let condition = match condition {
                            '<' => Condtion::Lt(char, amount),
                            '>' => Condtion::Gt(char, amount),
                            _ => panic!("invalid condition"),
                        };

                        let action = match action {
                            "A" => Action::Approve,
                            "R" => Action::Reject,
                            _ => Action::Redirect(action.to_string()),
                        };

                        return Rule {
                            condition: Some(condition),
                            action,
                        };
                    } else {
                        let action = match rule {
                            "A" => Action::Approve,
                            "R" => Action::Reject,
                            _ => Action::Redirect(rule.to_string()),
                        };

                        return Rule {
                            condition: None,
                            action,
                        };
                    }
                })
                .collect();

            (id.to_string(), rules)
        })
        .collect()
}

fn create_parts(parts: &str) -> Vec<Part> {
    parts
        .lines()
        .map(|line| {
            let line = &line[1..line.len() - 1];
            let parts = line.split(',');

            let mut x = 0;
            let mut m = 0;
            let mut a = 0;
            let mut s = 0;

            for part in parts {
                let (key, value) = part.split_once('=').unwrap();
                let value = value.parse::<i64>().unwrap();
                match key {
                    "x" => x = value,
                    "m" => m = value,
                    "a" => a = value,
                    "s" => s = value,
                    _ => panic!("invalid key"),
                }
            }

            return Part { x, m, a, s };
        })
        .collect()
}

fn process_part(part: &Part, rules: &Vec<Rule>, rule_map: &HashMap<String, Vec<Rule>>) -> bool {
    for rule in rules {
        // check if condition is met
        if let Some(condition) = &rule.condition {
            match condition {
                Condtion::Gt(char, condition_amount) => {
                    if part.get_value(*char) < *condition_amount {
                        continue;
                    }
                }
                Condtion::Lt(char, condition_amount) => {
                    if part.get_value(*char) > *condition_amount {
                        continue;
                    }
                }
            }
        }

        match &rule.action {
            Action::Approve => {
                return true;
            }
            Action::Reject => {
                return false;
            }
            Action::Redirect(id) => {
                let new_rules = &rule_map[id];
                return process_part(part, new_rules, rule_map);
            }
        }
    }

    panic!("something went wrong");
}

const START_RULE: &str = "in";

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();

    let (rules, parts) = file.split_once("\n\n").unwrap();

    let rule_map = create_rule_map(rules);
    let parts = create_parts(parts);

    let mut xmas_sum = 0;

    for part in parts {
        let rules = &rule_map[START_RULE];
        if process_part(&part, rules, &rule_map) {
            xmas_sum += part.x + part.m + part.a + part.s;
        }
    }
    println!("part_1_solution: {}", xmas_sum);
    
}
