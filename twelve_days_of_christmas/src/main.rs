fn main() {
    let mut cum = String::new();
    for i in 1..13 {
        println!("On the {} day of Christmas my true love sent to me", int_to_str(i));

        let new_line = match i {
            1 => "A partridge in a pear tree\n",
            2 => "Two turtle doves\n",
            3 => "Three French hens\n",
            4 => "Four calling birds\n",
            5 => "Five gold rings\n",
            6 => "Size geese a-laying\n",
            7 => "Seven swans a-swimming\n",
            8 => "Eight maids a-milking\n",
            9 => "Nine ladies dancing\n",
            10 => "Ten lords a-leaping\n",
            11 => "Eleven pipers piping\n",
            12 => "Twelve drummers drumming\n",
            _ => "",
        };
        cum.insert_str(0, new_line);

        println!("{cum}");
        println!("");

        // Remove the capital "A" at the start of "A partridge..."
        // and replace with "And a..."
        if i == 1 {
            cum.remove(0);
            cum.insert_str(0, "And a");
        }
    }
}

fn int_to_str(i: i32) -> String {
    let str = match i {
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "fourth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eighth",
        9 => "nineth",
        10 => "tenth",
        11 => "eleventh",
        12 => "twelth",
        _ => "Not understood!",
    };
    String::from(str)
}