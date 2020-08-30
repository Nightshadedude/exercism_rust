pub fn reply(message: &str) -> &str {
    println!("{}", message);
    let message = message.trim();
    let question = match message.chars().last() {
        Some('?') => true,
        _ => false,
    };
    let non_ascii = match message {
        _ if message.to_uppercase() == message.to_lowercase() => true,
        _ => false,
    };
    match message {
        _ if message.len() == 0 => "Fine. Be that way!",
        _ if question && non_ascii => "Sure.",
        _ if non_ascii => "Whatever.",
        _ if question &&
            message == message.to_uppercase()=>
            "Calm down, I know what I'm doing!",
        _ if question => "Sure.",
        _ if message == message.to_uppercase() => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
