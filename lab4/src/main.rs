use reqwest::StatusCode;
use regex::Regex;
use std::collections::HashSet;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Network error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Wrong HTTP status code: {0}")]
    BadHttpResult(u16),
    #[error("I/O error: {0}")]
    IOError(#[from] std::io::Error),
}

// impl From<reqwest::Error> for Error {
//     fn from(e: reqwest::Error) -> Error {
//         // Encapsulate the error into a Error::Reqwest variant
//         Error::Reqwest(e)
//     }
// }

// impl From<std::io::Error> for Error {
//     fn from(e: std::io::Error) -> Error {
//         Error::IOError(e)
//     }
// }

fn get(url: &str) -> Result<String, Error> {
    // when only string is returned:
    // reqwest::blocking::get(url).unwrap().text().unwrap()

    let response = reqwest::blocking::get(url)?;
    let status = response.status();

    // if !status.is_success() {
    //     // we have to return a result
    //     // in case of error result is returned via Err(...)
    //     return Err(Error::BadHttpResult(status.as_u16()));
    // }

    println!("status code: {}", status.as_u16());

    // match response.text() {
    //     Ok(page) => Ok(page),
    //     Err(e) => Err(Error::from(e)),
    // } 

    match status {
        StatusCode::OK => {
            match response.text() {
                Ok(page) => return Ok(page),
                // we need to convert reqwest::Error to our Error type
                Err(e) => return Err(Error::from(e)),
            }
        },
        _ => return Err(Error::BadHttpResult(status.as_u16()))
    }
}

// fn extract_names(s: &str) -> Vec<String> {
//     let re = Regex::new(r#"\p{Uppercase}\p{Lowercase}+ \p{Uppercase}\p{Lowercase}+"#).unwrap();
    
//     let mut names = vec![];
//     for matched in re.find_iter(s) {
//         names.push(matched.as_str().to_owned());
//     }
    
//     return names;
// }

fn extract_names(s: &str) -> HashSet<String> {
    let re = Regex::new(r#"\p{Uppercase}\p{Lowercase}+ \p{Uppercase}\p{Lowercase}+"#).unwrap();
    
    let mut names = HashSet::new();
    for matched in re.find_iter(s) {
        names.insert(matched.as_str().to_owned());
    }
    
    return names;
}

fn get_names(url: &str) -> Result<HashSet<String>, Error> {
    let page = get(url)?;
    let names = extract_names(page.as_str());
    Ok(names)
}

fn trending() -> Result<Vec<String>, Error> {
    let names1 = get_names("https://www.liberation.fr")?;
    let names2 = get_names("https://www.mediapart.fr")?;

    let mut vector = vec![];
    for name in names1.intersection(&names2) {
        vector.push(name.as_str().to_owned());
    }
    vector.sort();
    Ok(vector)
}

fn write_to_disk(path: &str, data: Vec<String>) -> Result<(), Error> {
    let content = data.join("\n");
    match std::fs::write(path, content) {
        Ok(x) => Ok(x),
        Err(e) => Err(From::from(e))
    }
}

fn main() -> Result<(), Error> {
    // get the webpage
    // println!("{}", get("https://rfc1149.net/kk")?);
    let page: String;
    match get("https://rfc1149.net/jeje") {
        Ok(p) => {
            page = p;
            // println!("{}", page);
            
        },
        Err(e) => println!("Error: {}", e)
    }

    // regex and extracting names to a hashset
    let names = extract_names("Yesterday, John Doe met François Grüß and John Doe in a tavern");
    println!("{names:?}");

    // returning the names in the webpage
    // names = get_names("https://www.liberation.fr")?;
    // println!("NAMES: {names:?}");

    // set intersection and file system
    println!("Trending names:");
    let trending_names = trending()?;
    for name in trending_names.clone() {
        println!("  - {name}");
    }
    // let _ = write_to_disk("/trending.txt", trending_names);
    match write_to_disk("./trending.txt", trending_names) {
        Ok(_) => println!("Wrote to disk successfully"),
        Err(e) => println!("Error: {}", e)
    }

    println!("main finished successfully");
    // if main succeeded return ok with no data
    Ok(())
}
