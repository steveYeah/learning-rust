fn main() {
    basic_example();
    consuming_adapter_example();
    iterator_adapter_example();
    iterators_and_closures();
}

fn basic_example() {
    let v1 = vec![1, 2, 3];
    // Create store iterator for example only
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    // Normal use
    let v2 = vec![1, 2, 3];

    for val in v2.iter() {
        println!("V2 Got: {}", val);
    }
}

fn consuming_adapter_example() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    // we manually call next until the iterator is fully consumed
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    // Methods that call next are called consuming adapters
    // In the code below, sum() calls next one the iterator until it is fully consumed and is an
    // example of a consuming adapter
    // sum() takes ownership of the iterator s1jo you cannot use it after the call
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}

fn iterator_adapter_example() {
    // methods that produce other iterators

    // map iterates through the iterator and applies the closure to each element. The result is
    // another iterator
    let v1: Vec<i32> = vec![1, 2, 3];
    // Iterators are lazy, so if they are not consumed the compiler will mark that as a warning
    //v1.iter().map(|x| x + 1);

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}

fn iterators_and_closures() {
    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }

    fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        // size is captured from the environment in the closure
        // filter is an interator adapter as it returns an iterator
        // we then use the consuming adapter, collect(), to consume the iterator to produce a
        // vector
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }

    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
        Shoe {
            size: 13,
            style: String::from("sandle"),
        },
        Shoe {
            size: 10,
            style: String::from("Boot"),
        },
    ];

    let in_my_size = shoes_in_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("boot")
            },
            Shoe {
                size: 10,
                style: String::from("Boot")
            }
        ]
    );
}
