use std::collections::HashMap;

#[allow(unused)]
fn main() {
    let mut v: Vec<i32> = vec![100, 25, 50];
    for i in &v {
        print!("{i} ");
    }

    for i in &mut v {
        *i += 50
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:?}", get_wordcounts(&"hello world wonderful world".to_string()));
}
fn get_wordcounts(text:&String) -> HashMap<&str, u32> {
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        *map.entry(word).or_insert(0) += 1;
    };
    map
}