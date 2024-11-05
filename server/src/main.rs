use std::{
    cmp::Ordering,
    fs::File,
    path::{Path, PathBuf},
};

use chrono::{DateTime, Datelike, NaiveDate, TimeDelta, Utc};
use serde::Deserialize;
use tokio::signal;
mod http_api;

#[derive(Eq, Debug, Deserialize)]
struct Person {
    #[serde(deserialize_with = "parse_date")]
    pub birthday: NaiveDate,
    pub name: String,
}

// Custom deserialization function for 'birthday'
fn parse_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: serde::Deserializer<'de>,
{
    // Deserialize the input into a string
    let s: String = Deserialize::deserialize(deserializer)?;
    // Parse the string into a NaiveDate with the format "yyyy-mm-dd"
    NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(serde::de::Error::custom)
}

impl Person {
    pub fn create(name: String, birth_year: i32, birth_month: u32, birth_day: u32) -> Self {
        let birthday = NaiveDate::from_ymd_opt(birth_year, birth_month, birth_day).unwrap();
        Person { name, birthday }
    }

    pub fn get_next_birthday(&self) -> (NaiveDate, String) {
        let current_date_naive = Utc::now().date_naive();

        let age = current_date_naive.years_since(self.birthday).unwrap();

        let brithday_this_year = self
            .birthday
            .with_year(current_date_naive.year_ce().1 as i32)
            .unwrap();

        let mut next_birthday = brithday_this_year.clone();

        if current_date_naive - brithday_this_year >= TimeDelta::try_days(0).unwrap() {
            next_birthday = brithday_this_year
                .with_year(current_date_naive.year_ce().1 as i32 + 1)
                .unwrap();
        }

        // println!("Current Age: {:?}", age);
        // println!("Next birthday is {:#?}", next_birthday);
        // println!("{:?} - {} ({})", next_birthday, self.name, age + 1);

        (next_birthday, format!("{} ({})", self.name, age + 1))
    }
}

impl Ord for Person {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_next_birthday().0.cmp(&other.get_next_birthday().0)
    }
}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.get_next_birthday().0 == other.get_next_birthday().0
    }
}

// https://users.rust-lang.org/t/function-to-list-files-in-directories-and-in-subdirectories/46236/3
pub fn _list_files(vec: &mut Vec<PathBuf>, path: &Path) -> std::io::Result<()> {
    if std::fs::metadata(&path)?.is_dir() {
        let paths = std::fs::read_dir(&path)?;
        for path_result in paths {
            let full_path = path_result?.path();
            if std::fs::metadata(&full_path)?.is_dir() {
                _list_files(vec, &full_path)?
            } else {
                vec.push(full_path);
            }
        }
    }
    Ok(())
}

// #[tokio::main]
fn main() {
    println!("Upcoming birthdays - ordered (with the upcoming age):");

    let mut subfiles = Vec::new();
    let _ = _list_files(&mut subfiles, Path::new("csv/"));
    let mut birthdays = Vec::new();

    for file in subfiles {
        let filename = file.to_str().unwrap();
        print!("Reading in {:?}", filename);
        let file = File::open(file.clone()).expect(format!("Could not open {:#?}", file).as_str());
        let mut rdr = csv::Reader::from_reader(file);

        let mut counter = 0;
        for result in rdr.deserialize() {
            let person: Person = result.expect("Could not parse csv line as person");
            birthdays.push(person);
            counter += 1;
        }

        println!(" - got {counter} entries.");
    }

    birthdays.sort();

    for bday in birthdays {
        let (date, title) = bday.get_next_birthday();
        println!("{date} - {title}");
    }
}
//     let http_server = http_api::HttpServer::new("0.0.0.0:3000").unwrap();
//     let mut http_state = http_server.state.clone();

//     tokio::spawn(async move {
//         loop {
//             tokio::select! {
//                 _ = signal::ctrl_c() => {
//                     tracing::warn!("Interrupt Signal received, shutting down.");
//                     break;
//                 }

//                 _ = tokio::time::sleep(Duration::from_secs(1)) => {
//                     let current_i = sniffer_clone.get_in_queue().get_i() as i64;
//                     let i_diff = last_i - current_i;
//                     last_i = current_i;

//                     let processing_count = sniffer_clone.clone().get_in_queue().get_counter();
//                     let write_count = sniffer_clone.get_out_queue().get_counter();
//                     let processing_difference = (last_processing_count as isize) - (processing_count as isize);
//                     let write_difference = (last_write_count as isize) - (write_count as isize);
//                     tracing::info!("Number of packets in processing-/write-queue: {} / {}, processing {} / {} packets per second. Total: {} / {i_diff} per second!", processing_count, write_count, processing_difference, write_difference, current_i);
//                     last_processing_count = processing_count.clone();
//                     last_write_count = write_count.clone();
//                     if last_processing_count == 0 && last_write_count == 0 {
//                         empty_queue_counter += 1;
//                     } else {
//                         empty_queue_counter = 0;
//                     }

//                     counter += 1;
//                     http_state.set_count(counter).await;
//                     let read_in = sniffer_clone.get_in_queue().get_i();
//                     http_state.set_processing_stats(read_in as u64, processing_count as u64, write_count as u64).await;

//                     http_state.update_device_map(processor_arc.devices.clone()).await;

//                     // in_queue_metrics.clone().add_data_point(processing_count as u64);

//                     // if empty_queue_counter >= EMPTY_QUEUE_COUNTER_MAX {
//                     //     tracing::warn!("Empty queue for {} secs, shutting down.", EMPTY_QUEUE_COUNTER_MAX);
//                     //     break;
//                     // }
//                 },
//             }
//         }
//     }).await.unwrap();

//     http_server.shutdown().await;
// }
