import math
import re
import time
from datetime import datetime, timezone
from email.utils import parsedate_to_datetime


RATE_LIMIT_STATUS = 429
RATE_LIMIT_RESET_PATTERN = re.compile(
    r"(?:^|[;,])\s*t\s*=\s*(\d+(?:\.\d+)?)"
)


def retry_after_seconds(error, fallback_seconds, retry_index, now=None):
    response = getattr(error, "response", None)
    if response is None or response.status_code != RATE_LIMIT_STATUS:
        return None

    headers = response.headers
    retry_after = headers.get("Retry-After")
    if retry_after:
        parsed_retry_after = _parse_retry_after(retry_after, now)
        if parsed_retry_after is not None:
            return parsed_retry_after

    rate_limit = headers.get("RateLimit")
    if rate_limit:
        match = RATE_LIMIT_RESET_PATTERN.search(rate_limit)
        if match:
            return math.ceil(float(match.group(1)))

    return fallback_seconds * (2**retry_index)


def retry_rate_limited(
    operation,
    description,
    max_retries,
    fallback_seconds,
    max_wait_seconds,
    sleep=time.sleep,
):
    retry_index = 0

    while True:
        try:
            return operation()
        except Exception as error:
            delay = retry_after_seconds(error, fallback_seconds, retry_index)
            if delay is None or retry_index >= max_retries:
                raise

            delay = min(max(delay, 1), max_wait_seconds)
            retry_number = retry_index + 1
            print(
                f"Hugging Face rate limit reached while {description}; "
                f"retrying in {delay} seconds "
                f"({retry_number}/{max_retries})",
                flush=True,
            )

            sleep(delay)
            retry_index += 1


def _parse_retry_after(value, now=None):
    try:
        return math.ceil(float(value))
    except ValueError:
        pass

    try:
        retry_at = parsedate_to_datetime(value)
    except (TypeError, ValueError):
        return None

    if retry_at.tzinfo is None:
        retry_at = retry_at.replace(tzinfo=timezone.utc)

    current_time = now or datetime.now(timezone.utc)

    return max(math.ceil((retry_at - current_time).total_seconds()), 0)
