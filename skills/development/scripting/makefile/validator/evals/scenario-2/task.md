# Modernize a Legacy C Build Makefile

## Problem/Feature Description

A systems engineering team is onboarding a legacy C project from an acquired company. The project's Makefile was written in the early 2000s and has not been touched since. The new team needs to bring it up to current standards so it integrates with their CI/CD pipeline, which runs `make test` and `make clean` as pre-deployment checks. Several engineers have reported intermittent issues where `make clean` does nothing on some machines, and the build occasionally fails in parallel mode (`make -j4`). The team lead wants a modernized, validated version of the Makefile before it enters their monorepo.

Review and modernize the Makefile below. Produce an updated version and a `modernization-notes.md` describing the process you followed and each change made.

## Output Specification

Produce:
- A modernized `Makefile`
- A `modernization-notes.md` describing the validation/modernization steps taken and each issue found

## Input Files

The following file is provided as input. Extract it before beginning.

=============== FILE: inputs/Makefile ===============
CC = gcc
CFLAGS = -Wall -O2
OBJS = main.o utils.o parser.o
TARGET = myapp

all: $(TARGET)

$(TARGET): $(OBJS)
        $(CC) $(CFLAGS) -o $(TARGET) $(OBJS)

main.o: main.c
        $(CC) $(CFLAGS) -c main.c

utils.o: utils.c
        $(CC) $(CFLAGS) -c utils.c

parser.o: parser.c
        $(CC) $(CFLAGS) -c parser.c

test:
        ./run_tests.sh

clean:
        rm -f $(TARGET) $(OBJS)

install:
        cp $(TARGET) /usr/local/bin/

.DEFAULT_GOAL = all
