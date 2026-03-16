# Task: Generate a C Project Makefile

You are building a C application called `httpd` for a project with the following layout:

```
httpd/
├── src/
│   ├── main.c
│   ├── server.c
│   └── router.c
├── include/
│   └── httpd.h
└── tests/
    └── test_router.c
```

Requirements:
- Compiler: GCC, user-overridable
- Compile flags: `-Wall -Wextra -O2`, user-overridable
- Output binary: `build/httpd`
- Dependencies: auto-detected via `-MMD -MP` flags
- Standard targets: `all`, `install`, `clean`, `test`, `help`
- `install` copies the binary to `$(PREFIX)/bin` (PREFIX defaults to `/usr/local`)
- `help` prints available targets using `##` comment extraction
- Install prefix must be user-overridable with `?=`
- Use the Modern GNU Make header (not POSIX-compatible header)
- Use order-only prerequisites for the build directory

Produce a single `Makefile` that satisfies all the requirements above. Include a validation checklist at the end as a comment block showing which items were verified.
