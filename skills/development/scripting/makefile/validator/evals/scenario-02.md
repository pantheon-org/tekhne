# Scenario 02: Modernize a Legacy C Build Makefile

## User Prompt

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

## Expected Behavior

1. Add a `.PHONY` declaration covering at least `all`, `test`, `clean`, and `install`
2. Explain in `modernization-notes.md` that `make clean` silently did nothing on machines with a file named `clean`, caused by missing `.PHONY`
3. Ensure recipe lines start with hard tab characters (not spaces or inconsistent indentation)
4. Describe running a validation step (e.g., `make -n`, `validate_makefile.sh`, or `mbake`) on the original Makefile before making changes
5. Add `.DELETE_ON_ERROR` to clean up partial output files on failure
6. Document that the modernized Makefile was re-validated after changes
7. Mention using `mbake format` or an equivalent formatting step as part of the modernization process
8. List each specific change made (not just a generic summary)

## Success Criteria

- **.PHONY added**: The modernized Makefile includes a `.PHONY` declaration covering at least `all`, `test`, `clean`, and `install`
- **.PHONY issue explained**: `modernization-notes.md` explains that `make clean` silently did nothing on machines with a file named `clean`
- **Tab indentation present**: Recipe lines in the modernized Makefile start with hard tab characters
- **Validation step documented**: `modernization-notes.md` describes running a validation step on the original Makefile before making changes
- **.DELETE_ON_ERROR added**: The modernized Makefile includes `.DELETE_ON_ERROR`
- **Re-validation documented**: `modernization-notes.md` documents that the modernized Makefile was re-validated after changes
- **Format step mentioned**: `modernization-notes.md` mentions using `mbake format` or an equivalent formatting step
- **Changes enumerated**: `modernization-notes.md` lists each specific change made

## Failure Conditions

- Agent does not add `.PHONY` to the modernized Makefile
- Agent does not explain why `make clean` silently did nothing on some machines
- Agent produces a modernized Makefile with spaces instead of tabs
- Agent does not document a validation step on the original Makefile
- Agent does not add `.DELETE_ON_ERROR`
- Agent provides only a generic summary instead of enumerating specific changes
