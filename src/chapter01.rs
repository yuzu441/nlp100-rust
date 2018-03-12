use std::collections::HashMap;

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

pub fn q04<'a>() -> HashMap<&'a str, u32> {
    let sentence = "Hi He Lied Because Boron Could Not Oxidize Fluorine. New Nations Might Also Sign Peace Security Clause. Arthur King Can.";
    let elements_idx = [1, 5, 6, 7, 8, 9, 15, 16, 19];
    return sentence.split_whitespace()
        .enumerate()
        .map(|(idx, w)| unsafe {
            let idx = idx + 1; 
            let c = match elements_idx.contains(&idx) {
                true => w.slice_unchecked(0, 1),
                false => w.slice_unchecked(0, 2)
            };
            (c, idx as u32)
        })
        .collect();
}

#[test]
fn q04_test() {
    let expected: HashMap<&str, u32> = [
        ("H", 1), ("He", 2), ("Li", 3), ("Be", 4), ("B", 5),
        ("C", 6), ("N", 7), ("O", 8), ("F", 9), ("Ne", 10),
        ("Na", 11), ("Mi", 12), ("Al", 13), ("Si", 14), ("P", 15),
        ("S", 16), ("Cl", 17), ("Ar", 18), ("K", 19), ("Ca", 20)
    ].iter().cloned().collect();
    let result = q04();
    for (key, idx) in expected {
        assert_eq!(idx, result[key]);
    }
}
