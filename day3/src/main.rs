use std::fs;
fn main() {
    let input = fs::read_to_string("./input.txt").expect("File should exists");
    let backpacks = parse_input(&input);
    // println!("{:?}",backpacks);

    let mut items: Vec<Item> = Vec::new();
    for backpack in backpacks {
        match find_item_in_both_compartments(backpack) {
            Some(c) => items.push(Item { value: c}),
            None => println!()
        }
    }
    let mut sum: u32= 0;

    for item in items {
        // println!("{}{}",item.value,item.get_priority());
        sum += item.get_priority() as u32;
    }

    println!("{sum}");
    
}

#[derive(Debug)]
struct Backpack {
    first: String,
    second: String,
}

impl Backpack {
    fn parse(source: &str) -> Backpack {
        let middle_index = source.len() / 2;
        return Backpack { first: source.trim().chars().take(middle_index).collect(), second: source.trim().chars().skip(middle_index).collect()};
    }
}

struct Item {
    value: char
}

impl Item {
    fn get_priority(&self) -> u8 {
        if self.value.is_uppercase() {
            return self.value as u8 - 38;
        }
        if self.value.is_lowercase() {
            return self.value as u8 - 96;
        }
        return 0;
    }
}


fn parse_input(input: &str) -> Vec<Backpack> {
    let mut backpacks: Vec<Backpack> = Vec::new();
    for line in input.split("\n") {
         backpacks.push(Backpack::parse(line));
    }
    return backpacks;
}

fn find_item_in_both_compartments(backpack: Backpack) -> Option<char> {
    for c1 in backpack.first.chars() {
        for c2 in backpack.second.chars() {
            if c1 == c2 {
                return Some(c1);
            }
        }
    }
    return None;
}