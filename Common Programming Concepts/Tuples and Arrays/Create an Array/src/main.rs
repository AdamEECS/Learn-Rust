fn main() {
    let a = [0; 100];

    let output = if a.len() >= 100 {
        "Wow, that's a big array!"
    } else {
        "Meh, I eat arrays like that for breakfast."
    };

    println!("{}", output)
}
