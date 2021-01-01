use crate::bits::read_data;

pub fn day18a() -> String {
    read_data("assets/day18.txt").iter()
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut stack = ExecutionStack::from_str(s.as_str());
            stack.exec()
        })
        .sum::<isize>()
        .to_string()
}

pub fn day18b() -> String {
    read_data("assets/day18.txt").iter()
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut stack = ExecutionStack::from_str(s.as_str());
            stack.exec_adv()
        })
        .sum::<isize>()
        .to_string()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StackItem {
    Number(isize),
    Add,
    Multiply,
    Paren(ExecutionStack),
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ExecutionStack {
    items: Vec<StackItem>,
}

impl ExecutionStack {
    /// Return a new `ExecutionStack` using the vector of [StackItem] in `items`
    pub fn new(items: Vec<StackItem>) -> Self {
        ExecutionStack { items }
    }

    pub fn from_str(s: &str) -> Self {
        let mut items = Vec::new();
        let mut sub_expr = "".to_string();
        let mut paren_count = 0;
        let mut in_sub_expr = false;
        // println!("{}", s);
        for token in s.split(' ') {
            // println!("Token: '{}' Subexpr='{}', ()={}, in_sub={}", token, sub_expr, paren_count, in_sub_expr);
            if token.is_empty() { continue; }
            if token.starts_with('(') {
                in_sub_expr = true;
                paren_count += token.chars().filter(|c| *c == '(').count();
            }
            if in_sub_expr {
                let num_closing = token.find(')').map(|i| token.len() - i).unwrap_or_default();
                paren_count -= num_closing;
                sub_expr = if sub_expr.is_empty() {
                    token.to_string()
                } else {
                    format!("{} {}", sub_expr, token)
                };
                if paren_count == 0 {
                    // println!("Calling sub stack with '{}'", &sub_expr[1..sub_expr.len()-1]);
                    let sub_stack = ExecutionStack::from_str(&sub_expr[1..sub_expr.len() - 1]);
                    items.push(StackItem::Paren(sub_stack));
                    sub_expr = "".to_string();
                    in_sub_expr = false;
                }
                continue;
            }
            if token == "+" {
                items.push(StackItem::Add);
                continue;
            }
            if token == "*" {
                items.push(StackItem::Multiply);
                continue;
            }
            // Should be a number
            let val = token.parse::<isize>().unwrap();
            items.push(StackItem::Number(val));
        }
        Self { items }
    }

    pub fn exec(&mut self) -> isize {
        let mut register = 0;
        let mut next_op = None;
        self.items.drain(0..).for_each(|op| {
            match op {
                StackItem::Add => next_op = Some(StackItem::Add),
                StackItem::Multiply => next_op = Some(StackItem::Multiply),
                StackItem::Number(v) => {
                    match next_op {
                        None => register = v,
                        Some(StackItem::Add) => register += v,
                        Some(StackItem::Multiply) => register *= v,
                        _ => unreachable!(),
                    }
                    next_op = None;
                }
                StackItem::Paren(mut substack) => {
                    let v = substack.exec();
                    match next_op {
                        None => register = v,
                        Some(StackItem::Add) => register += v,
                        Some(StackItem::Multiply) => register *= v,
                        _ => unreachable!(),
                    }
                    next_op = None;
                }
            }
        });
        register
    }

    pub fn exec_adv(&mut self) -> isize {
        let mut register = 0;
        let mut next_op = false;
        let items = self.items.clone();
        for (i, op) in items.into_iter().enumerate() {
            // println!("{}, {:?}", i, op);

            match op {
                StackItem::Add => next_op = true,
                StackItem::Multiply => {
                    let mut sub_stack = ExecutionStack::new(self.items[i+1..].to_vec());
                    let v = sub_stack.exec_adv();
                    return register * v
                }
                StackItem::Number(v) => {
                    if next_op {
                        register += v;
                    } else {
                        register = v;
                    }
                    next_op = false;
                }
                StackItem::Paren(mut substack) => {
                    let v = substack.exec_adv();
                    if next_op {
                        register += v;
                    } else {
                        register = v;
                    }
                    next_op = false;
                }
            }
        }
        register
    }
}