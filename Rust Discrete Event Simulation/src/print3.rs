use crate::simulation::{Event, Simulation};

struct Print(i32);

impl Event<()> for Print {
    fn invoke(self, sim: &mut Simulation<Self, ()>) {
        println!("Event {} invoked at time {}", self.0, sim.get_time());
    }
}

pub fn sim() {
    let mut sim = Simulation::new(Box::new(|_| false), ());

    sim.schedule(Print(1), 72);
    sim.schedule(Print(2), 116);
    sim.schedule(Print(3), 29);
    sim.simulate()
}
