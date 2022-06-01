#![allow(unused)]
extern crate csv;

use serde_derive::*;

use std::error::Error;
// use std::io;
// use std::process;

use rand::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Qotd {
    pub question: String,
    pub used: bool,
}

impl Qotd {
    pub fn from(question: String) -> Self {
        Qotd {
            question,
            used: false,
        }
    }
    pub fn all_questions() -> Result<Vec<Qotd>, Box<dyn Error>> {
        let mut rdr = csv::Reader::from_path("./qotd.csv")?;
        let mut qotds: Vec<Qotd> = Vec::new();
        for result in rdr.deserialize() {
            // Notice that we need to provide a type hint for automatic
            // deserialization.
            let record: Qotd = result?;
            qotds.push(record);
        }
        Ok(qotds)
    }
    pub fn add_question(question: Qotd) -> Result<(), Box<dyn Error>> {
        #[allow(unused)]
        let mut qotdcsv: Vec<Qotd> = Vec::new();
        match Qotd::all_questions() {
            Ok(vec) => qotdcsv = vec,
            Err(err) => return Err(err),
        };
        qotdcsv.push(question);
        let mut wtr = csv::Writer::from_path("./qotd.csv")?;

        for q in qotdcsv {
            wtr.serialize(q)?;
        }
        wtr.flush()?;
        Ok(())
    }
    pub fn a_question() -> Result<String, Box<dyn Error>> {
        let mut rdr = csv::Reader::from_path("./qotd.csv")?;
        let mut qotds: Vec<Qotd> = Vec::new();
        for result in rdr.deserialize() {
            // Notice that we need to provide a type hint for automatic
            // deserialization.
            let record: Qotd = result?;
            qotds.push(record);
        }

        let mut question = String::new();

        for n in 0..qotds.len() {
            if qotds[n].used == false {
                question = qotds[n].question.clone();
                qotds[n].used = true;
                break;
            }
        }

        if question.is_empty() {
            question = "There are no more quesionts. Please add more!".to_string();
        }

        let mut wtr = csv::Writer::from_path("./qotd.csv")?;

        for q in qotds {
            wtr.serialize(q)?;
        }
        Ok(question)
    }
}
