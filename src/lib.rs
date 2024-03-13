use std::fs::File;
use std::io::{Read, Write};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Question{
     pub question: String,
     pub option1: String,
     pub option2: String,
     pub option3: String,
     pub option4: String,
     pub right_answer: String
}

// TODO: prerob to aby to pouzivalo tento sposob... co som videl tak takto sa to ma po spravnosti robit
// impl Question{
//     pub fn new(question:&str,answer1:&str,answer2:&str,answer3:&str,answer4:&str,right_answer:i8) -> Self{
//         Question{
//             question: question.to_string(),
//             option1: answer1.to_string(),
//             option2: answer2.to_string(),
//             option3: answer3.to_string(),
//             option4: answer4.to_string(),
//             right_answer: right_answer,
//         }
//     }
// }
#[derive(Serialize, Deserialize)]
pub struct QuizData {
    questions: Vec<Question>,
}

impl QuizData {
    pub fn new(questions: Vec<Question>) -> Self {
        QuizData { questions }
    }
}

pub fn save_to_json(data: &QuizData, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json_data = serde_json::to_string_pretty(data)?;

    let mut file = File::create(file_path)?;
    file.write_all(json_data.as_bytes())?;

    Ok(())
}

pub fn load_from_json(file_path: &str) -> Result<Vec<Question>, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut json_data = String::new();
    file.read_to_string(&mut json_data)?;

    let quiz_data: QuizData = serde_json::from_str(&json_data)?;
    let questions = quiz_data.questions;
    Ok(questions)
}