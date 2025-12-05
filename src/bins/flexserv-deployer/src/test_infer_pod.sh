#!/usr/bin/env bash
set -euo pipefail


usage() {
    cat <<USAGE
Usage: $0 <command> --port=<port>

Commands:
    show_model    List locally available models (/v1/models)
    chat          Call /chat with a sample prompt
    chat_comp     Call /v1/chat/completions with sample messages
    responses     Call /v1/responses with sample input/instructions
    audio_trans   Call /v1/audio/transcriptions using $AUDIO_FILE

Environment overrides:
    BASE_URL     Base URL for the inference server (default: $BASE_URL)
    AUDIO_FILE   Audio file path for the transcription demo (default: $AUDIO_FILE)
USAGE
}


PORT=""
command=""
if [[ $# -lt 1 ]]; then
    usage; exit 1
fi

command=$1
shift

while [[ $# -gt 0 ]]; do
    case "$1" in
        --port=*) PORT="${1#--port=}" ;;
        --port)
            PORT="$2"
            shift
        ;;
        *)
            echo "Unknown option: $1" >&2
            usage
            exit 1
        ;;
    esac
    shift
done

if [[ -n "$PORT" ]]; then
    PORT=":${PORT}"
fi

BASE_HOST=${BASE_HOST:-"localhost"}
BASE_URL=${BASE_URL:-"https://${BASE_HOST}${PORT}"}
AUDIO_FILE=${AUDIO_FILE:-"thank_you_for_the_call.wav"}

case "$command" in
    show_model)
        curl -sS -X GET "$BASE_URL/v1/models" | jq .
    ;;
    chat)
        curl -sS -X POST "$BASE_URL/chat" \
        -H "Content-Type: application/json" \
        -d '{"message": "hi, just give me a random word"}' | jq .
    ;;
    chat_comp)
        curl -sS -X POST "$BASE_URL/v1/chat/completions" \
        -H "Content-Type: application/json" \
        -d '{"messages":[{"role":"user","content":"Suggest a two-word team name"}], "max_tokens": 20}' | jq .
    ;;
    responses)
        curl -sS -X POST "$BASE_URL/v1/responses" \
        -H "Content-Type: application/json" \
        -d '{"input":"I am Wei","instructions":"what is your name?","max_output_tokens": 20}' | jq .
    ;;
    audio_trans)
        if [[ ! -f "$AUDIO_FILE" ]]; then
            echo "Audio file '$AUDIO_FILE' not found. Set AUDIO_FILE to an existing file." >&2
            exit 1
        fi
        curl -sS -X POST "http://localhost:8000/v1/audio/transcriptions" \
        -F "file=@${AUDIO_FILE}" | jq .
    ;;
    *)
        echo "Unknown command: $command" >&2
        usage
        exit 1
    ;;
esac
