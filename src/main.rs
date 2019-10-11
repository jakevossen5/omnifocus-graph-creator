extern crate time;
use std::error::Error;
use std::io;
use std::collections::HashMap;
use std::collections::HashSet;
use time::Duration;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use plotters::prelude::*;

use csv::{ReaderBuilder, StringRecord};
use chrono::prelude::*;

#[derive(Debug, Hash, Eq, Clone)]
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

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        return self.task_id == other.task_id;
    }
}


fn get_records() -> Result<HashSet<Task>, Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = ReaderBuilder::new()
        .flexible(true)
        .has_headers(true) // Task ID,Type,Name,Status,Project,Context,Start Date,Due Date,Completion Date,Duration,Flagged,Notes,Tags
        .from_reader(io::stdin());
    let records = rdr
        .records()
        .collect::<Result<Vec<StringRecord>, csv::Error>>()?;

    let mut tasks: HashSet<Task> = HashSet::new();

    for r in records {
        tasks.insert(create_task(r));
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
    let tasks_result: Result<HashSet<Task>, Box<dyn Error>> = get_records();
    let tasks: HashSet<Task>;
    if !tasks_result.is_ok() {
        println!("Tasks are not okay!");
        return;
    } else {
        tasks = tasks_result.unwrap();
    }

    // example time: 2019-06-14 02:54:47 +0000
    // Should be this format: %Y-%m-%d %H:%M:%S %z

    println!("tasks size is {}", tasks.len());

    let days_requested: u32 = 7; // TODO: get this value from the user input / command line
    let mut days_ago_to_task_count: HashMap<u32, u32> = HashMap::new();
    days_ago_to_task_count = map_tasks_to_days_ago(tasks);

    let points: Vec<u32> = get_points(days_ago_to_task_count, days_requested);

    let max_point: u32 = get_max_point(&points);

    draw_data(points, max_point, days_requested);
}

fn get_points(map: HashMap<u32, u32>, days_requested: u32) -> Vec<u32> {
    let mut points: Vec<u32> = Vec::new();
    for x in 0..days_requested {
        for j in 0..(*map.get(&(x as u32)).unwrap_or(&0)) {
            points.push(x as u32);
        }
    }
    return points;
}

fn draw_data(points: Vec<u32>, max_point: u32, days_ago: u32) -> Result<(), Box<dyn std::error::Error>> {
        let root =
        BitMapBackend::new("stats.png", (640, 480)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        // .caption("Histogram Test", ("Arial", 50.0).into_font()) // TODO, custom title
        .build_ranged(0u32..days_ago, 0u32..max_point)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .line_style_1(&WHITE.mix(0.3))
        .x_label_offset(30)
        .y_desc("Tasks Completed")
        .x_desc("Days Ago")
        .axis_desc_style(("Arial", 15).into_font())
        .draw()?;

    let data = points;

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(BLUE.mix(1.0).filled())
            .data(data.iter().map(|x: &u32| (*x, 1))),
    )?;

    Ok(())
}


fn map_tasks_to_days_ago(tasks: HashSet<Task>) -> HashMap<u32, u32> {
    let mut map: HashMap<u32, u32> = HashMap::new();
    let utc_now: DateTime<Utc> = Utc::now();
    println!("tasks size in map fn is {}", tasks.len());
    for t in tasks.iter() {
        if t.completion_date.is_some() {

            // let t_completion_date: &mut String = &mut t.completion_date.unwrap();
            let t_completion_date: String = t.completion_date.as_ref().to_owned().unwrap().to_string();
            // it would be nice to put this in the loop above, but was getting issues with borrowing.
            if *t_completion_date != String::from("") {

                // date_dash_seperated = Vec<&str> = date.spli

                let dt_result = DateTime::parse_from_str(&t_completion_date, "%Y-%m-%d %H:%M:%S %z");
                let dt: DateTime<FixedOffset>;
                if dt_result.is_err() {
                    println!("there was an error parsing the completed date: {:?} ", dt_result.err());
                    break;
                } else {
                    dt = dt_result.unwrap();
                }
                // println!("dt       : {:?}", t_completion_date);
                // println!("dt object: {:?}", dt);

                // Currently requires the time package, but DateTime said they are going to move to std time soon
                let mut time_diff: std::time::Duration = utc_now.signed_duration_since(dt).to_std().unwrap();
                // println!("Time diff: {:?}" , time_diff);

                // 86400 seconds in a day

                let mut days_ago_counter: u32 = 0;

                while time_diff > std::time::Duration::from_secs(86400) {
                    time_diff = time_diff - std::time::Duration::from_secs(86400);
                    days_ago_counter = days_ago_counter + 1;
                }

                let days_ago: u32 = days_ago_counter;

                if !map.contains_key(&days_ago) {
                    map.insert(days_ago, 0);
                }
                let current_count: &mut u32 = map.get_mut(&days_ago).unwrap();
                *current_count += 1;
            }
        }
    }
    return map;
}
