fn main() {
    // 1. String Literal
    let _speech = "\"Ouch!\" said the well.\n";

    // String may span multiple lines.
    println!(
        "In the room the women come and go,
        Singing of Mount Abora"
    );

    // If one line of a string ends with a backslash,
    // then the newline character and the leading whitespace on the next line are dropped:
    println!(
        "It was a bright, cold day in April, and \
        there were four of us—\
        more or less."
    );

    // Raw strings
    let _default_win_install_path = r"C:\Program Files\Gorillas";
    println!(
        r###"
        This raw string started with 'r###"'.
        Therefore it does not end until we reach a quote mark ('"')
        followed immediately by three pound signs ('###'):
    "###
    );

    // 2. Byte String
    let method = b"GET";
    assert_eq!(method, &[b'G', b'E', b'T']);

    // 3. String
    // Converts a &str to a String
    let _error_message = "too many pets".to_string();

    // Works just like println!(), except it returns a String instead of writing to stdout.
    assert_eq!(
        format!("{}°{:02}′{:02}″N", 24, 5, 23),
        "24°05′23″N".to_string()
    );

    // Concatenate strings
    let bits = vec!["veni", "vidi", "vici"];
    assert_eq!(bits.concat(), "venividivici");
    assert_eq!(bits.join(", "), "veni, vidi, vici");

    // Various string methods
    assert!("ONE".to_lowercase() == "one");

    assert!("peanut".contains("nut"));
    assert_eq!("ಠ_ಠ".replace("ಠ", "■"), "■_■");
    assert_eq!(" clean\n".trim(), "clean");

    for word in "veni, vidi, vici".split(", ") {
        assert!(word.starts_with("v"));
    }
}
