fn line_to_score(line: String) -> Result<u32, String> {
    Ok(match line.as_str() {
        "A X" => 3 + 1,
        "A Y" => 6 + 2,
        "A Z" => 0 + 3,
        "B X" => 0 + 1,
        "B Y" => 3 + 2,
        "B Z" => 6 + 3,
        "C X" => 6 + 1,
        "C Y" => 0 + 2,
        "C Z" => 3 + 3,
        _ => return Err("invalid input".to_string()),
    })
}

fn line_to_score2(line: String) -> Result<u32, String> {
    Ok(match line.as_str() {
        "A X" => 3 + 1,
        "A Y" => 6 + 2,
        "A Z" => 0 + 3,
        "B X" => 0 + 1,
        "B Y" => 3 + 2,
        "B Z" => 6 + 3,
        "C X" => 6 + 1,
        "C Y" => 0 + 2,
        "C Z" => 3 + 3,
        _ => return Err("invalid input".to_string()),
    })
}

fn main() {
    let input = include_str!("day2.txt");
    let lines = input.lines();
    let part2lines = lines.clone();
    let sum = lines
        .map(|line| line_to_score(line.to_string()).unwrap())
        .collect::<Vec<u32>>()
        .iter()
        .sum::<u32>();
    println!("part 1: {:?}", sum);

    let part2sum = part2lines
        .map(|line| line_to_score2(line.to_string()).unwrap())
        .collect::<Vec<u32>>()
        .iter()
        .sum::<u32>();
    println!("part 2: {:?}", part2sum);
}
