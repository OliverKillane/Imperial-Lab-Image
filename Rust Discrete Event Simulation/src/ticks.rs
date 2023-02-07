use crate::simulation::{Event, Simulation, Time};

struct Tick;

impl Event<()> for Tick {
    fn invoke(self, sim: &mut Simulation<Self, ()>)
    where
        Self: Sized,
    {
        sim.schedule(Tick, 1);
        println!("Tick at {}", sim.get_time())
    }
}

pub fn sim(stop_time: Time) {
    let mut sim = Simulation::new(Box::new(move |s| s.get_time() >= stop_time), ());
    sim.schedule(Tick, 1);
    sim.simulate()
}
