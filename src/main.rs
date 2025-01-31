use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::process;

//TODO: add the rest of the field values

#[derive(Default, Parser, Debug)]
struct Arguments {
    calpads: String,
}

#[allow(unused_mut)]
fn main() {
    // get the file from the command line
    let args = Arguments::parse();
    //println!("{:?}", args);
    let data = fs::read_to_string(args.calpads).expect("unable to read the provided file.");
    // break the file into a vec of lines as strings.
    let lines: Vec<&str> = data.lines().collect();

    // get the filetype from the files first column.
    let mut filetype: String;
    if let Some(i) = lines[0].get(0..4) {
        filetype = i.to_string();
    } else {
        println!("could not read the filetype from the file. The file is out of specification.");
        process::exit(1);
    }

    // valid column totals adjust these numbers if any values get updated. the file spec is located
    // here https://www.cde.ca.gov/ds/sp/cl/systemdocs.asp
    let mut column_rules = HashMap::new();
    column_rules.insert("SELA".to_string(), 15);
    column_rules.insert("SPRG".to_string(), 25);
    column_rules.insert("SENR".to_string(), 34);
    column_rules.insert("SINF".to_string(), 49);
    column_rules.insert("SDEM".to_string(), 30);
    column_rules.insert("SASS".to_string(), 20);
    column_rules.insert("CRSC".to_string(), 35);
    column_rules.insert("CRSE".to_string(), 35);
    column_rules.insert("SCTE".to_string(), 13);
    column_rules.insert("STAS".to_string(), 22);
    column_rules.insert("PSTS".to_string(), 14);
    column_rules.insert("SINC".to_string(), 16);
    column_rules.insert("SIRS".to_string(), 16);
    column_rules.insert("SOFF".to_string(), 14);
    column_rules.insert("WBLR".to_string(), 14);
    column_rules.insert("SWDS".to_string(), 12);
    column_rules.insert("PLAN".to_string(), 32);
    column_rules.insert("MEET".to_string(), 18);
    column_rules.insert("SERV".to_string(), 16);

    let mut columns;
    for (index, line) in lines.iter().enumerate() {
        columns = line.split('^').count() as i32;
        if columns != *column_rules.get(&filetype).unwrap() {
            println!(
                "The file provided did not have the correct number of columns on line {}",
                index + 1
            );
            process::exit(1);
        }
    }
    println!(
            "the CALPADS file you entered is valid. number of columns expected for the {} CALPADS file is {}", 
            &filetype, 
            column_rules.get(&filetype).unwrap() + 1
        );
}

/* Pass file path into program.
 * file is read into a string.
 * Break string into a vec based on the \r
 * Count the Carrot symbols in each string
 * compare the number of carrots with the expected value for the filetype
 * change the display to show file good or file bad message along with the number of columns expected vs found.
 **/
