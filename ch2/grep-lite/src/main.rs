use regex::Regex;

fn main(){
    let re = Regex::new("picture").unwrap();
    
    let quote = "Every face, every shop bedroom window, publlic-house, and
dark square is picture feverishly turnes--in search of what?
It is the same with books.What do we seek through millions of pages?";
    for line in quote.lines() {
        let contains_substring = re.find(line);
        match contains_substring {
            Some(_) => println!("{}",line),
            None => (),
        }
    }
}