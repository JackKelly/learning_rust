fn main() {
    println!("Hello, world!");

    let mut v: Vec<String> = (0..10).map(|i| i.to_string()).collect();
    println!("{:?}", v);
    
    let a = ["hello".to_owned(), "there".to_owned()];
    extend(&mut v, &a);
    println!("v={:?}", v);
    println!("a={:?}", a);


    let mut t = (1, 2);
    let rt = &mut t;
    let rt0 = &rt.0;
    let rt1 = &mut rt.1;
    println!("{:?} {:?}", rt0, rt1);
}

fn extend(v: &mut Vec<String>, s: &[String]) {
    for element in s {
        v.push(element.clone());
    }
}