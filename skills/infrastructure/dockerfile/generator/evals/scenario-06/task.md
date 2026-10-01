# Scenario 06: Audit and Rewrite a Bloated Python Dockerfile

## User Prompt

A data team inherited a Dockerfile for a FastAPI application (`analytics-api`) from a contractor. After a recent Docker image size audit, the operations team found the image is 2.1 GB — nearly ten times what they expected. The image is also flagged by the security scanner for including build tools, stale apt lists, and cached pip packages in the final image.

The team needs the Dockerfile rewritten from scratch so it produces a small, clean production image. The application uses Python 3.11, the package list is in `requirements.txt`, and the server starts with `uvicorn app.main:app --host 0.0.0.0 --port 8000`.

Produce a new `Dockerfile` for the `analytics-api` Python/FastAPI application.

Also produce an appropriate `.dockerignore` for a Python project.

Place both files in the current directory.

The original (problematic) Dockerfile is:

```dockerfile
FROM python:3.11

RUN apt-get update
RUN apt-get install -y curl gcc build-essential
RUN pip install --upgrade pip

ADD . /app
WORKDIR /app

RUN pip install -r requirements.txt

CMD uvicorn app.main:app --host 0.0.0.0 --port 8000
```
