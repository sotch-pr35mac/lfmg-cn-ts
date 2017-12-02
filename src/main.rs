/*
 *  @author         ::  Preston Wang-Stosur-Bassett <http://stosur.info>
 *  @date           ::  Nov 30, 2017
 *  @description    ::  This file is a basic implementation of a Largest First Matching Greedy Algorithm for Chinese text segmentation
*/

extern crate serde;
extern crate serde_json;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::io::stdin;
use std::path::Path;

fn get_cedict() -> HashMap<String, u64> {
    let mut cedict: HashMap<String, u64> = HashMap::<String, u64>::new();

    if Path::new("cedict_map.json").exists() {
        let json_file = File::open("cedict_map.json").expect("There was a problem opening the pre-build cedict json file.");
        println!("Reading dictionary from json file...");
        cedict = serde_json::from_reader(json_file).expect("some error");
    } else {
        let path = Path::new("cc-cedict/");
        for entry in path.read_dir().expect("Could not read directory") {
            if let Ok(entry) = entry {
                let mut file = match File::open(entry.path()) {
                    Ok(file) => file,
                    Err(e) => {
                        panic!("{}", e);
                    }
                };

                println!("Now loading in file {:?}", entry.path());

                let mut contents = String::new();
                file.read_to_string(&mut contents).expect("Failed to read file.");

                for line in contents.lines() {
                    let starting_char = line.chars().nth(0);
                    if starting_char != Some('#') {
                        // Line is not a comment, add the simplified character to the hashmap
                        let information: Vec<&str> = line.split(' ').collect();
                        let simplified: String = information.get(1).unwrap().to_string();
                        cedict.insert(simplified, 0 as u64);
                    }
                }

                let buffer = File::create("cedict_map.json").expect("There was a problem.");
                serde_json::to_writer(buffer, &cedict).expect("Could not write file.");
            }
        }
    }

    return cedict;
}

fn join(segments: Vec<String>) -> String {
    let mut segmented = String::from("");

    for segment in segments {
        if segmented == String::from("") {
            segmented = format!("{}", segment);
        } else {
            segmented = format!("{}/{}", segmented, segment);
        }
    }

    return segmented;
}

fn segment(input: String, cedict: &HashMap<String, u64>) -> String {
    let mut segments: Vec<String> = Vec::<String>::new();

    let mut query: String = input.clone();

    while query.chars().count() > 0 {
        if query.chars().count() >= 4 && cedict.contains_key(&query.chars().take(4).collect::<String>()) {
            //segmented = format!("{}/{}", segmented, query.chars().take(4).collect::<String>());
            segments.push(query.chars().take(4).collect::<String>());
            query = query.chars().skip(4).collect::<String>();
        } else if query.chars().count() >= 3 && cedict.contains_key(&query.chars().take(3).collect::<String>()) {
            //segmented = format!("{}/{}", segmented, query.chars().take(3).collect::<String>());
            segments.push(query.chars().take(3).collect::<String>());
            query = query.chars().skip(3).collect::<String>();
        } else if query.chars().count() >= 2 && cedict.contains_key(&query.chars().take(2).collect::<String>()) {
            //segmented = format!("{}/{}", segmented, query.chars().take(2).collect::<String>());
            segments.push(query.chars().take(2).collect::<String>());
            query =  query.chars().skip(2).collect::<String>();
        } else if query.chars().count() >= 1 && cedict.contains_key(&query.chars().take(1).collect::<String>()) {
            //segmented = format!("{}/{}", segmented, query.chars().take(1).collect::<String>());
            segments.push(query.chars().take(1).collect::<String>());
            query = query.chars().skip(1).collect::<String>();
        } else {
            query = query.chars().skip(1).collect::<String>();
        }
    }

    let segmented: String = join(segments);

    return segmented;
}

fn main() {
    println!("Loading in data from CC-CEDICT file. This may take a while...");

    let cedict: HashMap<String, u64> = get_cedict();
    println!("Finished loading. You can start segmenting now:");

    loop {
        let mut query = String::new();
        println!("Enter a sentence in Simplified Chinese to segment: ");
        stdin().read_line(&mut query).expect("NOT A VALID STRING");
        if let Some('\n') = query.chars().next_back() {
            query.pop();
        }
        if let Some('\r') = query.chars().next_back() {
            query.pop();
        }

        let segmented: String = segment(query, &cedict);

        println!("{}", segmented);
    }
}
