# Audit and Rewrite a Bloated Python Dockerfile

## Problem/Feature Description

A data team inherited a Dockerfile for a FastAPI application (`analytics-api`) from a contractor. After a recent Docker image size audit, the operations team found the image is 2.1 GB — nearly ten times what they expected. The image is also flagged by the security scanner for including build tools, stale apt lists, and cached pip packages in the final image.

The team needs the Dockerfile rewritten from scratch so it produces a small, clean production image. The application uses Python 3.11, the package list is in `requirements.txt`, and the server starts with `uvicorn app.main:app --host 0.0.0.0 --port 8000`.

## Output Specification

Produce a new `Dockerfile` for the `analytics-api` Python/FastAPI application.

Also produce an appropriate `.dockerignore` for a Python project.

Place both files in the current directory.

## Input Files

The following files are provided as inputs. Extract them before beginning.

=============== FILE: Dockerfile.old ===============
FROM python:3.11

RUN apt-get update
RUN apt-get install -y curl gcc build-essential
RUN pip install --upgrade pip

ADD . /app
WORKDIR /app

RUN pip install -r requirements.txt

CMD uvicorn app.main:app --host 0.0.0.0 --port 8000
=============== FILE: requirements.txt ===============
fastapi==0.110.0
uvicorn[standard]==0.29.0
sqlalchemy==2.0.28
pydantic==2.6.3
httpx==0.27.0
=============== FILE: app/main.py ===============
from fastapi import FastAPI

app = FastAPI()

@app.get("/health")
def health():
    return {"status": "ok"}

@app.get("/analytics")
def analytics():
    return {"events": []}
