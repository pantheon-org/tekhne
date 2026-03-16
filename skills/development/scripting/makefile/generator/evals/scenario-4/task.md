# Task: Correct Anti-Patterns in an Existing Makefile

The following Makefile has been submitted for review. It contains several violations of the makefile-generator skill's anti-patterns and best practices. Identify each issue, explain why it is a problem, and produce a corrected version of the Makefile.

```makefile
CC = gcc
CFLAGS = -Wall -O2
PREFIX = /usr/local
ENVIRONMENT = production

all: build

build:
    gcc -o app main.c server.c

deploy:
    cd dist
    npm publish

release:
    make -C packages/core all
    make -C packages/ui all

clean:
    rm -rf build/

test:
    ./run_tests.sh

install:
    cp app $(PREFIX)/bin/app
```

Your response must:
1. List each anti-pattern found (name it, reference the skill rule it violates)
2. Show the corrected Makefile as a complete file
3. Confirm which validation checklist items the corrected Makefile satisfies
