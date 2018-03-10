pub fn q00() -> String {
    let x = "stressed";
    return x.chars().rev().collect::<String>();
}

#[test]
fn q00_test() {
    assert_eq!("desserts", q00());
}

pub fn q01() -> String {
    let x = "パタトクカシーー";
    return x.chars()
        .enumerate()
        .filter(|&(idx, _c)| idx % 2 == 0)
        .map(|(_, c)| c)
        .collect::<String>();
}

#[test]
fn q01_test() {
    assert_eq!("パトカー", q01());
}

pub fn q02() -> String {
    let in1 = "パトカー";
    let in2 = "タクシー";
    return in1.chars()
        .zip(in2.chars())
        .map(|(x, y)| format!("{}{}", x, y))
        .collect::<String>();
}

#[test]
fn q02_test() {
    assert_eq!("パタトクカシーー", q02());
}

pub fn q03() -> Vec<usize> {
    let sentence = "Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics.";
    let pat: &[_] = &[',', '.'];

    return sentence.split_whitespace()
        .map(|word| word.trim_right_matches(pat))
        .map(|w| w.len())
        .collect::<Vec<usize>>();
}

#[test]
fn q03_test() {
    assert_eq!(vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9], q03());
}