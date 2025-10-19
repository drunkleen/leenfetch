#![cfg(test)]

use once_cell::sync::Lazy;
use std::env;
use std::ffi::OsString;
use std::sync::{Mutex, MutexGuard};

static ENV_GUARD: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

pub(crate) struct EnvLock {
    saved: Vec<(OsString, Option<OsString>)>,
    _guard: MutexGuard<'static, ()>,
}

impl EnvLock {
    pub(crate) fn acquire(vars: &[&str]) -> Self {
        let guard = ENV_GUARD.lock().expect("env mutex poisoned");
        let saved = vars
            .iter()
            .map(|key| (OsString::from(key), env::var_os(key)))
            .collect();
        Self { saved, _guard: guard }
    }

    pub(crate) fn set_var(&self, key: &str, value: &str) {
        unsafe { env::set_var(key, value) };
    }

    pub(crate) fn remove_var(&self, key: &str) {
        unsafe { env::remove_var(key) };
    }
}

impl Drop for EnvLock {
    fn drop(&mut self) {
        for (key, value) in &self.saved {
            unsafe {
                if let Some(val) = value {
                    env::set_var(key, val);
                } else {
                    env::remove_var(key);
                }
            }
        }
    }
}
