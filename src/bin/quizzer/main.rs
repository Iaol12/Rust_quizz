use std::{env, io};
use quizzer::{load_from_json, Question, QuizData, save_to_json};

fn main() {
    let args: Vec<String> = env::args().collect();
    let option = args.get(1).unwrap();
    start(option)
}

fn start(value: &str){
    if value == "1"{
        question_mode()
    }else if value == "2"{
        quizz_mode()
    }
    else{
        println!("Wrong entry user")
    }
}
fn question_mode(){

    let mut array: Vec<Question> = vec![];
    loop{
        let mut question = String::new();
        println!("Enter question or press E to exit");
        io::stdin().read_line(&mut question).expect("Did not enter a correct string");
        if question.trim() == "E"{ break}
        else{
            let mut option1 = String::new();
            let mut option2 = String::new();
            let mut option3 = String::new();
            let mut option4 = String::new();
            let mut right_answer = String::new();

            println!("Enter option 1");
            io::stdin().read_line(&mut option1).expect("Did not enter a correct string");
            println!("Enter option 2");
            io::stdin().read_line(&mut option2).expect("Did not enter a correct string");
            println!("Enter option 3");
            io::stdin().read_line(&mut option3).expect("Did not enter a correct string");
            println!("Enter option 4");
            io::stdin().read_line(&mut option4).expect("Did not enter a correct string");
            println!("Enter correct option number");
            io::stdin().read_line(&mut right_answer).expect("Did not enter a correct string");



            let questiona = Question{question: question.trim().parse().unwrap(),
                option1: option1.trim().parse().unwrap(), // to robim, lebo na konci bolo ze /r/n
                option2: option2.trim().parse().unwrap(),
                option3: option3.trim().parse().unwrap(),
                option4: option4.trim().parse().unwrap(),
                right_answer: right_answer.trim().parse().unwrap()};
            array.push(questiona)
        }
    }
    let quiz_data = QuizData::new(array);
    if let Err(err) = save_to_json(&quiz_data, "quiz.json") {
        eprintln!("Error saving quiz data: {:?}", err);
    }
}


fn quizz_mode(){


    match load_from_json("quiz.json") {
        Ok(questions) => {
            for question in questions {
                let mut chosen_answer = String::new();
                println!("Odpovedzte na otázku   ");
                println!("{}", question.question);
                println!("1) {}",question.option1);
                println!("2) {}",question.option2);
                println!("3) {}",question.option3);
                println!("4) {}",question.option4);
                println!("Enter correct option number");
                io::stdin().read_line(&mut chosen_answer).expect("Did not enter a correct string");

                if chosen_answer.trim() == question.right_answer {
                    println!("výborne, správne ste odpovedali \n")
                }else{
                    println!("ZLA ODPOVED \n")
                }

            }
        }
        Err(err) => eprintln!("Error reading data from JSON: {}", err),
    }
}