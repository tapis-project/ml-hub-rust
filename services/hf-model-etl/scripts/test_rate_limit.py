import unittest
from datetime import datetime, timezone

from rate_limit import retry_after_seconds, retry_rate_limited


class Response:
    def __init__(self, status_code, headers=None):
        self.status_code = status_code
        self.headers = headers or {}


class RequestError(Exception):
    def __init__(self, status_code, headers=None):
        self.response = Response(status_code, headers)


class RetryAfterSecondsTests(unittest.TestCase):
    def test_prefers_retry_after_seconds(self):
        error = RequestError(
            429,
            {
                "Retry-After": "17",
                "RateLimit": '"api|pages|resolvers";r=0;t=42',
            },
        )

        delay = retry_after_seconds(error, fallback_seconds=5, retry_index=0)

        self.assertEqual(delay, 17)

    def test_parses_retry_after_http_date(self):
        error = RequestError(
            429,
            {"Retry-After": "Wed, 21 Oct 2015 07:28:00 GMT"},
        )
        now = datetime(2015, 10, 21, 7, 27, 30, tzinfo=timezone.utc)

        delay = retry_after_seconds(
            error,
            fallback_seconds=5,
            retry_index=0,
            now=now,
        )

        self.assertEqual(delay, 30)

    def test_uses_hugging_face_rate_limit_reset(self):
        error = RequestError(
            429,
            {"RateLimit": '"api|pages|resolvers";r=0;t=42'},
        )

        delay = retry_after_seconds(error, fallback_seconds=5, retry_index=0)

        self.assertEqual(delay, 42)

    def test_uses_exponential_fallback_without_headers(self):
        error = RequestError(429)

        delay = retry_after_seconds(error, fallback_seconds=5, retry_index=2)

        self.assertEqual(delay, 20)

    def test_does_not_retry_other_errors(self):
        error = RequestError(500, {"Retry-After": "17"})

        delay = retry_after_seconds(error, fallback_seconds=5, retry_index=0)

        self.assertIsNone(delay)


class RetryRateLimitedTests(unittest.TestCase):
    def test_retries_rate_limit_and_honors_server_delay(self):
        calls = 0
        sleeps = []

        def operation():
            nonlocal calls
            calls += 1

            if calls == 1:
                raise RequestError(429, {"Retry-After": "7"})

            return "model"

        result = retry_rate_limited(
            operation,
            description="fetching a model",
            max_retries=2,
            fallback_seconds=5,
            max_wait_seconds=60,
            sleep=sleeps.append,
        )

        self.assertEqual(result, "model")
        self.assertEqual(calls, 2)
        self.assertEqual(sleeps, [7])

    def test_stops_after_retry_limit(self):
        calls = 0

        def operation():
            nonlocal calls
            calls += 1

            raise RequestError(429)

        with self.assertRaises(RequestError):
            retry_rate_limited(
                operation,
                description="fetching a model",
                max_retries=2,
                fallback_seconds=5,
                max_wait_seconds=60,
                sleep=lambda _: None,
            )

        self.assertEqual(calls, 3)


if __name__ == "__main__":
    unittest.main()
