use std::time::{Duration, Instant};
use std::cell::RefCell;

pub fn throttle_cell<F, T>(func: F, duration: Duration) -> RefCell<Box<dyn FnMut() -> T>>
where
    F: FnMut() -> T + 'static,
    T: Clone + 'static,
{
    let throttled_func = throttle(func, duration);
    RefCell::new(Box::new(throttled_func))
}

pub fn throttle<F, T>(mut func: F, interval: Duration) -> impl FnMut() -> T
where
    F: FnMut() -> T,
    T: Clone,
{
    let mut last_call_time: Option<Instant> = None;
    let mut last_result: Option<T> = None;

    move || {
        let now = Instant::now();

        if let Some(last_call) = last_call_time {
            if now.duration_since(last_call) < interval {
                if let Some(ref result) = last_result {
                    return result.clone();
                }
            }
        }

        let result = func();
        last_result = Some(result.clone());
        last_call_time = Some(now);

        result
    }
}
