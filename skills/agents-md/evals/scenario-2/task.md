# Create AGENTS.md for Python FastAPI Project

## Problem/Feature Description

Your team maintains a Python-based microservice that provides REST API endpoints for a billing system. The service uses FastAPI, PostgreSQL, and Redis. Several AI assistants you've tried have been generating incorrect code because they assume Node.js/TypeScript stack based on common patterns they've seen.

The project lead wants you to create AGENTS.md documentation that will help AI assistants understand this is a Python project and provide accurate, working guidance. The documentation should be based on the actual technology stack, not assumptions.

## Output Specification

Create an `AGENTS.md` file in the repository root that provides:

- Correct Python/Poetry/Pip environment setup instructions
- How to run the FastAPI application
- How to run tests (pytest)
- Database migration commands
- Coding conventions

The key requirement: the documentation must accurately reflect the Python ecosystem, not assume Node.js or other frameworks.

## Input Files

The following files are provided as inputs. Extract them before beginning.

=============== FILE: pyproject.toml ===============
[tool.poetry]
name = "billing-service"
version = "2.0.0"
description = "Billing microservice API"
authors = ["Platform Team <platform@company.com>"]

[tool.poetry.dependencies]
python = "^3.10"
fastapi = "^0.100.0"
uvicorn = {extras = ["standard"], version = "^0.22.0"}
sqlalchemy = "^2.0.0"
alembic = "^1.11.0"
psycopg2-binary = "^2.9.0"
redis = "^4.5.0"
pydantic = "^2.0.0"
python-jose = {extras = ["cryptography"], version = "^3.3.0"}
passlib = "^1.7.4"
python-multipart = "^0.0.6"

[tool.poetry.group.dev.dependencies]
pytest = "^7.4.0"
pytest-asyncio = "^0.21.0"
pytest-cov = "^4.1.0"
httpx = "^0.24.0"
black = "^23.7.0"
ruff = "^0.0.275"
mypy = "^1.4.0"

[tool.poetry.scripts]
billing-service = "billing_service.main:app"

[tool.black]
line-length = 100
target-version = ["py310"]

[tool.ruff]
line-length = 100
select = ["E", "F", "I", "N", "W", "UP"]
ignore = ["E501"]

[tool.pytest.ini_options]
asyncio_mode = "auto"
testpaths = ["tests"]
python_files = ["test_*.py"]
python_classes = ["Test*"]
python_functions = ["test_*"]

[tool.mypy]
python_version = "3.10"
warn_return_any = true
warn_unused_configs = true
disallow_untyped_defs = false
