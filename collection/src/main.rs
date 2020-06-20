fn main() {
    println!("=================================");
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("==> {}", third);
    let third: Option<&i32> = v.get(2);
    println!("==> {:?}", third);
    //============================================
    println!("=================================");
    let v = vec![1, 2, 3, 4, 5];
    //let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
    println!("==> {:?}", does_not_exist);
    //============================================
    println!("=================================");
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    println!("==> {:?}", first);
    v.push(6);
    // if comment in
    // cannot borrow `v` as mutable because it is also borrowed as immutable
    // println!("==> {:?}", first);
    println!("==> {:?}", v);
    //============================================
    println!("=================================");
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
    //============================================
    println!("=================================");
    #[derive(Debug)]
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
    println!("{:?}", row);
    //============================================
    println!("=================================");
    for c in "あいうえお".chars() {
        println!("{}", c);
    }
    for b in "あいうえお".bytes() {
        println!("{}", b);
    }
    //============================================
    println!("=================================");
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);
    //============================================
    println!("=================================");
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
