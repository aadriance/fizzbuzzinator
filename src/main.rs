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
const ALL_FIZZBUZZS: [FizzBuzzFn; 3] = [brute_fizzbuzz, accumulate_fizzbuzz, compositional_fizzbuzz];
const ALL_FIZZBUZZ_NAMES: [&str; 3] = ["brute_fizzbuzz", "accumulate_fizzbuzz", "compositional_fizzbuzz"];

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

fn compositional_fizzbuzz(num: u32) -> String {
    fn opti_buzz(num:u32) -> String {
        if num % 5 == 0 {
            String::from("FizzBuzz")
        } else {
            String::from("Fizz")
        }
    }

    fn pessi_buzz(num:u32) -> String {
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

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;


    const TEST_NUMS: [u32; 14] = [3, 9, 12, 5, 10, 20, 15, 30, 45, 1, 2, 7, 11, 22];
    const TEST_ANSWER: [&str; 14] = [
        "Fizz", "Fizz", "Fizz", "Buzz", "Buzz", "Buzz", "FizzBuzz", "FizzBuzz", "FizzBuzz", "1", "2", "7",
        "11", "22",
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
