
struct Point {
    x: i32,
    y: i32,
  }
  fn main() {
    let mut p = Point { x: 1, y: 2 };
    let x = &mut p.x;
    let y = &mut p.y;
    *x += 1;
    *y += 1;
    println!("{} {}", p.x, p.y);
  }


fn get_first(strs: &mut (String, String)) -> &mut String {
    &mut strs.0
}

fn get_second(strs: &mut (String, String)) -> &mut String {
    &mut strs.1
}

fn transfer_string(strs: &mut (String, String)) {
    let fst = &mut strs.0;
    let snd = &mut strs.1;
    fst.push_str(snd);
    snd.clear();
}