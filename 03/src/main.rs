use std::io;

fn convert_temp() -> () {
    println!("Input temperature value: ");

    let t: i32 = loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<i32>() {
            Ok(num) => break num,
            Err(_) => println!("Unrecognised number"),
        }
    };

    println!("Input unit ('C' or 'F') :");

    let unit: char = loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<char>() {
            Ok(c) => match c {
                'C' | 'F' => break c,
                _ => (),
            },
            Err(_) => (),
        }
        println!("Unrecognised unit");
    };

    if unit == 'C' {
        println!("{:.2} F", (t as f32) * 9. / 5. + 32.);
    } else {
        println!("{:.2}°", ((t as f32) - 32.) * 5. / 9.);
    }
}

fn fibonacci(n: i32) -> i32 {
    let mut i = 0;
    let mut fibo_n: i32 = 0;
    let mut fibo_n_: i32 = 1;

    while i < n {
        (fibo_n, fibo_n_) = (fibo_n + fibo_n_, fibo_n);
        i += 1;
    }

    return fibo_n;
}

fn main() {
    println!(
        "Input a program:
    \t 'T' for Celsius-Fahrenheit temperature convertor;
    \t 'F' for Fibonacci number computation;"
    );

    let program: bool = loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim() {
            "T" => break true,
            "F" => break false,
            _ => println!("Unrecognised program"),
        }
    };

    if program {
        convert_temp();
    } else {
        println!("\t Input n ≥ 0 to compute F_n:");
        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let n: i32 = match input.trim().parse::<i32>() {
                Ok(num) => num,
                Err(_) => {
                    println!("\t Please provide a number.");
                    continue;
                },
            };

            if n >= 0 {
                {
                    println!("F_n = {}", fibonacci(n));
                    break;
                }
            } else {
                println!("n must be positive. Please input another number.")
            }
        }
    }
}
