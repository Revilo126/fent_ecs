#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tick(pub u64);

impl Tick {
    pub fn is_newer_than(self, last_run: Tick) -> bool {
        self.0 > last_run.0
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ComponentTicks {
    pub added: Tick,
    pub changed: Tick,
}

impl ComponentTicks {
    pub fn new(now: Tick) -> Self {
        Self {
            added: now,
            changed: now,
        }
    }
}
