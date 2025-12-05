#!/bin/bash

if [ -f "hf.env" ]; then
    source hf.env
    uvicorn main:app --host 0.0.0.0 --port 8000 --reload
else
    echo "hf.env not found. Please create hf.env from hf.env.template."
fi