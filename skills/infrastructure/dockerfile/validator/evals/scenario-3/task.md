# Task: Layer Ordering and Cache Efficiency Audit (Stage 3/4)

You are given the following Dockerfile for a Python web application:

```dockerfile
FROM python:3.11-slim

WORKDIR /app

COPY . /app

RUN pip install -r requirements.txt

RUN apt-get update && apt-get install -y libpq-dev
RUN rm -rf /var/lib/apt/lists/*

RUN pip install gunicorn

USER appuser

EXPOSE 8000

CMD ["gunicorn", "--bind", "0.0.0.0:8000", "wsgi:app"]
```

## Your Task

Perform Stage 3 (Best Practices Validation) with a focus on layer ordering and build cache efficiency.

1. Identify the cache busting problem caused by the COPY ordering. Explain what happens to the pip install layer every time source code changes.

2. Identify the cache cleanup layer ordering problem. Explain why the separate `RUN rm -rf /var/lib/apt/lists/*` on its own line does NOT reduce the image size.

3. Identify the split pip install issue (requirements.txt installed separately from gunicorn).

4. Verify the USER directive is present and assess whether it is correctly placed (before or after COPY/RUN).

5. For each issue, provide a corrected code snippet demonstrating the fix.

6. Produce a brief summary of how many layers would be saved by applying the fixes.
