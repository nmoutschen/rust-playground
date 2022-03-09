use chrono::{DateTime, Duration, Utc};
use rand::{prelude::*, distributions::{Alphanumeric, Standard}};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Appointment {
    customer: String,
    employee: String,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
}

impl Distribution<Appointment> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Appointment {
        let duration = Duration::seconds(rng.gen_range(1..=86400));
        let start_time = Utc::now() + Duration::seconds(rng.gen_range(1..=31556736));

        Appointment {
            customer: (0..15).map(|_| rng.sample(Alphanumeric) as char).collect(),
            employee: (0..15).map(|_| rng.sample(Alphanumeric) as char).collect(),
            start_time,
            end_time: start_time + duration,
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();

    let appt: Appointment = rng.gen();

    let appt_string = serde_json::to_string_pretty(&appt).expect("unable to serialize appointment");
    println!("{}", appt_string);
}