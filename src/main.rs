#[macro_use]
extern crate serde_json;

use std::collections::HashMap;

#[derive(Debug)]
struct Hoge {
    a: String,
    b: u32,
}

fn main() {
    let mut john = json!({
        "name": "hoge",
        "age": 2 + 1,
        "phones": [
            format!("+44 {}", 1)
        ]
    });

    let xs = vec![
        Hoge {a: "a".to_owned(), b: 0},
        Hoge {a: "aaa".to_owned(), b: 0},
        Hoge {a: "a".to_owned(), b: 0},
        Hoge {a: "aaa".to_owned(), b: 0},
        Hoge {a: "aa".to_owned(), b: 0},
        ];

    let h = xs.iter().fold(HashMap::<String, Vec<u32>>::new(), |mut acc, x| {
        acc.entry(x.a.clone()).or_insert(Vec::new()).push(x.b);
        acc 
    });

    println!("{:?}", h);

    john.as_object_mut().unwrap().insert("hoge".to_owned(), json!(1));
    println!("{:}", john);
}
