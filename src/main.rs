use clap::Parser;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::path::Path;
use std::process;
//TODO: add the rest of the field values

#[derive(Default, Parser, Debug)]
struct Arguments {
    calpads: String,
}


#[allow(unused_mut)]
fn check_carrot(rules: &HashMap<String, i32>, path: &Path) -> Result<String, Error> {
    let mut filetype: String; 
    let mut results: String;
    let mut file = File::open(&path)?;
    let mut dirty_buffer = Vec::new();
    file.read_to_end(&mut dirty_buffer)?;
    let mut data = String::from_utf8_lossy(&dirty_buffer);

    //let mut cfile = File::open(&path)?;
    let mut columns;
    //cfile.read_to_string(&mut data)?;

    let lines: Vec<&str> = data.lines().collect();

    // get the filetype from the files first column.
    if let Some(i) = lines[0].get(0..4) {
        filetype = i.to_string();
    } else {
        results = format!(
            "could not read the filetype from {}. The file is out of specification.\n",
            path.file_name().unwrap().to_str().unwrap(),
        );
            return Ok(results)
    }

    for (index, line) in lines.iter().enumerate() {
        columns = line.split('^').count() as i32;
        if columns != *rules.get(&filetype).unwrap() {
            results = format!(
                "{} did not have the correct number of columns on line {}\n
                columns expected: {} columns found: {}\n",
                path.file_name().unwrap().to_str().unwrap(),
                index + 1,
                *rules.get(&filetype).unwrap(),
                columns
            );
            return Ok(results)

        }
    }
    results = format!(
        "{} you entered is valid. number of columns expected for the {} CALPADS file is {}\n",
        path.file_name().unwrap().to_str().unwrap(),
        &filetype,
        rules.get(&filetype).unwrap() + 1
    );

    Ok(results)
}
fn main() -> Result<(), Error> {
    // valid column totals adjust these numbers if any values get updated. the file spec is located
    // here https://www.cde.ca.gov/ds/sp/cl/systemdocs.asp
    let mut column_rules = HashMap::new();
    column_rules.insert("SENR".to_string(), 35);
    column_rules.insert("SINF".to_string(), 50);
    column_rules.insert("SPRG".to_string(), 26);
    column_rules.insert("SELA".to_string(), 16);
    column_rules.insert("SDEM".to_string(), 31);
    column_rules.insert("SASS".to_string(), 21);
    column_rules.insert("CRSE".to_string(), 36);
    column_rules.insert("CRSC".to_string(), 36);
    column_rules.insert("SCSC".to_string(), 21);
    column_rules.insert("SCSE".to_string(), 21);
    column_rules.insert("SCTE".to_string(), 14);
    column_rules.insert("STAS".to_string(), 23);
    column_rules.insert("PSTS".to_string(), 15);
    column_rules.insert("SINC".to_string(), 17);
    column_rules.insert("SIRS".to_string(), 17);
    column_rules.insert("SOFF".to_string(), 15);
    column_rules.insert("WBLR".to_string(), 15);
    column_rules.insert("SWDS".to_string(), 13);
    column_rules.insert("PLAN".to_string(), 33);
    column_rules.insert("MEET".to_string(), 19);
    column_rules.insert("SERV".to_string(), 17);

    //final results of the files that will be printed.
    let mut results: String = "".to_string();
    // get the file or folder from the command line
    let args = Arguments::parse();
    // create a path to run checks before reading any files or directories.
    let filepath: &Path = Path::new(&args.calpads);

    if !filepath.exists() {
        println!("The file or folder path provided does not exist.");
        process::exit(1);
    }

    if filepath.is_dir() {
        for file in filepath
            .read_dir()
            .expect("failed to read the directory given")
        {
            if let Ok(file) = file {
                if !file.path().is_dir() {
                    results.push_str(&check_carrot(&column_rules, &file.path())?);
                }
            }
        }
    } else {
        results = check_carrot(&column_rules, &filepath)?
    }

    println!("{}", results);
    Ok(())
}

/* Pass file / or folder path into program.
 * program determines if its a file or a folder.
 * If the path is a folder it runs the validate function on each entry of the folder.
 * If a file is a subfolder it is ignored.
 * If it is a file then the validate function is called on the single file.
 * file is read into a string.
 * Break string into a vec based on the \r
 * Count the Carrot symbols in each string
 * compare the number of carrots with the expected value for the filetype
 * change the display to show file good or file bad message along with the number of columns expected vs found.
 **/
