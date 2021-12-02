use std::fs;

fn main() {
    let filename = "data.txt";
    let raw = fs::read_to_string(filename).expect("something went wrong oopsie");
    let commands: Vec<(&str, i32)> = raw.lines().map(|s| create_pair(s) ).collect();

    // part_one(commands);
    part_two(commands);
}

fn create_pair(s: &str) -> (&str, i32) {
    let i = s.find(" ").unwrap();
    let command = &s[0..i];
    let num: i32 = (&s[i+1..]).parse::<i32>().unwrap();
    (command, num)
}

fn part_one(commands: Vec<(&str, i32)>) {

    let mut x = 0; // horizontal
    let mut y = 0; // depth

    for cmd in commands {
        match cmd.0 {
            "forward" => x += cmd.1,
            "down" => y += cmd.1,
            "up" => y -= cmd.1,
            _ => break
        }
    }

    println!("{}", x * y);
}


fn part_two(commands: Vec<(&str, i32)>) {

    let mut aim = 0;
    let mut x = 0; // horizontal
    let mut y = 0; // depth

    for cmd in commands {
        match cmd.0 {
            "forward" => {
                x += cmd.1;
                y += cmd.1 * aim;
            }
            "down" => aim += cmd.1,
            "up" => aim -= cmd.1,
            _ => break
        }
    }

    println!("{}", x * y);
}