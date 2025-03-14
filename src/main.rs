use std::thread;
use std::sync::{Arc, Mutex};
use std::fs::File;
use std::io::{self, Write};
use std::time::Duration;

struct BaggageLog {
    baggage_id: String,
    passenger_id: String,
    passenger_name: String,
    flight_plan: String,
    baggage_status: String,
}
struct CheckInCounter {
    id: u32,
    status: String,
    baggage_log: Arc<Mutex<Vec<BaggageLog>>>,
}

impl CheckInCounter {
    fn new(id: u32, status: String, baggage_log: Arc<Mutex<Vec<BaggageLog>>>) -> Self {
        Self { id, status, baggage_log }
    }

    fn process_baggage(&self, baggage_id: &str, passenger_id: &str, passenger_name: &str, flight_plan: &str, baggage_status: &str) {
        let mut log = self.baggage_log.lock().unwrap();
        let baggage = BaggageLog {
            baggage_id: baggage_id.to_string(),
            passenger_id: passenger_id.to_string(),
            passenger_name: passenger_name.to_string(),
            flight_plan: flight_plan.to_string(),
            baggage_status: baggage_status.to_string(),
        };
        if(self.status == "open") {
            log.push(baggage);
            println!("Skranken {} registrerede bagage: {} for passager: {}, fly: {}, status: {}", self.id, baggage_id, passenger_name, flight_plan, baggage_status);
        } else if(self.status == "closed") {
            println!("Skranken {} er lukket", self.id)
        }
    }
}

#[derive(Clone)]
struct SortingFacility {
    baggage_log: Arc<Mutex<Vec<BaggageLog>>>,
}

impl SortingFacility {
    fn new(baggage_log: Arc<Mutex<Vec<BaggageLog>>>) -> Self {
        Self { baggage_log }
    }
    fn sort_baggage(&self, baggage_id: &str) {
        let mut log = self.baggage_log.lock().unwrap();
        let baggage = log.iter_mut().find(|x| x.baggage_id == baggage_id);

        if let Some(baggage) = baggage {
            baggage.baggage_status = "Sorteret".to_string();
            println!("Bagage {} er blevet sorteret", baggage_id);
        }
    }
}

#[derive(Clone)]
struct TerminalGate {
    baggage_log: Arc<Mutex<Vec<BaggageLog>>>,
}

impl TerminalGate {
    fn new(baggage_log: Arc<Mutex<Vec<BaggageLog>>>) -> Self {
        Self { baggage_log }
    }

    fn load_baggage(&self, baggage_id: &str) {
        let mut log = self.baggage_log.lock().unwrap();
        let baggage = log.iter_mut().find(|x| x.baggage_id == baggage_id);

        if let Some(baggage) = baggage {
            baggage.baggage_status = "Lastet på flyet".to_string();
            println!("Baggage {} er lastet på flyet", baggage_id);
        }
    }
}

fn main() -> io::Result<()> {
    let baggage_log = Arc::new(Mutex::new(Vec::new()));

    let counter = CheckInCounter::new(3, "open".to_string(), baggage_log.clone());
    let sorting_facility = SortingFacility::new(baggage_log.clone());
    let terminal_gate = TerminalGate::new(baggage_log.clone());

    let counter2 = CheckInCounter::new(1, "open".to_string(), baggage_log.clone());

    let counter_handle = thread::spawn(move || {
        counter.process_baggage("BAG123", "PAS123", "John", "Flight 101", "Pending");
    });

    let sorting_handle = thread::spawn({
        let sorting_facility = sorting_facility.clone();
        move || {
            thread::sleep(Duration::from_secs(1));
            sorting_facility.sort_baggage("BAG123");
        }
    });

    let terminal_handle = thread::spawn({
        let terminal_gate = terminal_gate.clone();
        move || {
            thread::sleep(Duration::from_secs(2));
            terminal_gate.load_baggage("BAG123");
        }
    });

    let counter_handle2 = thread::spawn(move || {
        counter2.process_baggage("BAG234", "PAS234", "Jens", "Flight 102", "Pending");
    });

    let sorting_handle2 = thread::spawn({
        let sorting_facility = sorting_facility.clone();
        move || {
            thread::sleep(Duration::from_secs(2));
            sorting_facility.sort_baggage("BAG234");
        }
    });

    let terminal_handle2 = thread::spawn({
        let terminal_gate = terminal_gate.clone();
        move || {
            thread::sleep(Duration::from_secs(3));
            terminal_gate.load_baggage("BAG234");
        }
    });

    counter_handle.join().unwrap();
    sorting_handle.join().unwrap();
    terminal_handle.join().unwrap();

    counter_handle2.join().unwrap();
    sorting_handle2.join().unwrap();
    terminal_handle2.join().unwrap();

    let log = baggage_log.lock().unwrap();
    let mut file = File::create("baggage_log.txt")?;
    for entry in log.iter() {
        writeln!(
            file,
            "Bagage ID: {} | Passager: {} | Fly: {} | Status: {}",
            entry.baggage_id, entry.passenger_name, entry.flight_plan, entry.baggage_status
        )?;
    }
    Ok(())
}