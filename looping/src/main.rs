use std::io;

fn main() {
    let riddle = String::from("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");
    let answer = String::from("The letter e");
    let mut usr_answer = String::new();
    let mut attempts = 0;

    while usr_answer.trim() != answer {
        println!("{}", riddle);
        usr_answer.clear();
        io::stdin().read_line(&mut usr_answer).expect("Failed to read line");
        attempts += 1;
    }

    println!("Number of trials: {}", attempts);
}
