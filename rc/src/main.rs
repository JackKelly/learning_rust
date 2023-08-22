use std::rc::Rc;

fn main() {
    let v = vec![1,2,3];
    let rc_a = Rc::new(v);
    println!("Reference count of rc_a = {}", Rc::strong_count(&rc_a));

    {
        let rc_b = Rc::clone(&rc_a);
        println!("Reference count of rc_a = {}", Rc::strong_count(&rc_a));
        println!("Reference count of rc_b = {}", Rc::strong_count(&rc_b));
    }
    println!("Reference count of rc_a = {}", Rc::strong_count(&rc_a));

}
