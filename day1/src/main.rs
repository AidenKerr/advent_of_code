use std::fs;

fn main() {
    let filename = "data.txt";
    let raw = fs::read_to_string(filename).expect("something went wrong oopsie");
    let nums: Vec<i32> = raw.lines().map(|s| s.parse().expect("error")).collect();

    // println!("count: {}", part_one(nums));
    println!("count: {}", part_two(nums));
    // println!("count: {}", part_two_2(nums));
    // println!("count: {}", part_two_3(nums));
}

fn part_one(nums: Vec<i32>) -> i32 {
    let mut last = -1;
    let mut count = 0;

    for n in nums {
        if last != -1 && n > last {
            count += 1;
        }
        last = n;
    }

    count
}

fn part_two(nums: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut last = 0;

    for i in 0..(nums.len() - 3) {
        let mut sum = 0;
        if last != -1 {
            for j in i..(i + 3) {
                sum += nums[j];
            }

            if sum > last {
                count += 1;
            }
        }

        last = sum;
    }

    count
}