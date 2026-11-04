# The environment that records the expected output of the python schema validators: python 3.12 and PyYAML 6.0.2, pinned
# by digest, with the locale fixed so the output does not depend on the machine.
FROM python:3.12-slim@sha256:b18b261e1b4974b4f321451ad5251ded85abb22c2b8b262da527dda1d031ab30
ENV PYTHONUTF8=1 LC_ALL=C.UTF-8
RUN pip install --no-cache-dir PyYAML==6.0.2
