fn main() {
    let input = include_str!("../input.txt");

    println!(
        "{}",
        &input
            .replace("one", "one1one")
            .replace("two", "two2two")
            .replace("three", "three3three")
            .replace("four", "four4four")
            .replace("five", "five5five")
            .replace("six", "six6six")
            .replace("seven", "seven7seven")
            .replace("eight", "eight8eight")
            .replace("nine", "nine9nine")
            .lines()
            .map(|x| {
                let mut data: Vec<char> = x.chars().filter(|x| x.is_numeric()).collect();
                format!("{}{}", &data.first().unwrap(), &data.last().unwrap())
                    .parse::<u32>()
                    .unwrap()
            })
            .sum::<u32>()
    );
}
