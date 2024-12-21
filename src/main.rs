use rand::Rng;
use std::collections::HashMap;
use std::io;
use std::process::Command;

fn main() {
    if cfg!(target_os = "windows") {
        let _ = Command::new("cmd")
            .args(&["/C", "chcp", "65001"])
            .output()
            .expect("Failed to set terminal encoding to UTF-8");
    }

    let mut bal = 10000;

    println!("Balance: ${}", bal);

    while bal > 0 {
        println!("Enter your bet:");
        let mut bet = String::new();
        io::stdin().read_line(&mut bet).expect("Failed to read input");

        let int_bet: i32 = match bet.trim().parse() {
            Ok(val) => {
                if val <= 0 || val > bal {
                    println!("Invalid bet amount. Your current balance is: ${}", bal);
                    continue;
                }
                val
            }
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                continue;
            }
        };

        let n1 = rand::thread_rng().gen_range(1..=50);
        let n2 = rand::thread_rng().gen_range(1..=50);
        let n3 = rand::thread_rng().gen_range(1..=50);

        bal -= int_bet;

        let s1 = slot(n1);
        let s2 = slot(n2);
        let s3 = slot(n3);

        println!("{} {} {}", s1, s2, s3);

        let w = win(s1, s2, s3, int_bet);
        bal += w;

        println!("Winnings: ${}", w);
        println!("Balance: ${}", bal);
    }

    println!("Game Over.");
}

fn slot(n: i32) -> &'static str {
    if n == 7 {
        "👑"
    } else if n <= 6 {
        "🍒"
    } else if n <= 9 {
        "💎"
    } else if n <= 18 {
        "🍋"
    } else if n <= 27 {
        "🍉"
    } else if n <= 30 {
        "🍀"
    } else if n <= 32 {
        "⭐"
    } else if n <= 38 {
        "🧨"
    } else if n <= 45 {
        "🍎"
    } else {
        "🍊"
    }
}

fn win(s1: &'static str, s2: &'static str, s3: &'static str, int_bet: i32) -> i32 {
    let mut counts = HashMap::new();
    *counts.entry(s1).or_insert(0) += 1;
    *counts.entry(s2).or_insert(0) += 1;
    *counts.entry(s3).or_insert(0) += 1;

    if counts.values().any(|&count| count == 3) {
        let symbol = counts.iter().find(|&(_, &count)| count == 3).unwrap().0;
        match *symbol {
            "👑" => int_bet * 100,
            "🍒" => int_bet * 5,
            "💎" => int_bet * 8,
            "🍋" => int_bet * 3,
            "🍉" => int_bet * 3,
            "🍀" => int_bet * 8,
            "⭐" => int_bet * 15,
            "🧨" => int_bet * 3,
            "🍎" => int_bet * 3,
            _ => int_bet * 3,
        }
    } else if counts.values().any(|&count| count == 2) {
        let symbol = counts.iter().find(|&(_, &count)| count == 2).unwrap().0;
        match *symbol {
            "👑" => int_bet * 25,
            "🍒" => int_bet * 2,
            "💎" => int_bet * 5,
            "🍋" => int_bet * 2,
            "🍉" => int_bet * 2,
            "🍀" => int_bet * 5,
            "⭐" => int_bet * 6,
            "🧨" => int_bet * 2,
            "🍎" => int_bet * 2,
            _ => int_bet * 2,
        }
    } else {
        0
    }
}

