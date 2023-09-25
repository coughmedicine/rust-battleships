use std::{
    fmt::format,
    io::{BufRead, BufReader, Read, Write},
    process::Stdio,
};

#[test]
fn test_cmd_game() {
    let mut cmd = test_bin::get_test_bin("cmd");
    let cmd = cmd.stdin(Stdio::piped()).stdout(Stdio::piped());

    let mut handle = cmd.spawn().unwrap();
    let mut stdin = handle.stdin.take().unwrap();
    let mut stdout = BufReader::new(handle.stdout.take().unwrap());

    stdin.write_all(b"1\n1\nH\n").unwrap();
    stdin.write_all(b"1\n2\nH\n").unwrap();
    stdin.write_all(b"1\n3\nH\n").unwrap();
    stdin.write_all(b"1\n4\nH\n").unwrap();
    stdin.write_all(b"1\n5\nH\n").unwrap();

    stdin.write_all(b"1\n1\nV\n").unwrap();
    stdin.write_all(b"2\n1\nV\n").unwrap();
    stdin.write_all(b"3\n1\nV\n").unwrap();
    stdin.write_all(b"4\n1\nV\n").unwrap();
    stdin.write_all(b"5\n1\nV\n").unwrap();

    for i in 1..=5 {
        for j in 1..=[2, 3, 3, 4, 5][i - 1] {
            let s1 = format!("{}\n{}\n", i, j);
            stdin.write_all(s1.as_bytes()).unwrap();
            stdin.write_all(b"0\n0\n").unwrap();
        }
    }

    loop {
        let mut out = String::new();
        let count = stdout.read_line(&mut out).unwrap();
        if out.contains("Congratulations Player 1!") {
            break;
        }
        if count == 0 {
            panic!("Failed because stdout never contained the Congratulations string");
        }
    }
}
