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

pub fn q02() {
    let in1 = "パトカー";
    let in2 = "タクシー";
    let result = in1.chars()
        .zip(in2.chars())
        .map(|(x, y)| format!("{}{}", x, y))
        .collect::<String>();
    println!("{}", result);
}