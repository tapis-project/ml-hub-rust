use std::future::Future;
use rand::Rng;
use tokio::time::{sleep, Duration};

pub enum Retry {
    /// Number of retires
    NTimes(u16),
    Indefinitely
}

pub enum Jitter {
    Full,
}

pub struct ExponentionalBackoff {
    pub retries: Retry,
    pub delay: u64,
    pub base: Option<u32>,
    pub max_delay: u64,
    pub jitter: Option<Jitter>
}

/// Retry instantly
pub struct NoBackoff {
    pub retries: Retry
}

/// Retry at some fixed interval
pub struct FixedBackoff {
    pub retries: Retry,
    pub delay: u64
}

/// Retry with a linear increase in delay time: `delay * retries`
pub struct LinearBackoff {
    pub retries: Retry,
    pub delay: u64
}

pub enum RetryPolicy {
    NoBackoff(NoBackoff),
    ExponentionalBackoff(ExponentionalBackoff),
    FixedBackoff(FixedBackoff),
    LinearBackoff(LinearBackoff)
}

fn calculate_delay(base_delay: &u64, attempt: &u16, policy: &RetryPolicy) -> u64 {
    match policy {
        RetryPolicy::ExponentionalBackoff(backoff) => {
            let base = backoff.base.unwrap_or(2) as u64;
            if let Some(jitter) = &backoff.jitter {
                match jitter {
                    Jitter::Full => {
                        let max = (base_delay * base.pow(*attempt as u32)).min(backoff.max_delay.clone());
                        rand::rng().random_range(0..max)
                    }
                };
            };
            
            (base_delay * base.pow(*attempt as u32)).min(backoff.max_delay.clone())
        },
        RetryPolicy::FixedBackoff(_) => {
            base_delay * 1
        },
        RetryPolicy::LinearBackoff(_) => {
            base_delay * attempt.clone() as u64 
        },
        RetryPolicy::NoBackoff(_) => {
            0
        }
    }
    
    
}

/// Retries an asynchronous function call based on the provided number of retries
/// and retry policy.
pub async fn retry_async<F, Fut, O, E>(op: F, policy: &RetryPolicy) -> Result<O, E>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<O, E>>
{
    // We use i16 because we want to allow -1 for retrying an indefinite number
    // of times. The Retry::NTimes(n) will be cast from u16 to i16 for all policies
    let retries: i16;
    let mut delay: u64 = 0;
    let mut attempt: i16 = 0;

    match policy {
        RetryPolicy::ExponentionalBackoff(ref backoff) => {
            match backoff.retries {
                Retry::NTimes(n) => {
                    retries = n as i16;
                    delay = backoff.delay;
                },
                Retry::Indefinitely => {
                    retries = -1;
                },
            };
        },
        RetryPolicy::FixedBackoff(ref backoff) => {
            delay = backoff.delay;
            match backoff.retries {
                Retry::NTimes(n) => {
                    retries = n as i16;
                },
                Retry::Indefinitely => {
                    retries = -1;
                },
            }
        },
        RetryPolicy::NoBackoff(ref backoff) => {
            match backoff.retries {
                Retry::NTimes(n) => {
                    retries = n as i16;
                },
                Retry::Indefinitely => {
                    retries = -1;
                },
            }
        },
        RetryPolicy::LinearBackoff(ref backoff) => {
            delay = backoff.delay;
            match backoff.retries {
                Retry::NTimes(n) => {
                    retries = n as i16;
                },
                Retry::Indefinitely => {
                    retries = -1;
                },
            }
        }
    };

    // Caculate the initial decay
    let mut calulated_delay = calculate_delay(&delay, &(attempt.clone() as u16).clone(), &policy);
    loop {
        // Call the operation
        let result = op().await;

        // Return result or retry based on provided policy
        match result {
            Ok(v) => return Ok(v),
            Err(err) => {
                // Handle delay
                if delay > 0 && attempt != retries {
                    sleep(Duration::from_millis(delay)).await;
                }

                // 1st condition: indefinite retry case.
                // 2nd condition: handle n retries
                if retries == -1 || attempt < retries  {
                    attempt += 1;
                    // Calculate the new delay
                    calulated_delay = calculate_delay(&calulated_delay, &(attempt.clone() as u16), &policy);
                    continue;
                }

                return Err(err)
            }
        }
    };
}