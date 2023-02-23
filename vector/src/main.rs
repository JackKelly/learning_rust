fn main() {
    let mut v: Vec<i32> = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);

    println!("{:#?}", v);

    let third: &i32 = &v[2];
    println!("Third {}", third);

    let fourth: Option<&i32> = v.get(3);
    match fourth {
        Some(x) => println!("{}", x),
        None => println!("Couldn't get fourth!"),
    }

    for (i, &element) in v.iter().enumerate() {
        println!("{i}: {element}");
    }

    for element in &mut v {
        *element += 5;
    }

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let v2 = vec![
        SpreadsheetCell::Int(5),
        SpreadsheetCell::Float(34.5),
        SpreadsheetCell::Text(String::from("hello!")),
    ];

    for (i, element) in v2.iter().enumerate() {
        println!("{i}={:?}", element);
        match element {
            SpreadsheetCell::Int(n) => println!("Int: {n}"),
            SpreadsheetCell::Float(n) => println!("Float: {n}"),
            SpreadsheetCell::Text(n) => println!("Text: {n}"),
        }
    }

}
