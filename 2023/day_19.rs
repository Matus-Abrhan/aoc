use core::panic;
use std::{fs, collections::HashMap};

fn read_file(path: &str) -> String {
    match fs::read_to_string(path) {
        Ok(string) => { return string; },
        Err(e) => {
            println!("{:?}", e);
            return String::new();
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Category {
    X(i32),
    M(i32),
    A(i32),
    S(i32)
}
impl Category {
    fn get(c: char, n: i32) -> Self {
        match c {
            'x' => { return Category::X(n); },
            'm' => { return Category::M(n); },
            'a' => { return Category::A(n); },
            's' => { return Category::S(n); },
            _ => { panic!("unrecognised category: {:?}", c); }
        };
    }
}
#[derive(Debug)]
enum Compare {
    Lesser,
    Greater
}
impl Compare {
    fn get(c: char) -> Self {
        match c {
            '>' => { return Compare::Greater; },
            '<' => { return Compare::Lesser; },
            _ => { panic!("unrecognised compare: {:?}", c); }
        };
    }
}

#[derive(Debug)]
struct Condition {
    category: Category,
    compare: Compare,
    workflow: String
}
impl Condition {
}

#[derive(Debug)]
struct Workflow {
    conditions: Vec<Condition>,
    default: String
}
impl Workflow {
    fn compare(&self, part_categories: &Vec<Category>) -> String {
        for condition in self.conditions.iter() {
            for category in part_categories.iter() {
                match (category, condition.category) {
                    (Category::X(a), Category::X(b))|
                    (Category::M(a), Category::M(b))|
                    (Category::A(a), Category::A(b))|
                    (Category::S(a), Category::S(b)) => {
                        match condition.compare {
                            Compare::Lesser => {
                                if a < &b {
                                    return condition.workflow.clone();
                                }
                            }
                            Compare::Greater => {
                                if a > &b {
                                    return condition.workflow.clone();
                                }
                            }
                        }
                    }
                    _ => {}
                };
            }
        }
        return self.default.clone();
    }
}

fn main() {
    let data = read_file("input");
    
    let (workflows, parts) = data.split_once("\n\n").unwrap();

    let mut workflow_map: HashMap<String, Workflow> = HashMap::new();
    for workflow in workflows.lines() {
        let (name, content) = workflow.split_once('{').unwrap();
        let mut conditions: Vec<String> = content.split(',').map(|s| s.to_string()).collect();
        let mut default = conditions.pop().unwrap().to_string();
        default.pop();

        let mut cond_vec: Vec<Condition> = Vec::new();
        for condition in conditions {
            let (content, workflow) = condition.split_once(':').unwrap();
            let mut number = content.to_string();
            let category = number.remove(0);
            let compare = number.remove(0);
            cond_vec.push(Condition { category: Category::get(category, number.parse::<i32>().unwrap()), compare: Compare::get(compare), workflow: workflow.to_string() })
        }
        
        workflow_map.insert(name.to_string(), Workflow { conditions: cond_vec, default: default });
    }

    let mut sum = 0;
    for part in parts.lines() {
        let mut part_categories: Vec<Category> = Vec::new();
        let categories = part.chars().filter(|c| *c != '{' && *c != '}').collect::<String>();
        for category in categories.split(',') {
            let (c, n) = category.split_once('=').unwrap();
            
            part_categories.push(Category::get(c.to_string().pop().unwrap(), n.parse::<i32>().unwrap()));
        }

        let mut name: String = "in".to_string();
        loop {
            let workflow = workflow_map.get(&name).unwrap();
            name = workflow.compare(&part_categories);
            if name == "A" || name == "R" {
                break;
            }
        }

        if name == "A" {
            for category in part_categories {
                match category {
                    Category::X(c)|
                    Category::M(c)|
                    Category::A(c)|
                    Category::S(c) => { sum += c; }
                };
            }
        }
    }
    println!("{:?}", sum);
}
