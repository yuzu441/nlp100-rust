pub fn q00() {
    let x = "stressed";
    println!("{}", x.chars().rev().collect::<String>());
}

pub fn q01() {
    let x = "パタトクカシーー";
    let result = x.chars()
        .enumerate()
        .filter(|&(idx, _c)| idx % 2 == 0)
        .map(|(_, c)| c )
        .collect::<String>();
    println!("{}", result);
}