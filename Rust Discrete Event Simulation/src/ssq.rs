use std::ops::Range;

use crate::simulation::{Event, Simulation, Time};
use rand::{rngs::ThreadRng, thread_rng, Rng};

struct WaitTracker {
    queue_size: u32,
    accumulated_wait: u32,
    last_time: Time,
    time_gen: ThreadRng,
}

impl WaitTracker {
    fn new() -> Self {
        WaitTracker {
            queue_size: 0,
            accumulated_wait: 0,
            last_time: 0,
            time_gen: thread_rng(),
        }
    }

    fn join_queue(&mut self, time: Time) -> Time {
        self.update_accumulation(time);
        self.queue_size += 1;
        self.serve_time()
    }

    fn serve_time(&mut self) -> Time {
        const TIME_RANGE: Range<Time> = 0..100;
        self.time_gen.gen_range(TIME_RANGE)
    }

    fn leave_queue(&mut self, time: Time) {
        assert!(
            self.queue_size > 0,
            "Customer attempted to leave empty queue."
        );
        self.update_accumulation(time);
        self.queue_size -= 1;
    }

    fn update_accumulation(&mut self, time: Time) {
        assert!(time >= self.last_time, "arrival/departures not in order");
        self.accumulated_wait += self.queue_size * (time - self.last_time);
        self.last_time = time;
    }

    fn mean_time(&mut self) -> Time {
        if self.last_time > 0 {
            self.accumulated_wait / self.last_time
        } else {
            0
        }
    }
}

enum Queuer {
    Join,
    Leave,
}

impl Event<WaitTracker> for Queuer {
    fn invoke(self, sim: &mut Simulation<Self, WaitTracker>)
    where
        Self: Sized,
    {
        const OFFSET_TIME: Time = 25;
        let time = sim.get_time();
        match self {
            Queuer::Join => {
                let new_time = sim.get_state().join_queue(time);
                sim.schedule(Queuer::Join, new_time);
                if sim.get_state().queue_size == 1 {
                    sim.schedule(Queuer::Leave, OFFSET_TIME)
                }
                print!("Departure")
            }
            Queuer::Leave => {
                sim.get_state().leave_queue(time);
                if sim.get_state().queue_size > 0 {
                    sim.schedule(Queuer::Leave, OFFSET_TIME)
                }
                print!("Arrival")
            }
        }
        println!(
            " at {} new population {}",
            sim.get_time(),
            sim.get_state().queue_size
        )
    }
}

pub fn sim(stop_time: Time) {
    let mut sim = Simulation::new(
        Box::new(move |s| s.get_time() >= stop_time),
        WaitTracker::new(),
    );
    let first_time = sim.get_state().serve_time();
    sim.schedule(Queuer::Join, first_time);
    let mean = sim.simulate().mean_time();
    println!("Mean queue time was {mean}")
}

#[cfg(test)]
mod test {
    use super::WaitTracker;

    #[test]
    #[should_panic]
    fn cannot_have_negative_queue() {
        let mut x = WaitTracker::new();
        x.leave_queue(0);
    }

    #[test]
    #[should_panic]
    fn queuers_must_arrive_in_order() {
        let mut x = WaitTracker::new();
        x.join_queue(9);
        x.join_queue(8);
    }
}
