use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

fn load_file(name: &str) -> Result<Vec<String>, io::Error> {
    io::BufReader::new(File::open(name)?).lines().collect()
}

fn count_chars<S: AsRef<str> + Send>(input: &[S]) -> HashMap<char, usize> {
    let mut map = HashMap::new();

    for string in input {
        for c in string.as_ref().chars() {
            // Entry API is used to insert or update the count
            let counter = map.entry(c).or_insert(0);
            *counter += 1;
        }
    }

    map
}

fn count_chars_parallel<S: AsRef<str> + Send + Sync>(input: &[S], n: usize) -> HashMap<char, usize> {
    let (tx, rx) = mpsc::channel::<HashMap<char, usize>>();
    let input_chunks = input.chunks(n);

    let tx = Arc::new(tx);

    thread::scope(|s| {
        for chunk in input_chunks {
            let tx = Arc::clone(&tx);

            s.spawn(move|| {
                let result = count_chars(chunk);
                tx.send(result).expect("Failed to send result on channel");
            });
        }
    });

    drop(tx); // Drop the last sender to signal that no more data will be sent

    let mut freq = HashMap::default();
    while let Ok(fr) = rx.recv() {
        for (c, n) in fr {
            *freq.entry(c).or_default() += n;
        }
    }

    freq
}


fn main() -> Result<(), io::Error> {
    let text = load_file("./text.txt")?;
    let mut occurences = count_chars_parallel(&text, 4);
    println!("{occurences:?}");
    occurences = count_chars(&text);
    println!("{occurences:?}");

    Ok(())
}
