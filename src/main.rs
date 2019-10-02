use std::error::Error;
use std::io;
use std::process;

use csv::{ReaderBuilder, StringRecord};

use serde::Deserialize;

#[derive(Clone,Debug, Deserialize)]
struct Task {
    TaskID: String,
    Type: Option<String>,
    Name: Option<String>,
    Status: Option<String>,
    Project: Option<String>,
    Context: Option<String>,
    StartDate: Option<String>,
    DueDate: Option<String>,
    CompletionDate: Option<String>,
    Duration: Option<String>,
    Flagged: Option<u8>,
    Notes: Option<String>,
    Tags: Option<String>
}


fn getRecords() -> Result<(), Box<Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = ReaderBuilder::new()
        .flexible(true)
        .has_headers(false) // Task ID,Type,Name,Status,Project,Context,Start Date,Due Date,Completion Date,Duration,Flagged,Notes,Tags
        .from_reader(io::stdin());
    let records = rdr
        .records()
        .collect::<Result<Vec<StringRecord>, csv::Error>>()?;

    for r in records {
        let task: Task = createTask(r);
        println!("{:?}", task)
    }
    // println!("{:?}", records);
    Ok(())
    // Ok(records)
}

fn createTask(r: StringRecord) -> Task {

    let t: Task = Task{
        TaskID: r[0].to_string(),
        Type: r.get(1).map(|s| s.to_owned()),
        Name: r.get(2).map(|s| s.to_owned()),
        Status: r.get(3).map(|s| s.to_owned()),
        Project: r.get(4).map(|s| s.to_owned()),
        Context: r.get(5).map(|s| s.to_owned()),
        StartDate: r.get(6).map(|s| s.to_owned()),
        DueDate: r.get(7).map(|s| s.to_owned()),
        CompletionDate: r.get(8).map(|s| s.to_owned()),
        Duration: r.get(9).map(|s| s.to_owned()),
        Flagged: r.get(10).map(|s| s.parse().unwrap()),
        Notes: r.get(11).map(|s| s.to_owned()),
        Tags: r.get(12).map(|s| s.to_owned())
    };
    return t;
}

fn main() {
    let records_result = getRecords();
    let records = records_result.unwrap(); // TODO probably want some error handling
    // for r in records {
    //     let task: Task = r.deserialize(None);
    // }
}
