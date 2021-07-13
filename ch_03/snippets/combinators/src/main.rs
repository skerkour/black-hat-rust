fn main() {}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, iter::FromIterator};

    #[test]
    fn vector() {
        let v = vec![1, 2, 3];

        for x in v.into_iter() {
            println!("{}", x);
        }
    }

    #[test]
    fn hashmap() {
        let mut h = HashMap::new();
        h.insert(String::from("Hello"), String::from("World"));

        for (key, value) in h.iter() {
            println!("{}: {}", key, value);
        }
    }

    #[test]
    fn array() {
        let a = [1, 2, 3];

        for x in a.iter() {
            println!("{}", x);
        }
    }

    #[test]
    fn collect() {
        let x = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10].into_iter();

        let _: Vec<u64> = x.collect();
    }

    #[test]
    fn filter() {
        let v = vec![-1, 2, -3, 4, 5].into_iter();

        let _positive_numbers: Vec<i32> = v.filter(|x: &i32| x.is_positive()).collect();
    }

    #[test]
    fn inspect() {
        let v = vec![-1, 2, -3, 4, 5].into_iter();

        let _positive_numbers: Vec<i32> = v
            .inspect(|x| println!("Before filter: {}", x))
            .filter(|x: &i32| x.is_positive())
            .inspect(|x| println!("After filter: {}", x))
            .collect();
    }

    #[test]
    fn from_iter() {
        let x = vec![(1, 2), (3, 4), (5, 6)].into_iter();

        let _: HashMap<u64, u64> = HashMap::from_iter(x);
    }

    #[test]
    fn reduce() {
        let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10].into_iter();

        let _sum = values.reduce(|acc, x| acc + x);
    }

    #[test]
    fn fold() {
        let values = vec!["Hello", "World", "!"].into_iter();

        let _sentence = values.fold(String::new(), |acc, x| acc + x);
    }

    #[test]
    fn for_each() {
        let v = vec!["Hello", "World", "!"].into_iter();

        v.for_each(|word| {
            println!("{}", word);
        });
    }

    #[test]
    fn map() {
        let v = vec!["Hello", "World", "!"].into_iter();

        let w: Vec<String> = v.map(String::from).collect();
    }

    #[test]
    fn filter_map() {
        let v = vec!["Hello", "World", "!"].into_iter();

        let w: Vec<String> = v
            .filter_map(|x| {
                if x.len() > 2 {
                    Some(String::from(x))
                } else {
                    None
                }
            })
            .collect();

        assert_eq!(w, vec!["Hello".to_string(), "World".to_string()]);
    }

    #[test]
    fn chain() {
        let x = vec![1, 2, 3, 4, 5].into_iter();
        let y = vec![6, 7, 8, 9, 10].into_iter();

        let z: Vec<u64> = x.chain(y).collect();
        assert_eq!(z.len(), 10);
    }

    #[test]
    fn flatten() {
        let x = vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10]].into_iter();

        let z: Vec<u64> = x.flatten().collect();
        assert_eq!(z.len(), 10);
    }

    #[test]
    fn combinators() {
        let a = vec![
            "1",
            "2",
            "-1",
            "4",
            "-4",
            "100",
            "invalid",
            "Not a number",
            "",
        ];

        let _only_positive_numbers: Vec<i64> = a
            .into_iter()
            .filter_map(|x| x.parse::<i64>().ok())
            .filter(|x| x > &0)
            .collect();
    }

    #[test]
    fn option_unwrap_or() {
        let _port = std::env::var("PORT").ok().unwrap_or(String::from("8080"));
    }

    #[test]
    fn result_ok() {
        let _port: Option<String> = std::env::var("PORT").ok();
    }

    #[test]
    fn result_or() {
        let _port: Result<String, std::env::VarError> =
            std::env::var("PORT").or(Ok(String::from("8080")));
    }
}
