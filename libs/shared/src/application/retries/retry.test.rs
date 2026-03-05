#[cfg(test)]
mod retries_test {
    use crate::application::retries;
    use crate::application::retries::{Jitter, RetryPolicy};

    mod calculate_delay_test {
        use super::*;

        mod exponential_backoff_test {
            use super::*;
            #[test]
            fn test_with_full_jitter() {
                let policy = RetryPolicy::ExponentialBackoff(retries::ExponentialBackoff {
                    retries: retries::Retry::NTimes(4),
                    delay: 100,
                    base: Some(2),
                    max_delay: 1000,
                    jitter: Some(Jitter::Full),
                });

                assert!(retries::calculate_delay(&100, &0, &policy) < 100);
                assert!(retries::calculate_delay(&100, &1, &policy) < 200);
                assert!(retries::calculate_delay(&100, &2, &policy) < 400);
                assert!(retries::calculate_delay(&100, &3, &policy) < 800);
                // limit to max_delay
                assert!(retries::calculate_delay(&100, &4, &policy) < 1000);
            }

            #[test]
            fn test_without_full_jitter() {
                let policy = RetryPolicy::ExponentialBackoff(retries::ExponentialBackoff {
                    retries: retries::Retry::NTimes(4),
                    delay: 100,
                    base: Some(2),
                    max_delay: 1000,
                    jitter: None,
                });

                assert_eq!(retries::calculate_delay(&100, &0, &policy), 100);
                assert_eq!(retries::calculate_delay(&100, &1, &policy), 200);
                assert_eq!(retries::calculate_delay(&100, &2, &policy), 400);
                assert_eq!(retries::calculate_delay(&100, &3, &policy), 800);
                // limit to max_delay
                assert_eq!(retries::calculate_delay(&100, &4, &policy), 1000);
            }
        }

        mod no_backoff_test {
            use super::retries;
            use super::retries::RetryPolicy;
            #[test]
            fn test() {
                let policy = RetryPolicy::NoBackoff(retries::NoBackoff {
                    retries: retries::Retry::NTimes(4),
                });

                assert_eq!(retries::calculate_delay(&100, &1, &policy), 0);
                assert_eq!(retries::calculate_delay(&100, &2, &policy), 0);
                assert_eq!(retries::calculate_delay(&100, &3, &policy), 0);
            }
        }

        mod fixed_backoff_test {
            use super::retries;
            use super::retries::RetryPolicy;
            #[test]
            fn test() {
                let policy = RetryPolicy::FixedBackoff(retries::FixedBackoff {
                    retries: retries::Retry::NTimes(4),
                    delay: 100,
                });

                assert_eq!(retries::calculate_delay(&100, &1, &policy), 100);
                assert_eq!(retries::calculate_delay(&100, &2, &policy), 100);
                assert_eq!(retries::calculate_delay(&100, &3, &policy), 100);
            }
        }

        mod linear_backoff_test {
            use super::retries;
            use super::retries::RetryPolicy;
            #[test]
            fn test() {
                let policy = RetryPolicy::LinearBackoff(retries::LinearBackoff {
                    retries: retries::Retry::NTimes(4),
                    delay: 100,
                });

                assert_eq!(retries::calculate_delay(&100, &0, &policy), 100);
                assert_eq!(retries::calculate_delay(&100, &1, &policy), 200);
                assert_eq!(retries::calculate_delay(&100, &2, &policy), 300);
                assert_eq!(retries::calculate_delay(&100, &3, &policy), 400);
            }
        }
    }

    mod retry_async_test {
        use std::cell::Cell;

        #[tokio::test]
        async fn test_with_exponential_backoff() {
            use super::*;
            let policy = RetryPolicy::ExponentialBackoff(retries::ExponentialBackoff {
                retries: retries::Retry::NTimes(5),
                delay: 100,
                base: Some(8),
                max_delay: 1000,
                jitter: None,
            });

            let attempts = Cell::new(0);
            let timestamp = Cell::new(std::time::Instant::now());
            let result = retries::retry_async(
                || async {
                    let now = std::time::Instant::now();
                    let delay = now - timestamp.get();
                    timestamp.set(now);
                    match attempts.get() {
                        0 => assert_eq!(delay.as_millis(), 0),
                        1 => assert!(delay.as_millis() > 100),
                        2 => assert!(delay.as_millis() > 800),
                        3 => assert!(delay.as_millis() > 1000 && delay.as_millis() < 6400),
                        _ => {}
                    }
                    attempts.set(attempts.get() + 1);
                    if attempts.get() < 4 {
                        Err("Error")
                    } else {
                        Ok("Success")
                    }
                },
                &policy,
            ).await;

            assert_eq!(result, Ok("Success"));
            assert_eq!(attempts.get(), 4);
        }

        #[tokio::test]
        async fn test_with_fixed_backoff() {
            use super::*;
            let policy = RetryPolicy::FixedBackoff(retries::FixedBackoff {
                retries: retries::Retry::NTimes(3),
                delay: 100,
            });

            let attempts = Cell::new(0);
            let timestamp = Cell::new(std::time::Instant::now());
            let result = retries::retry_async(
                || async {
                    let now = std::time::Instant::now();
                    let delay = now - timestamp.get();
                    timestamp.set(now);
                    match attempts.get() {
                        0 => assert_eq!(delay.as_millis(), 0),
                        1 => assert!(delay.as_millis() > 100),
                        2 => assert!(delay.as_millis() > 100),
                        3 => assert!(delay.as_millis() > 100),
                        _ => {}
                    }
                    attempts.set(attempts.get() + 1);
                    if attempts.get() < 4 {
                        Err("Error")
                    } else {
                        Ok("Success")
                    }
                },
                &policy,
            )
                .await;

            assert_eq!(result, Ok("Success"));
            assert_eq!(attempts.get(), 4);
        }

        #[tokio::test]
        async fn test_with_linear_backoff() {
            use super::*;
            let policy = RetryPolicy::LinearBackoff(retries::LinearBackoff {
                retries: retries::Retry::NTimes(3),
                delay: 100,
            });

            let attempts = Cell::new(0);
            let timestamp = Cell::new(std::time::Instant::now());
            let result = retries::retry_async(
                || async {
                    let now = std::time::Instant::now();
                    let delay = now - timestamp.get();
                    timestamp.set(now);
                    match attempts.get() {
                        0 => assert_eq!(delay.as_millis(), 0),
                        1 => assert_eq!(delay.as_millis()/100, 1),
                        2 => assert_eq!(delay.as_millis()/100, 2),
                        3 => assert_eq!(delay.as_millis()/100, 3),
                        _ => {}
                    }
                    attempts.set(attempts.get() + 1);
                    if attempts.get() < 4 {
                        Err("Error")
                    } else {
                        Ok("Success")
                    }
                },
                &policy,
            ).await;

            assert_eq!(result, Ok("Success"));
            assert_eq!(attempts.get(), 4);
        }

        #[tokio::test]
        async fn test_with_no_backoff() {
            use super::*;
            let policy = RetryPolicy::NoBackoff(retries::NoBackoff {
                retries: retries::Retry::NTimes(3),
            });

            let attempts = Cell::new(0);
            let timestamp = Cell::new(std::time::Instant::now());
            let result = retries::retry_async(
                || async {
                    let now = std::time::Instant::now();
                    let delay = now - timestamp.get();
                    timestamp.set(now);
                    match attempts.get() {
                        0 => assert_eq!(delay.as_millis(), 0),
                        1 => assert_eq!(delay.as_millis()/100, 0),
                        2 => assert_eq!(delay.as_millis()/100, 0),
                        3 => assert_eq!(delay.as_millis()/100, 0),
                        _ => {}
                    }
                    attempts.set(attempts.get() + 1);
                    if attempts.get() < 4 {
                        Err("Error")
                    } else {
                        Ok("Success")
                    }
                },
                &policy,
            ).await;

            assert_eq!(result, Ok("Success"));
            assert_eq!(attempts.get(), 4);
        }

        #[tokio::test]
        async fn test_with_unmatched_number_of_retries() {
            let number_of_retries = 5;
            use super::*;
            let policy = RetryPolicy::NoBackoff(retries::NoBackoff {
                retries: retries::Retry::NTimes(number_of_retries -3),
            });

            let attempts = Cell::new(0);
            let result = retries::retry_async(
                || async {
                    attempts.set(attempts.get() + 1);
                    if attempts.get() < number_of_retries {
                        Err("Error")
                    } else {
                        Ok("Success")
                    }
                },
                &policy,
            ).await;

            // The result should be an error since retries are less than the number of attempts
            assert_eq!(result, Err("Error"));
            assert_eq!(attempts.get(), number_of_retries - 2);
        }
    }
}
