from huggingface_hub import HfApi
import os, json, dataclasses, time, gzip
from datetime import datetime, date
from pathlib import Path

HF_TOKEN = os.environ.get("HF_TOKEN") 
INBOX = os.environ.get("MLHUB_INBOX", "inbox")
os.makedirs(INBOX, exist_ok=True)

api = HfApi(token=HF_TOKEN)

BATCH = 5000          # records per shard file
PAGE_SLEEP = 0.5      # small pause every ~page (1000 items)
page_counter = 0
buffer, shard = [], 0


def json_default(o):
    # Make common non-JSON types serializable
    if isinstance(o, (datetime, date)):
        return o.isoformat()
    if isinstance(o, (set, frozenset)):
        return list(o)
    if isinstance(o, Path):
        return str(o)
    # dataclasses nested somewhere
    if dataclasses.is_dataclass(o):
        return dataclasses.asdict(o)
    
    if isinstance(o, (bytes, bytearray)):
        return o.decode("utf-8", errors="replace")
    # fall back
    return str(o)

def save_jsonl(records, shard_idx, compress=False):
    fname = f"models_{shard_idx:05d}.jsonl"
    path = os.path.join(INBOX, fname if not compress else fname + ".gz")
    if compress:
        with gzip.open(path, "wt", encoding="utf-8") as f:
            for r in records:
                f.write(json.dumps(r, ensure_ascii=False, default=json_default) + "\n")
    else:
        with open(path, "w", encoding="utf-8") as f:
            for r in records:
                f.write(json.dumps(r, ensure_ascii=False, default=json_default) + "\n")
    return path

print(api)

i = 0
for info in api.list_models(full=True, cardData=True, fetch_config=True, limit=None, sort='last_modified', direction=-1):
    rec = dataclasses.asdict(info)
    buffer.append(rec)
    i += 1

    if i % 1000 == 0:
        time.sleep(PAGE_SLEEP) 

    if len(buffer) >= BATCH:
        save_jsonl(buffer, shard)
        shard += 1
        buffer = []

if buffer:
    save_jsonl(buffer, shard)