pub fn build_proverb(list: &[&str]) -> String {
    match list.len(){
        0 => String::from(""),
        _ => {
            let p1 = list.windows(2)
                .map(|word| format!("For want of a {} the {} was lost.",
                                    word[0],word[1])).collect::<Vec<String>>()
                .join("\n");
            let p2 = format!("And all for the want of a {}.",list[0]);
            let spacer = match list.len(){
                1 => "",
                _ => "\n",
            };
            format!("{}{}{}", p1,spacer,p2)
        }
    }
}
