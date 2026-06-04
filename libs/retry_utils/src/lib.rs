use std::future::Future;
use rand::RngExt;
use tokio::time::{sleep, Duration};

pub enum Retry {
    /// Number of retires
    NTimes(u16),
    Indefinitely
}

pub enum Jitter {
    Full,
}

pub struct ExponentialBackoff {
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
    ExponentialBackoff(ExponentialBackoff),
    FixedBackoff(FixedBackoff),
    LinearBackoff(LinearBackoff)
}

fn calculate_delay(base_delay: &u64, attempt: &u16, policy: &RetryPolicy) -> u64 {
    match policy {
        RetryPolicy::ExponentialBackoff(backoff) => {
            let base = backoff.base.unwrap_or(2) as u64;
            if let Some(jitter) = &backoff.jitter {
                return match jitter {
                    Jitter::Full => {
                        let exp = base.pow(*attempt as u32);
                        let max = base_delay
                            .saturating_mul(exp)
                            .min(backoff.max_delay);
                        rand::rng().random_range(0..max)
                    }
                }
            }
            
            return (base_delay * base.pow(*attempt as u32)).min(backoff.max_delay.clone())
        },
        RetryPolicy::FixedBackoff(_) => base_delay * 1,
        RetryPolicy::LinearBackoff(_) => base_delay * (attempt.clone() as u64 + 1),
        RetryPolicy::NoBackoff(_) => 0,
    }
}

pub enum RetryStrategyAction {
    ContinueRetries,
    ReturnResult
}

/// Trait to handle error filtering for retries
pub trait RetryStrategy<E>: Send + Sync {
    fn handle_error(&self, error: &E, attempt: i16) -> RetryStrategyAction;
}

impl<E, F> RetryStrategy<E> for F 
    where F: Fn(&E, i16) -> RetryStrategyAction + Send + Sync 
{
    fn handle_error(&self, error: &E, attempt: i16) -> RetryStrategyAction {
        self(error, attempt)
    }
}

// Gives the compiler a default type for the option when None is passed for the 
// optional retry strategy. Without this, the compile is unable to infer the option
// type.
impl<E> RetryStrategy<E> for Option<fn(&E, i16) -> RetryStrategyAction> {
    fn handle_error(&self, error: &E, attempt: i16) -> RetryStrategyAction {
        if let Some(f) = self {
            return f(error, attempt)
        }
        
        RetryStrategyAction::ContinueRetries // Default to continuing retries
    }
}

/// Retries an asynchronous function call based on the provided number of retries
/// and retry policy.
pub async fn retry_async<F, Fut, O, E, S>(
    op: F,
    policy: &RetryPolicy,
    retry_strategy: S
) -> Result<O, E>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = Result<O, E>>,
        S: RetryStrategy<E>
{
    // We use i16 because we want to allow -1 for retrying an indefinite number
    // of times. The Retry::NTimes(n) will be cast from u16 to i16 for all policies
    let retries: i16;
    let mut delay: u64 = 0;

    match policy {
        RetryPolicy::ExponentialBackoff(backoff) => {
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
        RetryPolicy::FixedBackoff(backoff) => {
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
        RetryPolicy::NoBackoff(backoff) => {
            match backoff.retries {
                Retry::NTimes(n) => {
                    retries = n as i16;
                },
                Retry::Indefinitely => {
                    retries = -1;
                },
            }
        },
        RetryPolicy::LinearBackoff(backoff) => {
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

    let mut attempt: i16 = 0;

    // Calculate the initial decay
    let mut calculated_delay = calculate_delay(&delay, &(attempt.clone() as u16).clone(), &policy);

    loop {
        // Call the operation
        let result = op().await;

        // Return result or retry based on provided policy
        match result {
            Ok(v) => return Ok(v),
            Err(err) => {
                // If the error filter returns false, return the error early
                if matches!(retry_strategy.handle_error(&err, attempt + 1), RetryStrategyAction::ReturnResult) {
                    return Err(err)
                }
                // Handle delay
                if calculated_delay > 0 && attempt != retries {
                    sleep(Duration::from_millis(calculated_delay)).await;
                }

                // 1st condition: indefinite retry case.
                // 2nd condition: handle n retries
                if retries == -1 || attempt < retries  {
                    attempt += 1;
                    // Calculate the new delay
                    calculated_delay = calculate_delay(&delay, &(attempt.clone() as u16), &policy);
                    continue;
                }

                return Err(err)
            }
        }
    };
}

// Unit tests
#[cfg(test)]
#[path = "test.rs"]
mod test;