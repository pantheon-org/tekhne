# Task: Generate a Multi-Directory C++ Project Makefile

You are building a top-level Makefile for a C++ project with three sub-components:

```
engine/
  src/   (produces libengine.a)
renderer/
  src/   (produces librenderer.a, depends on libengine.a)
app/
  src/   (produces bin/game, links libengine.a and librenderer.a)
```

Top-level Makefile requirements:
- Use $(MAKE) for all recursive calls into sub-directories, never bare make
- Build order: engine first, then renderer, then app (explicit dependency ordering)
- `all` builds everything in correct order
- `clean` calls $(MAKE) clean in each sub-directory
- `test` calls $(MAKE) test in each sub-directory
- `help` target present with ## comments

For the `app/` component Makefile (produce this as `app/Makefile`):
- Compiler: `CXX ?= g++`
- Source glob: `$(wildcard src/*.cpp)`
- Object files in `build/` directory using order-only prerequisite `| $(BUILDDIR)`
- Dependency tracking: `-MMD -MP` and `-include` of generated `.d` files
- Modern GNU Make header

Produce both the top-level `Makefile` and `app/Makefile`.
