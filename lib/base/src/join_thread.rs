use std::thread::JoinHandle;

use anyhow::anyhow;
use hiarc::Hiarc;

/// A thread that joins when dropped.
/// It should _usually_ be the last member of a struct
#[derive(Debug, Hiarc)]
pub struct JoinThread<T>(Option<JoinHandle<T>>);

impl<T> JoinThread<T> {
    pub const fn new(handle: JoinHandle<T>) -> Self {
        Self(Some(handle))
    }
    pub const fn new_opt(handle: Option<JoinHandle<T>>) -> Self {
        Self(handle)
    }
}

impl<T> JoinThread<T> {
    /// Check wether the thread is in a state where it is not running anymore,
    /// this includes that the thread was never started in first place.
    pub fn is_finished(&self) -> bool {
        self.0.as_ref().is_none_or(|thread| thread.is_finished())
    }

    /// Returns an error if joining the thread failed.
    ///
    /// Returns `Ok(None)` if the thread was either not started at all
    /// or is still running.
    ///
    /// Otherwise it returns the value returned by the thread.
    pub fn try_join(&mut self) -> anyhow::Result<Option<T>> {
        let is_finished = self.is_finished();
        if let Some(thread) = self.0.take() {
            if is_finished {
                Ok(thread.join().map(|res| Some(res)).map_err(|e| {
                    anyhow!(
                        "{}",
                        match (e.downcast_ref::<&str>(), e.downcast_ref::<String>()) {
                            (Some(&s), _) => s,
                            (_, Some(s)) => s,
                            (None, None) => "<No panic info>",
                        }
                    )
                })?)
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}

impl<T> Drop for JoinThread<T> {
    fn drop(&mut self) {
        if let Some(thread) = self.0.take() {
            let _ = thread.join();
        }
    }
}
