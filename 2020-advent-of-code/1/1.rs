fn main() {
    let dat = std::fs::read_to_string("input.txt").expect("couldn't read file");

    let nums: Vec<i64> = dat
        .lines()
        .map(|x| x.parse::<i64>().expect("invalid int"))
        .collect();

    for i in 0..(nums.len() - 1) {
        for j in i..nums.len() {
            if nums[i] + nums[j] == 2020 {
                println!("{}", nums[i] * nums[j]);
                return;
            }
        }
    }
}
