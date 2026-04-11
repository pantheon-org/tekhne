# Scenario 03: Layer Ordering and Cache Efficiency Audit (Stage 3/4)

## User Prompt

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

Perform Stage 3 (Best Practices Validation) with a focus on layer ordering and build cache efficiency.

1. Identify the cache busting problem caused by the COPY ordering. Explain what happens to the pip install layer every time source code changes.

2. Identify the cache cleanup layer ordering problem. Explain why the separate `RUN rm -rf /var/lib/apt/lists/*` on its own line does NOT reduce the image size.

3. Identify the split pip install issue (requirements.txt installed separately from gunicorn).

4. Verify the USER directive is present and assess whether it is correctly placed (before or after COPY/RUN).

5. For each issue, provide a corrected code snippet demonstrating the fix.

6. Produce a brief summary of how many layers would be saved by applying the fixes.

## Expected Behavior

1. Identify that `COPY . /app` before `RUN pip install -r requirements.txt` causes the pip install cache to be invalidated on every source code change, and propose copying requirements.txt first
2. Correctly explain that `RUN rm -rf /var/lib/apt/lists/*` in a separate RUN instruction creates a new layer and does not reduce the size of the layer where apt-get ran; the cleanup must be in the same RUN layer
3. Identify that running pip install twice (once for requirements.txt, once for gunicorn) creates unnecessary layers and recommend combining them into one RUN or adding gunicorn to requirements.txt
4. Note the USER directive is present but assess whether it is correctly placed relative to COPY and RUN instructions
5. Provide at least one corrected code block combining apt-get update, install, and cache cleanup in a single RUN layer

## Success Criteria

- **COPY ordering cache bust identified**: Agent identifies that `COPY . /app` before `RUN pip install -r requirements.txt` causes the pip install cache to be invalidated on every source code change, and proposes copying requirements.txt first
- **Separate cache cleanup layer explained**: Agent correctly explains that `RUN rm -rf /var/lib/apt/lists/*` in a separate RUN instruction creates a new layer and does not reduce the size of the layer where apt-get ran; the cleanup must be in the same RUN layer
- **Split pip install identified**: Agent identifies that running pip install twice (once for requirements.txt, once for gunicorn) creates unnecessary layers and recommends combining into one RUN or adding gunicorn to requirements.txt
- **USER placement assessed correctly**: Agent notes the USER directive is present (good) but placed after all COPY and RUN instructions, which is the correct position for a non-root runtime user
- **Corrected code snippet provided**: Agent provides at least one corrected code block combining apt-get update, install, and cache cleanup in a single RUN layer

## Failure Conditions

- Cache busting caused by COPY ordering is not identified
- The explanation for the separate cache cleanup layer is missing or incorrect (agent does not explain that the separate RUN does not reduce layer size)
- Split pip install calls are not flagged as creating unnecessary layers
- USER placement is not assessed, or the assessment is incorrect
- No corrected code snippet is provided
