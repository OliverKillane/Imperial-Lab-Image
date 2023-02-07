use binary_heap_plus::{BinaryHeap, KeyComparator};
use std::marker::PhantomData;

pub type Time = u32;

pub trait Event<S> {
    /// Invokes the event, potentially adding new events or altering the state
    /// of the simulation.
    ///
    /// Note: Due to &mut the event can only access: sim.get_time, sim.get_state, sim.schedule
    fn invoke(self, sim: &mut Simulation<Self, S>)
    where
        Self: Sized;
}

struct ScheduledEvent<E: Event<S>, S>(Time, E, PhantomData<S>);

type ScheduleKey<E, S> = KeyComparator<fn(&ScheduledEvent<E, S>) -> Time>;
type StopCond<E, S> = Box<dyn Fn(&Simulation<E, S>) -> bool>;
pub struct Simulation<E: Event<S>, S> {
    events: BinaryHeap<ScheduledEvent<E, S>, ScheduleKey<E, S>>,
    time: Time,
    stop_condition: StopCond<E, S>,
    state: S,
}

impl<E: Event<S>, S> Simulation<E, S> {
    pub fn new(stop_condition: StopCond<E, S>, state: S) -> Self {
        Simulation {
            events: BinaryHeap::new_by_key(|x: &ScheduledEvent<E, S>| x.0),
            time: 0,
            stop_condition,
            state,
        }
    }

    pub fn get_time(&self) -> Time {
        self.time
    }

    pub fn get_state(&mut self) -> &mut S {
        &mut self.state
    }

    pub fn schedule(&mut self, event: E, time: Time) {
        self.events
            .push(ScheduledEvent(self.get_time() + time, event, PhantomData))
    }

    /// Runs the simulation and consumes it, returning the final state.
    ///
    /// Note: The mut self means simulate() takes ownership of the simulation
    /// (user can no longer access it without a compile error) and changes self.
    pub fn simulate(mut self) -> S {
        while let Some(ScheduledEvent(new_time, event, _)) = self.events.pop() {
            self.time = new_time;
            if (self.stop_condition)(&self) {
                break;
            } else {
                event.invoke(&mut self)
            }
        }
        self.state
    }
}
