use std::time::Instant;

fn main() {
    let mut results: Vec<Vec<f64>> = Vec::new();
    for _fizzbuzz in ALL_FIZZBUZZS {
        results.push(Vec::new());
    }

    for _iteration in 0..100 {
        for (idx, fizzbuzz) in ALL_FIZZBUZZS.iter().enumerate() {
            let current = Instant::now();
            for num in 0..10_000_000 {
                fizzbuzz(num);
            }

            let duration = current.elapsed();
            results[idx].push(duration.as_secs_f64());
        }
    }

    for (idx, name) in ALL_FIZZBUZZ_NAMES.iter().enumerate() {
        print!("{}", name);
        for val in &results[idx] {
            print!(",{:?}", val);
        }

        println!();
    }
}

type FizzBuzzFn = fn(u32) -> String;
const ALL_FIZZBUZZS: [FizzBuzzFn; 11] = [
    brute_fizzbuzz,
    worsebrute_fizzbuzz,
    accumulate_fizzbuzz,
    compositional_fizzbuzz,
    bitaccumulate_fizzbuzz,
    matchbox_fizzbuzz,
    branchless_fizzbuzz,
    smartbranchless_fizzbuzz,
    branchful_fizzbuzz,
    doof_fizzbuzz,
    tuple_match_fizzbuzz,
];

const ALL_FIZZBUZZ_NAMES: [&str; 11] = [
    "brute_fizzbuzz",
    "worsebrute_fizzbuzz",
    "accumulate_fizzbuzz",
    "compositional_fizzbuzz",
    "bitaccumulate_fizzbuzz",
    "matchbox_fizzbuzz",
    "branchless_fizzbuzz",
    "smartbranchless_fizzbuzz",
    "branchful_fizzbuzz",
    "doof_fizzbuzz",
    "tuple_match_fizzbuzz",
];

/// brute_fizzbuzz is a brute force solution that does what is effectivley a
/// linear search of the solution space by manually checking all possible
/// outcomes. This will incure anywhere from 1-4 checks.
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

/// worsebrute_fizzbuzz is a strategically worse fizzbuzz. Since everythird
/// number is divisible by three but every fifth number is divisible by four
/// this will cause an increase in the number of times 3 checks are required.
fn worsebrute_fizzbuzz(num: u32) -> String {
    if num % 15 == 0 {
        String::from("FizzBuzz")
    } else if num % 5 == 0 {
        String::from("Buzz")
    } else if num % 3 == 0 {
        String::from("Fizz")
    } else {
        num.to_string()
    }
}

/// accumulate_fizzbuzz trys to be clever by recognizing divible by 15 can be
/// skipped if you accumulate the result of the 3 & 5 check. It suffers from
/// the speed of string operations however.
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

/// bitaccumulate_fizzbuzz trys to sidestep the performance issues of
/// accumulate_fizzbuzz by using clever indexing to keep the combined result in
/// a number and index into an array of answers.
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

/// compositional_fizzbuzz recognizes that all problems can be solved by
/// adding some new functions. Instead of a linear search of the problem space
/// it is a binary tree search. After determining if the value is divisible by
/// three there are two possible outcomes. Each outcome is encoded as it's own
/// functions (optimistic when divisible by three, pessimistic when not). This
/// implementation will always make 2 checks. No more no less.
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

/// matchbox_fizzbuzz function is logically equivalent to compositional_fizzbuzz
/// It uses nested match statements to acheive the same goal of always making 2
/// checks.
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

/// branchful_fizzbuzz takes the indexing idea from bitaccumulate_fizzbuzz and
/// turns it up to the max. fizzdex contains three elements (index 0-2) which
/// covers all possible outputs of x % 3. Likewise for buzzdex. Indexing into
/// the arrays gives the result needed for the accumulation instead of using an
/// if statement to decide when to add. It suffers from always having to convert
/// the input into a string.
fn branchless_fizzbuzz(num: u32) -> String {
    let fizzdex = [1, 0, 0];
    let buzzdex = [2, 0, 0, 0, 0];
    let results = [&num.to_string(), "Fizz", "Buzz", "FizzBuzz"];
    let mut idx = 0;
    idx += fizzdex[(num % 3) as usize];
    idx += buzzdex[(num % 5) as usize];
    results[idx].to_string()
}

/// smartbranchless_fizzbuzz takes the ideas from branchless_fizzbuzz and
/// combines them with the function calling of compositional_fizzbuzz. This
/// allows the function to only pat the cost of converting the input when it's
/// required.
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

/// branchful_fizzbuzz decides trys to take the ideas of branchless_fizzbuzz and
/// insead of being overly clever just uses an if.
fn branchful_fizzbuzz(num: u32) -> String {
    let results: [&str; 4] = ["", "Fizz", "Buzz", "FizzBuzz"];
    let fizzdex = [1, 0, 0];
    let buzzdex = [2, 0, 0, 0, 0];
    let mut idx = 0;
    idx += fizzdex[(num % 3) as usize];
    idx += buzzdex[(num % 5) as usize];
    if idx == 0 {
        num.to_string()
    } else {
        results[idx].to_string()
    }
}

/// doof_fizzbuzz is in honor of that time as a freshman when I forgot the mod
/// operator existed and I ended up defining one from scratch.
fn doof_fizzbuzz(num: u32) -> String {
    fn doof_mod(num: u32, check: u32) -> bool {
        let div = num / check;
        (div * check) == num
    }

    if doof_mod(num, 15) {
        String::from("FizzBuzz")
    } else if doof_mod(num, 3) {
        String::from("Fizz")
    } else if doof_mod(num, 5) {
        String::from("Buzz")
    } else {
        num.to_string()
    }
}

/// tuple_match_fizzbuzz uses tuple matching to optimize the computation
fn tuple_match_fizzbuzz(num: u32) -> String {
    match (num % 3, num % 5) {
        (0, 0) => "FizzBuzz".into(),
        (0, _) => "Fizz".into(),
        (_, 0) => "Buzz".into(),
        _  => num.to_string()
    }
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
