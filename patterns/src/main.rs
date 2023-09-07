struct Track {
    album: String,
    track_number: i8,
    title: String,
    artist: String,
}

fn main() {

    for x in 0..5 {
        let a = match x {
            n if n > 2 => n,
            _ => 100,
        };

        println!("x={x}, a={a}");
    }

    // Unpack a struct into three new local variables.
    let song = Track{
        album: "Foo".to_string(),
        track_number: 23,
        title: "Bar".to_string(),
        artist: "Baz".to_string()
    };

    let Track{album, track_number, title, ..} = song;
    println!("{album}, {track_number}, {title}");
}
