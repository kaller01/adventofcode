use std::fs;
use std::collections::VecDeque;
use std::collections::HashSet;

fn main() {
    let message = fs::read_to_string("./input.txt").expect("File should exists");

    let size = 14;

    let mut window: VecDeque<char> = VecDeque::with_capacity(size);

    for (index,c) in message.chars().enumerate() {

        if window.len() >= size {
            let mut set_from_window = HashSet::new();
            for a in &window {
                set_from_window.insert(*a);
            }

            if set_from_window.len() == size {
                println!("Message start at {index}");
                break;
            }

            window.pop_front();
        }

        //Put into window
        window.push_back(c);
    }
}
