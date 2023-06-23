#![allow(dead_code)]

pub fn backwards1(forwards: &str) -> String {
    let mut backwards = String::new();
    for i in (0..forwards.chars().count()).rev() {
        backwards.push(forwards.chars().nth(i).unwrap());
    }
    backwards
}

pub fn backwards2(forwards: &str) -> String {
    //let forwards = String::from(forwards);
    let mut backwards = String::new();
    for c in forwards.chars().rev() {
        backwards.push(c);
    }
    backwards
}

pub fn backwards3(forwards: &str) -> String {
    //let forwards = forwards.chars().collect::<Vec<char>>();
    let mut list: Vec<char> = forwards.chars().collect();
    let half_len = list.len() / 2;
    for i0 in 0..half_len {
        let i1 = list.len() - 1 - i0;
        list.swap(i0, i1);
    }
    list.iter().collect()
}

pub fn backwards4(forwards: &str) -> String {
    forwards.chars().rev().collect()
}
