use std::io;

fn fibo(n: i32) -> i32 {
    if n > 2 {
        return fibo(n - 1) + fibo(n - 2);
    } else {
        return 1;
    }
}

fn check_valid(n: i32) -> i32 {
    if n < 0 {
        -2
    } else if 0 == n {
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
    if -2 == valid {
        println!("Inputed number less than zero");
    } else if -1 == valid {
        println!("Result 0")
    } else {
        let res = fibo(n);
        println!("Result {res}");
    }
}
