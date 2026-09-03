import dataclasses
import gzip
import json
import os
import time
from datetime import date, datetime
from pathlib import Path

from huggingface_hub import HfApi


HF_TOKEN = os.environ.get("HF_TOKEN")
INBOX = os.environ.get("INBOX", "inbox")
MAX_RECORDS = int(os.environ.get("MAX_RECORDS", -1))
BATCH_SIZE = 1000
PAGE_SLEEP_SECONDS = 301
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
    filename = f"datasets_{shard_index:05d}.jsonl"
    path = os.path.join(INBOX, filename if not compress else filename + ".gz")

    opener = gzip.open if compress else open
    with opener(path, "wt", encoding="utf-8") as output:
        for record in records:
            output.write(json.dumps(record, ensure_ascii=False, default=json_default) + "\n")
    return path


def extract_record(summary):
    if summary.private or summary.gated:
        return None
    details = api.dataset_info(
        repo_id=summary.id,
        revision=summary.sha,
        files_metadata=True,
    )
    if not details.sha:
        raise ValueError(f"Dataset {summary.id} did not include a revision SHA")
    siblings = []
    for sibling in details.siblings or []:
        if sibling.size is None:
            raise ValueError(
                f"Dataset {summary.id} item {sibling.rfilename} did not include its size"
            )
        siblings.append({"rfilename": sibling.rfilename, "size": sibling.size})

    return {
        "id": details.id,
        "sha": details.sha,
        "tags": details.tags or [],
        "private": bool(details.private),
        "gated": bool(details.gated),
        "siblings": siblings,
    }

buffer = []
shard = 0
emitted = 0
examined = 0

result = api.list_datasets(full=True, limit=1000, sort="likes")
for dataset in result:
    if MAX_RECORDS != -1 and emitted >= MAX_RECORDS:
        break

    examined += 1
    try:
        record = extract_record(dataset)
    except Exception as error:
        continue

    if record is None:
        continue

    buffer.append(record)
    emitted += 1
    if examined % 1000 == 0:
        time.sleep(PAGE_SLEEP_SECONDS)

    if len(buffer) >= BATCH_SIZE:
        save_jsonl(buffer, shard)
        shard += 1
        buffer = []

if buffer:
    save_jsonl(buffer, shard)

print(f"Hugging Face Dataset extraction complete: examined={examined}, emitted={emitted}")
