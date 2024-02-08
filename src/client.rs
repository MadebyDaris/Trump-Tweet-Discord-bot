use std::{error::Error, fs::File, io, process};
use csv::*;
use rand::seq::SliceRandom;

pub struct tweet {
    pub date: String,
    pub contents: String,
    pub link: String
}
impl tweet {
    fn print(self: &Self) {
        println!("date: {}", self.date);
        println!("{}", self.contents);
        println!("src: {}", self.link);

    }
}

pub fn new_tweet(tweet_source: &Vec<String>) -> tweet{
    return tweet {
        date: tweet_source.get(1).unwrap().to_string(), 
        contents: tweet_source.get(4).unwrap().to_string(), 
        link: tweet_source.get(5).unwrap().to_string()}
}

fn read() -> Result<Vec<Vec<String>>>{
    let file = File::open("trump_data.csv")?;
    let mut rdr = Reader::from_reader(file);

    let mut records: Vec<Vec<String>> = Vec::new();

    for result in rdr.records() {
        // Handle errors if any
        let record = result?;
        let record_values: Vec<String> = record.iter().map(|field| field.to_string()).collect();
        // Process each record
        records.push(record_values)
    }
    return Ok(records);
}

pub fn generate() -> tweet {
    let file = read().unwrap();
    let mut rand = file.choose(&mut rand::thread_rng()).unwrap();
    let t = new_tweet(rand);
    return t
}