pub fn test() {
    println!("[ITERATORS] Start");

    lazy();
    let v = vec![1, 2, 3, 4];
    let a: Vec<_> = v
        .iter()
        .filter(|x: &&i32| *x % 2 == 0)
        .map(|x: &i32| x * 2)
        .collect();

    println!("a={a:?}");

    let shoe = Shoe {
        size: 9,
        style: "Boxing Shoe".to_string(),
    };

    let results = shoes_in_size(vec![shoe], 9);

    println!("my shoes: {results:?}");

    println!("[ITERATORS] End");
}

fn lazy() {
    let v = vec![1, 2, 3];

    let v_iter = v.iter();

    println!("v_iter is lazy: {v_iter:?}");

    for val in v_iter {
        println!("Got: {val}");
    }
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes
        .into_iter()
        .filter(|shoe| shoe.size == shoe_size)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterator_demo() {
        let v = vec![1, 2, 3];

        let mut v_iter = v.iter();
        assert_eq!(v_iter.next(), Some(&1));
        assert_eq!(v_iter.next(), Some(&2));
        assert_eq!(v_iter.next(), Some(&3));
        assert_eq!(v_iter.next(), None);
    }

    #[test]
    fn iterator_sum() {
        // Consuming adapters
        let v = vec![1, 2, 3];
        let v_iter = v.iter();
        let total: i32 = v_iter.sum();
        assert_eq!(total, 6);
    }

    #[test]
    fn iterator_adapter() {
        let v = vec![1, 2, 3];
        let v_iter = v.iter();
        let v2: Vec<_> = v_iter.map(|x| x + 1).collect();
        assert_eq!(vec![2, 3, 4], v2);
    }

    #[test]
    fn filter_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}
