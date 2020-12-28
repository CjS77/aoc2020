use crate::bits::read_data;
use regex::Regex;

pub fn day16a() -> String {
    let data = read_data("assets/day16.txt");
    let (fields, _my_ticket, tickets) = process_data(data);
    let res = tickets.iter()
        .filter_map(|t| {
            t.find_invalid(&fields).map(|(_i, v)| v)
        })
        .sum::<usize>();
    format!("{}", res)
}

pub fn day16b() -> String {
    let data = read_data("assets/day16.txt");
    let (mut fields, my_ticket, tickets) = process_data(data);
    let valid_tickets = tickets.into_iter()
        .filter(|t| {
            t.find_invalid(&fields).is_none()
        }).collect::<Vec<Ticket>>();
    sort_tickets(&mut fields, &valid_tickets);
    let res = fields.iter().filter(|f| f.name.starts_with("departure"))
        .map(|f| {
            let pos = f.ticket_position().unwrap();
            my_ticket.0[pos]
        }).product::<usize>();
    format!("{}", res)
}

fn sort_tickets(fields: &mut [Field], valid_tickets: &[Ticket]) {
    while fields.iter().any(|f| f.count_pos() > 1) {
        print!(".");
        iterate(fields, valid_tickets);
    }
    fields.iter().for_each(|f| println!("{}, {}", f.name, f.ticket_position().unwrap()));
}

fn iterate(fields: &mut[Field], valid_tickets: &[Ticket]) {
    for pos in 0..20 {
        for field in fields.iter_mut() {
            if valid_tickets.iter().any(|t| !field.is_valid(t.0[pos])) {
                field.pos[pos] = false;
            }
        }
    }
    // Elimate from rest
    for field in fields.to_vec() {
        if field.count_pos() == 1 {
            let fixed_pos = field.ticket_position().unwrap();
            fields.iter_mut().for_each(|f| if f.name != field.name { f.pos[fixed_pos] = false; })
        }
    }
}


#[derive(Debug, Clone, Copy)]
struct Ticket([usize; 20]);

impl Ticket {
    pub fn parse(s: &str) -> Ticket {
        let mut res = [0; 20];
        s.split(',').enumerate().for_each(|(i, v)| {
            let val = v.parse::<usize>().unwrap();
            res[i] = val;
        }
        );
        Self(res)
    }

    pub fn find_invalid(&self, fields: &[Field]) -> Option<(usize, usize)> {
        self.0.iter().copied().enumerate().find(|(_i, v)| {
            fields.iter().all(|f| !f.is_valid(*v))
        })
    }
}

#[derive(Debug, Clone)]
struct Field {
    name: String,
    pos: [bool; 20],
    range: [usize; 4],
}

impl Field {
    pub fn parse(s: &str, re: &Regex) -> Self {
        let cap = re.captures(s).unwrap();
        Self {
            pos: [true; 20],
            name: cap.get(1).unwrap().as_str().to_string(),
            range: [
                cap.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                cap.get(3).unwrap().as_str().parse::<usize>().unwrap(),
                cap.get(4).unwrap().as_str().parse::<usize>().unwrap(),
                cap.get(5).unwrap().as_str().parse::<usize>().unwrap(),
            ],
        }
    }

    pub fn is_valid(&self, val: usize) -> bool {
        (val >= self.range[0] && val <= self.range[1]) ||
            (val >= self.range[2] && val <= self.range[3])
    }

    pub fn count_pos(&self) -> usize {
        self.pos.iter().filter(|&p| *p).count()
    }

    pub fn ticket_position(&self) -> Option<usize> {
        self.pos.iter().enumerate().find(|(_i, &p)| p).map(|(i, _)| i)
    }
}

// departure track: 37-258 or 268-964
const REGEX: &str = r"^(.*): (\d+)-(\d+) or (\d+)-(\d+)";

fn process_data(lines: Vec<String>) -> (Vec<Field>, Ticket, Vec<Ticket>) {
    let re = Regex::new(REGEX).unwrap();
    let fields = lines[0..20].iter().map(|s| Field::parse(s.as_str(), &re)).collect::<Vec<Field>>();
    println!("{:?}", fields);
    let my_ticket = Ticket::parse(lines[22].as_str());
    println!("{:?}", my_ticket);
    let tickets = lines[25..260].iter()
        .map(|s| Ticket::parse(s.as_str())).collect::<Vec<Ticket>>();
    (fields, my_ticket, tickets)
}