use std::time::Instant;

fn main() {
    for (idx, fizzbuzz) in ALL_FIZZBUZZS.iter().enumerate() {
        let current = Instant::now();
        for num in 0..10_000_000 {
            fizzbuzz(num);
        }

        let duration = current.elapsed();
        println!("{} ran in {:?}", ALL_FIZZBUZZ_NAMES[idx], duration);
    }
}

type FizzBuzzFn = fn(u32) -> String;
const ALL_FIZZBUZZS: [FizzBuzzFn; 7] = [
    brute_fizzbuzz,
    accumulate_fizzbuzz,
    compositional_fizzbuzz,
    bitaccumulate_fizzbuzz,
    matchbox_fizzbuzz,
    branchless_fizzbuzz,
    smartbranchless_fizzbuzz,
];

const ALL_FIZZBUZZ_NAMES: [&str; 7] = [
    "brute_fizzbuzz",
    "accumulate_fizzbuzz",
    "compositional_fizzbuzz",
    "bitaccumulate_fizzbuzz",
    "matchbox_fizzbuzz",
    "branchless_fizzbuzz",
    "smartbranchless_fizzbuzz",
];

fn brute_fizzbuzz(num: u32) -> String {
    if num % 15 == 0 {
        String::from("FizzBuzz")
    } else if num % 3 == 0 {
        String::from("Fizz")
    } else if num % 5 == 0 {
        String::from("Buzz")
    } else {
        num.to_string()
    }
}

fn accumulate_fizzbuzz(num: u32) -> String {
    let mut result = String::from("");
    if num % 3 == 0 {
        result += "Fizz";
    }

    if num % 5 == 0 {
        result += "Buzz";
    }

    if result.is_empty() {
        result += &num.to_string();
    }

    result
}

fn bitaccumulate_fizzbuzz(num: u32) -> String {
    let results: [&str; 3] = ["Fizz", "Buzz", "FizzBuzz"];
    let mut idx = 0;

    if num % 3 == 0 {
        idx += 1;
    }

    if num % 5 == 0 {
        idx += 2;
    }

    if idx == 0 {
        num.to_string()
    } else {
        String::from(results[idx - 1])
    }
}

fn compositional_fizzbuzz(num: u32) -> String {
    fn opti_buzz(num: u32) -> String {
        if num % 5 == 0 {
            String::from("FizzBuzz")
        } else {
            String::from("Fizz")
        }
    }

    fn pessi_buzz(num: u32) -> String {
        if num % 5 == 0 {
            String::from("Buzz")
        } else {
            num.to_string()
        }
    }

    if num % 3 == 0 {
        opti_buzz(num)
    } else {
        pessi_buzz(num)
    }
}

fn matchbox_fizzbuzz(num: u32) -> String {
    match num % 3 {
        0 => match num % 5 {
            0 => String::from("FizzBuzz"),
            _ => String::from("Fizz"),
        },
        _ => match num % 5 {
            0 => String::from("Buzz"),
            _ => num.to_string(),
        },
    }
}

fn branchless_fizzbuzz(num: u32) -> String {
    let fizzdex = [1, 0, 0];
    let buzzdex = [2, 0, 0, 0, 0];
    let results = [&num.to_string(), "Fizz", "Buzz", "FizzBuzz"];
    let mut idx = 0;
    idx += fizzdex[(num % 3) as usize];
    idx += buzzdex[(num % 5) as usize];
    results[idx].to_string()
}

fn smartbranchless_fizzbuzz(num: u32) -> String {
    fn fizzbuzz_answer(idx: usize, _num: u32) -> String {
        let results: [&str; 4] = ["", "Fizz", "Buzz", "FizzBuzz"];
        results[idx].to_string()
    }

    fn num_answer(_idx: usize, num: u32) -> String {
        num.to_string()
    }

    let fizzdex = [1, 0, 0];
    let buzzdex = [2, 0, 0, 0, 0];
    let results = [
        num_answer,
        fizzbuzz_answer,
        fizzbuzz_answer,
        fizzbuzz_answer,
    ];
    let mut idx = 0;
    idx += fizzdex[(num % 3) as usize];
    idx += buzzdex[(num % 5) as usize];
    results[idx](idx, num)
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    const TEST_NUMS: [u32; 14] = [3, 9, 12, 5, 10, 20, 15, 30, 45, 1, 2, 7, 11, 22];
    const TEST_ANSWER: [&str; 14] = [
        "Fizz", "Fizz", "Fizz", "Buzz", "Buzz", "Buzz", "FizzBuzz", "FizzBuzz", "FizzBuzz", "1",
        "2", "7", "11", "22",
    ];

    #[test]
    fn test_all_fizzbuzz() {
        for fizzbuzz in ALL_FIZZBUZZS {
            for (idx, num) in TEST_NUMS.iter().enumerate() {
                assert_eq!(fizzbuzz(*num), TEST_ANSWER[idx]);
            }
        }
    }

    #[test]
    fn test_compare_fizzbuzz() {
        for idx in 0..(ALL_FIZZBUZZS.len() - 1) {
            for num in 1..500 {
                assert_eq!(ALL_FIZZBUZZS[idx](num), ALL_FIZZBUZZS[idx + 1](num));
            }
        }
    }
}
