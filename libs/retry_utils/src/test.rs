#[cfg(test)]
mod retries_test {
    use crate::*;

    mod calculate_delay_test {
        use super::*;

        mod exponential_backoff_test {
            use super::*;
            #[test]
            fn test_with_full_jitter() {
                let policy = RetryPolicy::ExponentialBackoff(ExponentialBackoff {
                    retries: Retry::NTimes(4),
                    delay: 100,
                    base: Some(2),
                    max_delay: 1000,
                    jitter: Some(Jitter::Full),
                });

                assert!(calculate_delay(&100, &0, &policy) < 100);
                assert!(calculate_delay(&100, &1, &policy) < 200);
                assert!(calculate_delay(&100, &2, &policy) < 400);
                assert!(calculate_delay(&100, &3, &policy) < 800);
                // limit to max_delay
                assert!(calculate_delay(&100, &4, &policy) < 1000);
            }

            #[test]
            fn test_without_full_jitter() {
                let policy = RetryPolicy::ExponentialBackoff(ExponentialBackoff {
                    retries: Retry::NTimes(4),
                    delay: 100,
                    base: Some(2),
                    max_delay: 1000,
                    jitter: None,
                });

                assert_eq!(calculate_delay(&100, &0, &policy), 100);
                assert_eq!(calculate_delay(&100, &1, &policy), 200);
                assert_eq!(calculate_delay(&100, &2, &policy), 400);
                assert_eq!(calculate_delay(&100, &3, &policy), 800);
                // limit to max_delay
                assert_eq!(calculate_delay(&100, &4, &policy), 1000);
            }
        }

        mod no_backoff_test {
            use super::calculate_delay;
            use super::{RetryPolicy, NoBackoff, Retry};
            #[test]
            fn test() {
                let policy = RetryPolicy::NoBackoff(NoBackoff {
                    retries: Retry::NTimes(4),
                });

                assert_eq!(calculate_delay(&100, &1, &policy), 0);
                assert_eq!(calculate_delay(&100, &2, &policy), 0);
                assert_eq!(calculate_delay(&100, &3, &policy), 0);
            }
        }

        mod fixed_backoff_test {
            use super::calculate_delay;
            use super::{RetryPolicy, FixedBackoff, Retry};
            #[test]
            fn test() {
                let policy = RetryPolicy::FixedBackoff(FixedBackoff {
                    retries: Retry::NTimes(4),
                    delay: 100,
                });

                assert_eq!(calculate_delay(&100, &1, &policy), 100);
                assert_eq!(calculate_delay(&100, &2, &policy), 100);
                assert_eq!(calculate_delay(&100, &3, &policy), 100);
            }
        }

        mod linear_backoff_test {
            use super::calculate_delay;
            use super::{RetryPolicy, LinearBackoff, Retry};
            #[test]
            fn test() {
                let policy = RetryPolicy::LinearBackoff(LinearBackoff {
                    retries: Retry::NTimes(4),
                    delay: 100,
                });

                assert_eq!(calculate_delay(&100, &0, &policy), 100);
                assert_eq!(calculate_delay(&100, &1, &policy), 200);
                assert_eq!(calculate_delay(&100, &2, &policy), 300);
                assert_eq!(calculate_delay(&100, &3, &policy), 400);
            }
        }
    }

    mod retry_async_test {
        use std::cell::Cell;

        #[tokio::test]
        async fn test_with_exponential_backoff() {
            use super::*;
            let policy = RetryPolicy::ExponentialBackoff(ExponentialBackoff {
                retries: Retry::NTimes(5),
                delay: 100,
                base: Some(8),
                max_delay: 1000,
                jitter: None,
            });

            let attempts = Cell::new(0);
            let timestamp = Cell::new(std::time::Instant::now());
            let result = retry_async(
                || async {
                    let now = std::time::Instant::now();
                    let delay = now - timestamp.get();
                    timestamp.set(now);
                    match attempts.get() {
                        0 => assert!(delay.as_millis() == 0),
                        1 => assert!(delay.as_millis() >= 100),
                        2 => assert!(delay.as_millis() >= 800),
                        3 => assert!(delay.as_millis() >= 1000 && delay.as_millis() < 1500),
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
                None,
            ).await;

            assert_eq!(result, Ok("Success"));
            assert_eq!(attempts.get(), 4);
        }

        #[tokio::test]
        async fn test_with_fixed_backoff() {
            use super::*;
            let policy = RetryPolicy::FixedBackoff(FixedBackoff {
                retries: Retry::NTimes(3),
                delay: 100,
            });

            let attempts = Cell::new(0);
            let timestamp = Cell::new(std::time::Instant::now());
            let result = retry_async(
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
                None,
            )
                .await;

            assert_eq!(result, Ok("Success"));
            assert_eq!(attempts.get(), 4);
        }

        #[tokio::test]
        async fn test_with_linear_backoff() {
            use super::*;
            let policy = RetryPolicy::LinearBackoff(LinearBackoff {
                retries: Retry::NTimes(3),
                delay: 100,
            });

            let attempts = Cell::new(0);
            let timestamp = Cell::new(std::time::Instant::now());
            let result = retry_async(
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
                None
            ).await;

            assert_eq!(result, Ok("Success"));
            assert_eq!(attempts.get(), 4);
        }

        #[tokio::test]
        async fn test_with_no_backoff() {
            use super::*;
            let policy = RetryPolicy::NoBackoff(NoBackoff {
                retries: Retry::NTimes(3),
            });

            let attempts = Cell::new(0);
            let timestamp = Cell::new(std::time::Instant::now());
            let result = retry_async(
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
                None
            ).await;

            assert_eq!(result, Ok("Success"));
            assert_eq!(attempts.get(), 4);
        }

        #[tokio::test]
        async fn test_with_unmatched_number_of_retries() {
            let number_of_retries = 5;
            use super::*;
            let policy = RetryPolicy::NoBackoff(NoBackoff {
                retries: Retry::NTimes(number_of_retries -3),
            });

            let attempts = Cell::new(0);
            let result = retry_async(
                || async {
                    attempts.set(attempts.get() + 1);
                    if attempts.get() < number_of_retries {
                        Err("Error")
                    } else {
                        Ok("Success")
                    }
                },
                &policy,
                None
            ).await;

            // The result should be an error since retries are less than the number of attempts
            assert_eq!(result, Err("Error"));
            assert_eq!(attempts.get(), number_of_retries - 2);
        }

        #[tokio::test]
        async fn test_terminate_early_with_error_filter() {
            use super::*;

            #[derive(Debug, Eq, PartialEq)]
            enum TestError {
                E1,
                E2,
                E3,
            }

            let retries = 4;

            // Backoff not important here, just required
            let policy = RetryPolicy::LinearBackoff(LinearBackoff {
                retries: Retry::NTimes(retries),
                delay: 0,
            });

            let attempts = Cell::new(0);

            // Does not match TestError::E3
            let retry_strategy = |e: &TestError, attempt: i16| {
                // Should be first attempt
                if matches!(e, TestError::E1) {
                    assert_eq!(attempt, 1);
                }

                // Should be second attempt
                if matches!(e, TestError::E2) {
                    assert_eq!(attempt, 2);
                }

                // Continue retries on the first 2 errors, then return result on
                // the third error
                if matches!(e, TestError::E1) || matches!(e, TestError::E2) {
                    return RetryStrategyAction::ContinueRetries
                }

                RetryStrategyAction::ReturnResult
            };
            
            let result = retry_async(
                || async {
                    attempts.set(attempts.get() + 1);

                    let err = match attempts.get() {
                        1 => Err(TestError::E1),
                        2 => Err(TestError::E2),
                        3 => Err(TestError::E3),
                        _ => Ok(()),
                    };

                    return err
                },
                &policy,
                retry_strategy,
            ).await;

            assert_eq!(attempts.get(), 3);
            assert_eq!(result, Err(TestError::E3));
        }
    }
}
