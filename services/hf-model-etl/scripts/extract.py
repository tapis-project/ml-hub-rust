import dataclasses
import gzip
import json
import os
from datetime import date, datetime
from pathlib import Path

from huggingface_hub import HfApi

from rate_limit import retry_rate_limited


HF_TOKEN = os.environ.get("HF_TOKEN")
INBOX = os.environ.get("INBOX", "inbox")
MAX_RECORDS = int(os.environ.get("MAX_RECORDS", -1))
PROGRESS_INTERVAL = int(os.environ.get("PROGRESS_INTERVAL", 100))
RATE_LIMIT_MAX_RETRIES = int(os.environ.get("HF_RATE_LIMIT_MAX_RETRIES", 5))
RATE_LIMIT_FALLBACK_SECONDS = int(
    os.environ.get("HF_RATE_LIMIT_FALLBACK_SECONDS", 60)
)
RATE_LIMIT_MAX_WAIT_SECONDS = int(
    os.environ.get("HF_RATE_LIMIT_MAX_WAIT_SECONDS", 300)
)
BATCH_SIZE = 5000

if PROGRESS_INTERVAL <= 0:
    raise ValueError("PROGRESS_INTERVAL must be greater than zero")


os.makedirs(INBOX, exist_ok=True)

api = HfApi(token=HF_TOKEN)


def json_default(value):
    if isinstance(value, (datetime, date)):
        return value.isoformat()
    if isinstance(value, (set, frozenset)):
        return list(value)
    if isinstance(value, Path):
        return str(value)
    if dataclasses.is_dataclass(value):
        return dataclasses.asdict(value)
    if isinstance(value, (bytes, bytearray)):
        return value.decode("utf-8", errors="replace")

    return str(value)


def save_jsonl(records, shard_index, compress=False):
    filename = f"models_{shard_index:05d}.jsonl"
    path = os.path.join(INBOX, filename if not compress else filename + ".gz")
    opener = gzip.open if compress else open

    with opener(path, "wt", encoding="utf-8") as output:
        for record in records:
            output.write(
                json.dumps(record, ensure_ascii=False, default=json_default) + "\n"
            )

    return path


def print_progress(examined, emitted, skipped, metadata_errors, current_model):
    target = "unlimited" if MAX_RECORDS == -1 else str(MAX_RECORDS)

    print(
        "Hugging Face model extraction progress: "
        f"examined={examined}, "
        f"emitted={emitted}/{target}, "
        f"skipped={skipped}, "
        f"metadata_errors={metadata_errors}, "
        f"current_model={current_model}",
        flush=True,
    )


buffer = []
shard = 0
examined = 0
emitted = 0
skipped = 0
metadata_errors = 0

print(
    "Starting Hugging Face model extraction: "
    f"max_records={MAX_RECORDS}, progress_interval={PROGRESS_INTERVAL}",
    flush=True,
)

models = api.list_models(
    full=True,
    cardData=True,
    fetch_config=True,
    limit=None,
    sort="likes",
)

for summary in models:
    if MAX_RECORDS != -1 and emitted >= MAX_RECORDS:
        break

    examined += 1

    if examined == 1 or examined % PROGRESS_INTERVAL == 0:
        print_progress(
            examined,
            emitted,
            skipped,
            metadata_errors,
            summary.id,
        )

    if summary.private or summary.gated:
        skipped += 1

        continue

    model_id = summary.id
    revision = summary.sha

    try:
        details = retry_rate_limited(
            lambda: api.model_info(
                model_id,
                revision=revision,
            ),
            description=f"fetching metadata for {model_id}",
            max_retries=RATE_LIMIT_MAX_RETRIES,
            fallback_seconds=RATE_LIMIT_FALLBACK_SECONDS,
            max_wait_seconds=RATE_LIMIT_MAX_WAIT_SECONDS,
        )
    except Exception as error:
        metadata_errors += 1

        print(
            f"Skipping {model_id}: unable to fetch metadata: {error}",
            flush=True,
        )

        continue

    if details.private or details.gated:
        skipped += 1

        continue

    if details.used_storage is None:
        skipped += 1

        print(
            f"Skipping {model_id}: storage size metadata is unavailable",
            flush=True,
        )

        continue

    buffer.append(dataclasses.asdict(details))
    emitted += 1

    if len(buffer) >= BATCH_SIZE:
        shard_path = save_jsonl(buffer, shard)

        print(
            f"Saved Hugging Face model shard: path={shard_path}, records={len(buffer)}",
            flush=True,
        )

        shard += 1
        buffer = []

if buffer:
    shard_path = save_jsonl(buffer, shard)

    print(
        f"Saved Hugging Face model shard: path={shard_path}, records={len(buffer)}",
        flush=True,
    )

print(
    "Hugging Face model extraction complete: "
    f"examined={examined}, "
    f"emitted={emitted}, "
    f"skipped={skipped}, "
    f"metadata_errors={metadata_errors}",
    flush=True,
)
