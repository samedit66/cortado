mod tokenizer;
use tokenizer::Tokenizer;

fn main() {
    let mut tokenizer = Tokenizer::new("when a>-b");
    println!("{:?}", tokenizer.next_token());
    println!("{:?}", tokenizer.next_token());
    println!("{:?}", tokenizer.next_token());
    println!("{:?}", tokenizer.next_token());
    println!("{:?}", tokenizer.next_token());
    println!("{:?}", tokenizer.next_token());
    println!("{:?}", tokenizer.next_token());
    println!("{:?}", tokenizer.next_token());
}
