use crate::world::change_detection::Tick;

pub struct SystemMeta {
    pub(crate) name: &'static str,
    pub(crate) thread_access: ThreadAccess,
    pub(crate) last_run: Tick,
}

impl Default for SystemMeta {
    fn default() -> Self {
        Self {
            name: "System",
            thread_access: ThreadAccess::AnyThread,
            last_run: Tick(0),
        }
    }
}

impl SystemMeta {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            thread_access: ThreadAccess::AnyThread,
            last_run: Tick(0),
        }
    }

    #[inline]
    pub fn name(&self) -> &'static str {
        self.name
    }

    #[inline]
    pub fn thread_access(&self) -> &ThreadAccess {
        &self.thread_access
    }

    #[inline]
    pub fn is_main_thread_only(&self) -> bool {
        matches!(self.thread_access, ThreadAccess::MainThread)
    }

    #[inline]
    pub fn set_main_thread_only(&mut self) {
        self.thread_access = ThreadAccess::MainThread;
    }

    #[inline]
    pub fn last_run(&self) -> Tick {
        self.last_run
    }
}

pub enum ThreadAccess {
    AnyThread,
    MainThread,
}
