use std::io;

fn fibo(n: i32) -> i32 {
    if n > 1 {
        return fibo(n - 1) + fibo(n - 2);
    } else {
        return n;
    }
}

fn check_valid(n: i32) -> i32 {
    if n < 0 {
        -1
    } else {
        n
    }
}

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("failed to readline");
    let n: i32 = n.trim().parse().expect("inputed is not a num");
    let valid = check_valid(n);
    if -1 == valid {
        println!("Inputed number less than zero");
    } else {
        let res = fibo(n);
        println!("Result {res}");
    }
}
