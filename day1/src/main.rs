use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("File should exists");

    let elfs = parse_input(&input);

    let most_calories = get_elf_with_most_calories(&elfs);
    println!("The most calories an elf carries is {}", most_calories);

    let top_three = get_top_three_most_calories(&elfs);
    println!("The top three elf carries {} calories", top_three);
    

}

fn get_top_three_most_calories(elfs: &Vec<Vec<u32>>) -> u32 {

    let mut top_three_calories: Vec<u32> = vec![0,0,0]; 

    let mut current_calories: u32 = 0;

    for elf in elfs {
        for snack in elf {
            current_calories += snack;
        }
        top_three_calories.sort();
        let min_top_three = top_three_calories[0];
        if current_calories > min_top_three {
            top_three_calories[0] = current_calories;
        }
        current_calories = 0;
    }

    return top_three_calories.iter().sum();

}

fn get_elf_with_most_calories(elfs: &Vec<Vec<u32>>) -> u32{

    let mut most_calories: u32 = 0;
    let mut current_calories: u32 = 0;

    for elf in elfs {
        for snack in elf {
            current_calories += snack;
        }
        if current_calories > most_calories {
            most_calories = current_calories;
        } 
        current_calories = 0;
    }

    return most_calories;
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    let parts = input.split("\n");

    let mut data: Vec<Vec<u32>> = Vec::new();

    let mut current: Vec<u32> = Vec::new();

    for part in parts {
        let value_result = part.trim().parse::<u32>();
        match value_result {
            Ok(value) => current.push(value),
            Err(_e) =>  {
                data.push(current);
                current = Vec::new();
            }
        };
    }

    if !current.is_empty() {
        data.push(current);
    }


    return data;
}

