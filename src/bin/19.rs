use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, one_of},
    combinator::map,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

advent_of_code::solution!(19);

#[derive(Debug, Clone, PartialEq, Eq)]
enum Destination {
    Accept,
    Reject,
    Workflow(String),
}

impl Destination {
    fn parse(input: &str) -> IResult<&str, Destination> {
        let (input, destination) = alt((tag("A"), tag("R"), alpha1))(input)?;
        match destination {
            "A" => Ok((input, Destination::Accept)),
            "R" => Ok((input, Destination::Reject)),
            _ => Ok((input, Destination::Workflow(destination.to_string()))),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Condition {
    LessThan(char, isize),
    GreaterThan(char, isize),
    LessThanEqual(char, isize),
    GreaterThanEqual(char, isize),
}

impl Condition {
    fn opposite(&self) -> Self {
        match self {
            Self::LessThan(var, val) => Self::GreaterThanEqual(*var, *val),
            Self::GreaterThan(var, val) => Self::LessThanEqual(*var, *val),
            Self::LessThanEqual(var, val) => Self::GreaterThan(*var, *val),
            Self::GreaterThanEqual(var, val) => Self::LessThan(*var, *val),
        }
    }

    fn evaluate(&self, part: Part) -> bool {
        match self {
            Self::LessThan(var, val) if part.value(var) < *val => true,
            Self::GreaterThan(var, val) if part.value(var) > *val => true,
            Self::LessThanEqual(var, val) if part.value(var) <= *val => true,
            Self::GreaterThanEqual(var, val) if part.value(var) >= *val => true,
            _ => false,
        }
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, (var, op, val)) = tuple((one_of("xmas"), one_of("<>"), digit1))(input)?;
        let val = val.parse().unwrap();
        let condition = match op {
            '<' => Condition::LessThan(var, val),
            '>' => Condition::GreaterThan(var, val),
            _ => unreachable!(),
        };
        Ok((input, condition))
    }
}

#[derive(Debug, Clone)]
enum Rule {
    Evaluation(Condition, Destination),
    Fallthrough(Destination),
}

impl Rule {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            map(
                tuple((Condition::parse, tag(":"), Destination::parse)),
                |(c, _, d)| Rule::Evaluation(c, d),
            ),
            map(Destination::parse, Rule::Fallthrough),
        ))(input)
    }

    fn evaluate(&self, part: Part) -> Option<Destination> {
        match self {
            Self::Evaluation(condition, destination) => {
                if condition.evaluate(part) {
                    Some(destination.clone())
                } else {
                    None
                }
            }
            Self::Fallthrough(destination) => Some(destination.clone()),
        }
    }
}

#[derive(Debug, Clone)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, name) = alpha1(input)?;
        let name = name.to_string();
        let (input, _) = tag("{")(input)?;
        let (input, rules) = separated_list1(tag(","), Rule::parse)(input)?;
        let (input, _) = tag("}")(input)?;
        Ok((input, Workflow { name, rules }))
    }

    fn evaluate(&self, part: &Part) -> Destination {
        self.rules
            .iter()
            .find_map(|rule| rule.evaluate(part.clone()))
            .unwrap()
    }
}

#[derive(Debug, Clone)]
struct Part {
    x: isize,
    m: isize,
    a: isize,
    s: isize,
}

impl Part {
    fn total_rating(&self) -> isize {
        self.x + self.m + self.a + self.s
    }

    fn set(&mut self, var: char, val: isize) {
        match var {
            'x' => self.x = val,
            'm' => self.m = val,
            'a' => self.a = val,
            's' => self.s = val,
            _ => unreachable!(),
        }
    }

    fn value(&self, var: &char) -> isize {
        match var {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => unreachable!(),
        }
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, _) = tag("{x=")(input)?;
        let (input, x) = digit1(input)?;
        let x = x.parse().unwrap();
        let (input, _) = tag(",m=")(input)?;
        let (input, m) = digit1(input)?;
        let m = m.parse().unwrap();
        let (input, _) = tag(",a=")(input)?;
        let (input, a) = digit1(input)?;
        let a = a.parse().unwrap();
        let (input, _) = tag(",s=")(input)?;
        let (input, s) = digit1(input)?;
        let s = s.parse().unwrap();
        let (input, _) = tag("}")(input)?;
        Ok((input, Part { x, m, a, s }))
    }

    fn process(&self, workflows: &HashMap<String, Workflow>) -> Destination {
        let mut cur = workflows.get("in").unwrap();
        loop {
            match cur.evaluate(self) {
                Destination::Accept => return Destination::Accept,
                Destination::Reject => return Destination::Reject,
                Destination::Workflow(name) => {
                    cur = workflows.get(&name).unwrap();
                }
            }
        }
    }
}

fn generate_accepted_paths(
    workflows: &HashMap<String, Workflow>,
    cur: &str,
    parents: &[Condition],
) -> Vec<Vec<Condition>> {
    let mut paths = vec![];

    let workflow = workflows.get(cur).unwrap();
    let mut previous_conditions = vec![];

    for rule in &workflow.rules {
        let mut new_parents = parents.to_vec();
        new_parents.extend(previous_conditions.clone());
        match rule {
            Rule::Evaluation(condition, dest) => {
                new_parents.push(*condition);

                previous_conditions.push(condition.opposite());

                match dest {
                    Destination::Accept => {
                        paths.push(new_parents);
                    }
                    Destination::Reject => {}
                    Destination::Workflow(name) => {
                        paths.extend(generate_accepted_paths(workflows, name, &new_parents));
                    }
                }
            }
            Rule::Fallthrough(dest) => match dest {
                Destination::Accept => {
                    paths.push(new_parents);
                }
                Destination::Reject => {}
                Destination::Workflow(name) => {
                    paths.extend(generate_accepted_paths(workflows, name, &new_parents));
                }
            },
        }
    }
    paths
}

fn calculate_possible_combinations(path: &[Condition]) -> isize {
    let mut min_part = Part {
        x: 1,
        m: 1,
        a: 1,
        s: 1,
    };
    let mut max_part = Part {
        x: 4000,
        m: 4000,
        a: 4000,
        s: 4000,
    };

    for condition in path {
        match condition {
            Condition::LessThan(var, val) => {
                max_part.set(*var, max_part.value(var).min(val - 1));
            }
            Condition::LessThanEqual(var, val) => {
                max_part.set(*var, max_part.value(var).min(*val));
            }
            Condition::GreaterThan(var, val) => {
                min_part.set(*var, min_part.value(var).max(val + 1));
            }
            Condition::GreaterThanEqual(var, val) => {
                min_part.set(*var, min_part.value(var).max(*val));
            }
        }
    }

    (max_part.x - min_part.x + 1)
        * (max_part.m - min_part.m + 1)
        * (max_part.a - min_part.a + 1)
        * (max_part.s - min_part.s + 1)
}

pub fn part_one(input: &str) -> Option<isize> {
    let (rest, workflows) = separated_list1(tag("\n"), Workflow::parse)(&input).unwrap();
    let workflows = workflows
        .iter()
        .map(|w| (w.name.clone(), w.clone()))
        .collect::<HashMap<String, Workflow>>();

    let parts = rest
        .trim()
        .split('\n')
        .map(|line| Part::parse(line).unwrap().1)
        .collect::<Vec<Part>>();

    let result = parts
        .iter()
        .filter(|p| p.process(&workflows) == Destination::Accept)
        .map(|p| p.total_rating())
        .sum::<isize>();
    Option::Some(result)
}

pub fn part_two(input: &str) -> Option<isize> {
    let (_, workflows) = separated_list1(tag("\n"), Workflow::parse)(&input).unwrap();
    let workflows = workflows
        .iter()
        .map(|w| (w.name.clone(), w.clone()))
        .collect::<HashMap<String, Workflow>>();

    let paths = generate_accepted_paths(&workflows, "in", &[]);
    let result = paths
        .iter()
        .map(|p| calculate_possible_combinations(p))
        .sum::<isize>();

    Option::Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Option::Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Option::Some(167409079868000));
    }
}
