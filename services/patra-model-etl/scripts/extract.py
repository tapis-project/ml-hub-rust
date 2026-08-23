import os
import json
import time
import gzip
import sys
import urllib.error
import urllib.request
from datetime import datetime, date
from pathlib import Path

# https: // patrabackend.pods.icicleai.tapis.io/modelcards
# https: // patrabackend.pods.icicleai.tapis.io/modelcard/{uuid}


PATRA_BASE_URL = os.environ.get("PATRA_BASE_URL", "https://patrabackend.pods.icicleai.tapis.io")
INBOX = os.environ.get("INBOX", "inbox")
MAX_RECORDS = int(os.environ.get("MAX_RECORDS", -1))
os.makedirs(INBOX, exist_ok=True)

BATCH = 5000          # records per shard file
PAGE_SLEEP = 0.5      # small pause every ~page (1000 items)
buffer, shard = [], 0


def json_default(o):
    # Make common non-JSON types serializable
    if isinstance(o, (datetime, date)):
        return o.isoformat()
    if isinstance(o, (set, frozenset)):
        return list(o)
    if isinstance(o, Path):
        return str(o)
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


def fetch_patra_list():
    url = f"{PATRA_BASE_URL.rstrip('/')}/modelcards"
    req = urllib.request.Request(url)
    with urllib.request.urlopen(req, timeout=60) as resp:
        data = json.loads(resp.read().decode("utf-8"))
        if isinstance(data, list):
            return data

        raise ValueError("Patra /modelcards response did not contain a model list")


def fetch_patra_model(uuid):
    url = f"{PATRA_BASE_URL.rstrip('/')}/modelcard/{uuid}"
    req = urllib.request.Request(url)

    with urllib.request.urlopen(req, timeout=60) as resp:
        data = json.loads(resp.read().decode("utf-8"))
        if isinstance(data, dict):
            return data

        raise ValueError(f"Patra /modelcard/{uuid} response did not contain a model record")


try:
    records = fetch_patra_list()
except (urllib.error.URLError, urllib.error.HTTPError, TimeoutError, ValueError) as err:
    print(f"Error fetching Patra model list: {err}", file=sys.stderr)
    sys.exit(1)


i = 0


for short_rec in records:
    if MAX_RECORDS != -1 and i >= MAX_RECORDS:
        break

    rec_uuid = short_rec.get("uuid")

    try:
        rec = fetch_patra_model(rec_uuid)
    except (urllib.error.URLError, urllib.error.HTTPError, TimeoutError, ValueError) as err:
        print(f"Error fetching Patra model {rec_uuid}: {err}", file=sys.stderr)
        continue

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

if i == 0:
    print("No Patra model records were extracted", file=sys.stderr)
else:
    print(f"Extracted {i} Patra model records into {shard + 1} shard files", file=sys.stdout)
