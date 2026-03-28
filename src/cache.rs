use std::sync::Mutex;
use std::time::{Duration, Instant};

const DEFAULT_TTL_SECS: u64 = 5;

pub struct Cache<T> {
    entries: Mutex<std::collections::HashMap<String, CacheEntry<T>>>,
    ttl: Duration,
}

struct CacheEntry<T> {
    value: T,
    expires_at: Instant,
}

impl<T> Cache<T> {
    pub fn new(ttl_secs: u64) -> Self {
        Self {
            entries: Mutex::new(std::collections::HashMap::new()),
            ttl: Duration::from_secs(ttl_secs),
        }
    }

    pub fn default_ttl() -> Self {
        Self::new(DEFAULT_TTL_SECS)
    }

    pub fn get_or_compute<F>(&self, key: &str, compute: F) -> T
    where
        F: FnOnce() -> T,
        T: Clone,
    {
        let now = Instant::now();

        // Try to get from cache first (without lock)
        {
            let entries = &mut *self.entries.lock().unwrap();

            if let Some(entry) = entries.get(key) {
                if entry.expires_at > now {
                    return entry.value.clone();
                }
                // Entry expired
                entries.remove(key);
            }
        }

        // Compute new value (outside of lock)
        let value = compute();

        // Store in cache
        {
            let entries = &mut *self.entries.lock().unwrap();
            let expires_at = now + self.ttl;
            entries.insert(
                key.to_string(),
                CacheEntry {
                    value: value.clone(),
                    expires_at,
                },
            );
        }

        value
    }

    pub fn invalidate(&self, key: &str) {
        self.entries.lock().unwrap().remove(key);
    }

    pub fn clear(&self) {
        self.entries.lock().unwrap().clear();
    }
}

impl<T> Default for Cache<T> {
    fn default() -> Self {
        Self::default_ttl()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration as StdDuration;

    #[test]
    fn test_cache_basic() {
        let cache = Cache::new(1);
        let call_count = std::cell::RefCell::new(0);

        let get_value = || {
            *call_count.borrow_mut() += 1;
            42
        };

        // First call - computes
        let v1 = cache.get_or_compute("key", get_value);
        assert_eq!(v1, 42);
        assert_eq!(*call_count.borrow(), 1);

        // Second call - should use cache
        let v2 = cache.get_or_compute("key", get_value);
        assert_eq!(v2, 42);
        assert_eq!(*call_count.borrow(), 1);
    }

    #[test]
    fn test_cache_expiry() {
        let cache = Cache::new(1); // 1 second TTL
        let call_count = std::cell::RefCell::new(0);

        let get_value = || {
            *call_count.borrow_mut() += 1;
            42
        };

        // First call
        let v1 = cache.get_or_compute("key", get_value);
        assert_eq!(v1, 42);
        assert_eq!(*call_count.borrow(), 1);

        // Wait for expiry
        thread::sleep(StdDuration::from_millis(1100));

        // Should compute again
        let v2 = cache.get_or_compute("key", get_value);
        assert_eq!(v2, 42);
        assert_eq!(*call_count.borrow(), 2);
    }
}
