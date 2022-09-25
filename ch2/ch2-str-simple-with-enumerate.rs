fn main(){
    let search_term = "picture";
    let quote = "\
Every face, every shop bedroom window, publlic-house, and
dark square is picture feverishly turnes--in search of what?
It is the same with books.
What do we seek through millions of pages?";

    for (i, line) in quote.lines().enumerate() {
        if line.contains(search_term) {
            let line_num = i + 1 ;
            println!("{}: {}", line_num, line);
        }
    }
}