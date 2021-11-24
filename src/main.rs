use std::env;

use rand::{thread_rng, Rng};

struct Lotto {
    take: usize,
    from: usize,
    numbers: Vec<usize>,
}

impl Lotto {
    fn new(take: usize, from: usize) -> Self {
        let mut rng = thread_rng();
        Self {
            take,
            from,
            numbers: (0..take).map(|_| rng.gen_range(1..=from)).collect(),
        }
    }
    // compiler warns, since he doesn't check the test cases for usages
    // left the usages in there since you shouldn't edit test cases usually
    #[allow(dead_code)]
    fn get_numbers(self) -> Vec<usize> {
        self.numbers
    }
}

fn format_lotto_results(lotto: &Lotto) -> String {
    format!(
        "{take} of {from}: {:?}",
        numbers = lotto.numbers,
        take = lotto.take,
        from = lotto.from
    )
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // check for correct number of arguments.
    // first arg is program name, after that there should be pairs of {take, from}
    let lotto_numbers: Vec<Lotto> = if args.len() >= 3 && (args.len() - 1) % 2 == 0 {
        args[1..]
            .chunks(2)
            .map(|x| {
                Lotto::new(
                    x[0].parse().expect("Argument was not a number"),
                    x[1].parse().expect("Argument was not a number"),
                )
            })
            .collect()
    } else {
        println!("Entered an invalid number of args");
        Vec::new()
    };

    for draw in lotto_numbers {
        println!("{}", format_lotto_results(&draw));
    }
}

#[test]
fn test_format_lotto_results() {
    let lotto = Lotto {
        take: 6,
        from: 45,
        numbers: vec![2, 3, 10, 25, 30, 40],
    };

    assert_eq!(
        "6 of 45: [2, 3, 10, 25, 30, 40]",
        format_lotto_results(&lotto)
    );
}

#[test]
fn test_lotto_constructor() {
    let lotto = Lotto::new(6, 45);

    let numbers = lotto.get_numbers();

    assert_eq!(numbers.len(), 6);
}

#[test]
fn test_lotto_constructor_uniques() {
    use std::collections::HashSet;
    let lotto = Lotto::new(6, 45);

    let numbers = lotto.get_numbers();
    let set: HashSet<usize> = numbers.into_iter().collect();

    assert_eq!(set.len(), 6);
}
