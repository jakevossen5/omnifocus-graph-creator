use std::error::Error;
use std::io;
use std::collections::HashMap;

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


fn get_records() -> Result<Vec<Task>, Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = ReaderBuilder::new()
        .flexible(true)
        .has_headers(false) // Task ID,Type,Name,Status,Project,Context,Start Date,Due Date,Completion Date,Duration,Flagged,Notes,Tags
        .from_reader(io::stdin());
    let records = rdr
        .records()
        .collect::<Result<Vec<StringRecord>, csv::Error>>()?;

    let mut tasks: Vec<Task> = Vec::new();

    for r in records {
        tasks.push(create_task(r))
    }
    // println!("{:?}", records);
    Ok(tasks)
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
    let tasks_result: Result<Vec<Task>, Box<dyn Error>> = get_records();
    let tasks: Vec<Task>;
    if !tasks_result.is_ok() {
        println!("Tasks are not okay!");
        return;
    } else {
        tasks = tasks_result.unwrap();
    }


    let mut days_ago_to_tasks: HashMap<i64, Vec<Task>> = HashMap::new();
    // example time: 2019-06-14 02:54:47 +0000
    for t in tasks {

        // No way around it, this section of code is going to be ugly.
        // Going to manually parse the date by looking at multiplying it by duration to get a number of seconds.

        let mut current_seconds: i64 = 0;

        // println!("{:?}", t);

        if t.completion_date.is_some() {
            let t_completion_date: String = t.completion_date.unwrap();

            // it would be nice to put this in the loop above, but was getting issues with borrowing.
            if t_completion_date == String::from("") {
                break;
            }

            println!("t_completion_date: {}", t_completion_date);
            // let space_seperateed: Vec<&str> = t_completion_date.split_terminator(' ').collect();
            let year: u16 = t_completion_date[..4].to_string().parse::<u16>().unwrap();
            let month: u8 = t_completion_date[5..7].to_string().parse::<u8>().unwrap();
            let day: u8 = t_completion_date[8..10].to_string().parse::<u8>().unwrap();

            let hours: u8 = t_completion_date[11..13].to_string().parse::<u8>().unwrap();
            let mins: u8 = t_completion_date[14..16].to_string().parse::<u8>().unwrap();
            let secs: u8 = t_completion_date[17..19].to_string().parse::<u8>().unwrap();

            // let hour:


            println!("year: {}", year);
            println!("month: {}", month);
            println!("day: {}", day);

            println!("hours {}", hours);
            println!("mins {}", mins);
            println!("secs {}", secs);
            // date_dash_seperated = Vec<&str> = date.spli
        }

    }

}
