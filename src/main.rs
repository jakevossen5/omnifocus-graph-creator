use std::error::Error;
use std::io;

use csv::{ReaderBuilder, StringRecord};

#[derive(Debug)]
struct Task {
    task_id: String,
    type_of_task: Option<String>,
    name: Option<String>,
    status: Option<String>,
    project: Option<String>,
    context: Option<String>,
    start_date: Option<String>,
    due_date: Option<String>,
    completion_date: Option<String>,
    duration: Option<String>,
    flagged: Option<u8>,
    notes: Option<String>,
    tags: Option<String>
}


fn get_records() -> Result<(), Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = ReaderBuilder::new()
        .flexible(true)
        .has_headers(false) // Task ID,Type,Name,Status,Project,Context,Start Date,Due Date,Completion Date,Duration,Flagged,Notes,Tags
        .from_reader(io::stdin());
    let records = rdr
        .records()
        .collect::<Result<Vec<StringRecord>, csv::Error>>()?;

    for r in records {
        let task: Task = create_task(r);
        println!("{:?}", task)
    }
    // println!("{:?}", records);
    Ok(())
    // Ok(records)
}

fn create_task(r: StringRecord) -> Task {

    let t: Task = Task{
        task_id: r[0].to_string(),
        type_of_task: r.get(1).map(|s| s.to_owned()),
        name: r.get(2).map(|s| s.to_owned()),
        status: r.get(3).map(|s| s.to_owned()),
        project: r.get(4).map(|s| s.to_owned()),
        context: r.get(5).map(|s| s.to_owned()),
        start_date: r.get(6).map(|s| s.to_owned()),
        due_date: r.get(7).map(|s| s.to_owned()),
        completion_date: r.get(8).map(|s| s.to_owned()),
        duration: r.get(9).map(|s| s.to_owned()),
        flagged: r.get(10).map(|s| s.parse().unwrap()),
        notes: r.get(11).map(|s| s.to_owned()),
        tags: r.get(12).map(|s| s.to_owned())
    };
    return t;
}

fn main() {
    get_records();
}
