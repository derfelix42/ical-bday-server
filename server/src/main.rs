use std::cmp::Ordering;

use chrono::{DateTime, Datelike, NaiveDate, TimeDelta, Utc};
use tokio::signal;
mod http_api;

#[derive(Eq)]
struct Person {
    pub name: String,
    pub birthday: NaiveDate,
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

// #[tokio::main]
fn main() {
    println!("Hallo Welt!");

    let mut birthdays = Vec::new();

    birthdays.push(Person::create("Test".to_string(), 1970, 1, 1));

    println!("Hello, world!");
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
