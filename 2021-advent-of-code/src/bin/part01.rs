fn main() {
    let dat = std::str::from_utf8(include_bytes!("input01.txt")).expect("bad input file!");

    let nums: Vec<u64> = dat
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect();

    let mut count = 0;
    for i in 1..nums.len() {
        if nums[i] > nums[i - 1] {
            count += 1;
        }
    }

    println!("{}", count);

    let mut sums: Vec<u64> = Vec::new();

    for i in 0..nums.len() - 2 {
        sums.push(nums[i] + nums[i + 1] + nums[i + 2]);
    }

    let mut count = 0;
    for i in 1..sums.len() {
        if sums[i] > sums[i - 1] {
            count += 1;
        }
    }

    println!("{}", count);
}
