use std::future::Future;

pub enum Retry {
    Finite(u8),
    Infinity
}

pub enum Jitter {

}

pub struct ExponentionalBackoff {
    pub retries: Retry,
    pub base: u32,
    pub exponent: u32,
    pub jitter: Option<Jitter>
}

pub enum RetryPolicy {
    ExponentionalBackoff(ExponentionalBackoff)
}

/// Retries an asynchronous function call based on the provided number of retries
/// and retry policy.
pub async fn retry_async<F, Fut, O, E>(op: F, policy: RetryPolicy) -> Result<O, E>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<O, E>>
{
    let mut retries: i8 = 0;
    match policy {
        RetryPolicy::ExponentionalBackoff(backoff) => {
            retries = match backoff.retries {
                Retry::Finite(r) => r,
                Retry::Infinity => -1,
            }
        }
    }
    let mut i: u8 = 0;
    loop {
        let result = op().await;
        match result {
            Ok(v) => return Ok(v),
            Err(err) => {
                if i <  retries {
                    i += 1;
                    continue;
                };
                return Err(err)
            }
        }
    };
}