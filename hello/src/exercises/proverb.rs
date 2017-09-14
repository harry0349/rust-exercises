pub fn build_proverb(pieces :Vec<&str>) -> String {
    if pieces.len() == 0 {
        return String::new();
    }
    let mut out: Vec<String> = vec![];
    for i in 1..pieces.len() {
        out.push(format!("For want of a {} the {} was lost.", pieces[i-1], pieces[i]));
    }
    let end: String;
    if pieces.len() > 2 {
        end = format!("{} {} {}", pieces[2], pieces[1], pieces[0]);
    } else {
        end = format!("{}", pieces[0]);
    }
    out.push(format!("And all for the want of a {}.", end));
    out.join("\n")
}