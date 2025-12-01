use std::cmp::Ordering;

#[derive(Debug)]
struct Value<'rules> {
    pub val: usize,
    pub relevant_rules: Vec<&'rules Rule>
}

impl<'a> Value<'a> {
    pub fn new(val: usize, all_rules: &'a Vec<Rule>) -> Self {
        let relevant_rules = all_rules.iter().filter(|x| x.is_relevant(&vec![val])).collect();
        
        Self {
            val,
            relevant_rules
        }
    }
}

impl<'rules> std::cmp::Eq for Value<'rules> {}

impl<'rules> std::cmp::PartialEq for Value<'rules> {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }

    fn ne(&self, other: &Self) -> bool {
        self.val != other.val
    }
}

impl<'rules> std::cmp::Ord for Value<'rules> {

    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if let Some(the_rule) = self.relevant_rules.iter().filter(|x| x.is_relevant(&vec![other.val])).next() {
            if the_rule.before == self.val {
                return Ordering::Less
            }
            else {
                return Ordering::Greater
            }
        }

        Ordering::Equal
    }
}

impl<'rules> std::cmp::PartialOrd for Value<'rules> {

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}


#[derive(Debug, Clone)]
struct Rule {
    pub before: usize,
    pub after: usize,
}

impl Rule {
    pub fn is_relevant(&self, values: &Vec<usize>) -> bool {
        values.contains(&self.before) || values.contains(&self.after)
    }

    pub fn is_correct_order(&self, values: &Vec<Value>) -> bool {
        let values: Vec<usize> = values.iter().map(|x| x.val).collect();
        let relevant_values: Vec<&usize> = values.iter().filter(|x| **x == self.after || **x == self.before).collect();
        
        if relevant_values.len() != 2 {
            return true
        }

        if *relevant_values[0] == self.before && *relevant_values[1] == self.after {
            return true
        }

        return false
    }
}

fn middle_of_pages(line: &str, rules: &Vec<Rule>) -> usize {
    let mut values: Vec<Value> = line.split(",").map(|x| x.parse().unwrap()).map(|x: usize| Value::new(x, rules)).collect();

    if rules.iter().all(|x| x.is_correct_order(&values)) { 
        return 0
    }
    
    values.sort();
    values[values.len() / 2].val
}

fn main() {
    let input = include_str!("../input.txt");
    let mut rules: Vec<Rule> = vec![];

    let mut iterator = input.lines();

    for line in &mut iterator {
        if line == "" {
            break;
        } else {
            rules.push(Rule {
                before: line[0..=1].parse().unwrap(),
                after: line[3..=4].parse().unwrap(),
            });
        }
    }

    let left_over_lines: Vec<&str> = iterator.collect();

    let a: usize = left_over_lines.iter().map(|x| middle_of_pages(x, &rules)).sum();

    println!("{}", a)
}
