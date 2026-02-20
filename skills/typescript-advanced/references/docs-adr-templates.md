# Architectural Decision Records (ADR) Templates

## ADR Template

```markdown
# ADR-{number}: {Title}

**Status:** {Proposed | Accepted | Deprecated | Superseded}
**Date:** {YYYY-MM-DD}
**Deciders:** {List of people involved}
**Technical Story:** {Link to issue/ticket}

## Context

What is the issue we're facing in a given context?
- What are the current constraints?
- What are the requirements?
- What forces are at play (technical, business, political)?

## Decision

What is the change we're proposing/making?
- Be specific and concrete
- Use active voice: "We will..."

## Consequences

What becomes easier or more difficult to do because of this change?

### Positive
- Benefit 1
- Benefit 2

### Negative
- Drawback 1
- Drawback 2

### Neutral
- Implication 1

## Alternatives Considered

What other options did we evaluate?

### Alternative 1: {Name}
- Description
- Pros
- Cons
- Why rejected

### Alternative 2: {Name}
- Description
- Pros
- Cons
- Why rejected

## Compliance

How will we ensure this decision is followed?
- Code review checks
- Automated tests
- Linting rules
- Documentation updates

## References

- [Link to relevant docs]
- [Link to research]
- [Link to related ADRs]
```

## Example: State Management ADR

```markdown
# ADR-003: Use Zustand for Client State Management

**Status:** Accepted
**Date:** 2024-01-15
**Deciders:** Frontend Team
**Technical Story:** #234

## Context

Our React application currently uses Redux Toolkit for state management. As the app grows, we're experiencing:
- Boilerplate overhead (actions, reducers, selectors)
- Difficulty with TypeScript inference
- Complex async logic with thunks
- Large bundle size (~50KB)

We need a simpler, more TypeScript-friendly state management solution that:
- Reduces boilerplate
- Has excellent TypeScript support
- Is lightweight (<5KB)
- Handles async state easily

## Decision

We will adopt Zustand as our primary client state management library.

```typescript
// Simple, typed store
import { create } from 'zustand';

interface TodoStore {
  todos: Todo[];
  addTodo: (text: string) => void;
  removeTodo: (id: string) => void;
}

export const useTodoStore = create<TodoStore>((set) => ({
  todos: [],
  addTodo: (text) =>
    set((state) => ({
      todos: [...state.todos, { id: nanoid(), text, done: false }],
    })),
  removeTodo: (id) =>
    set((state) => ({
      todos: state.todos.filter((todo) => todo.id !== id),
    })),
}));
```

## Consequences

### Positive
- **Less boilerplate**: No actions, reducers, or providers
- **Better TypeScript**: Automatic type inference
- **Smaller bundle**: 3KB vs 50KB (94% reduction)
- **Simpler async**: No middleware needed
- **Easier testing**: Stores are just functions

### Negative
- **Team learning curve**: New library to learn
- **Migration effort**: Need to migrate existing Redux stores
- **Middleware ecosystem**: Smaller than Redux
- **DevTools**: Less mature than Redux DevTools

### Neutral
- **No Provider**: Stores are hooks, not context
- **Multiple stores**: Encouraged over single store

## Alternatives Considered

### Alternative 1: Keep Redux Toolkit
- **Pros**: Team knows it, robust ecosystem, proven at scale
- **Cons**: Heavy boilerplate, bundle size, TypeScript friction
- **Why rejected**: Pain points outweigh familiarity

### Alternative 2: Jotai
- **Pros**: Atom-based, very TypeScript-friendly, tiny bundle
- **Cons**: Atom composition can be complex, less intuitive API
- **Why rejected**: Learning curve steeper than Zustand

### Alternative 3: Valtio
- **Pros**: Proxy-based reactivity, minimal API
- **Cons**: Proxy compatibility issues, less predictable updates
- **Why rejected**: Prefer explicit update model

## Compliance

- **Code review**: Ensure new state uses Zustand patterns
- **Migration plan**: Migrate one store per sprint
- **Documentation**: Create usage guide with TypeScript examples
- **Linting**: Add ESLint rule to prevent new Redux stores
- **Testing**: Document testing patterns for Zustand stores

## References

- [Zustand Documentation](https://docs.pmnd.rs/zustand)
- [Bundle Size Comparison](https://bundlephobia.com/package/zustand)
- [TypeScript Guide](https://docs.pmnd.rs/zustand/guides/typescript)
- [Migration from Redux](https://docs.pmnd.rs/zustand/guides/migration)
```

## Example: TypeScript Configuration ADR

```markdown
# ADR-007: Enable Strict TypeScript Mode

**Status:** Accepted
**Date:** 2024-02-01
**Deciders:** Engineering Team
**Technical Story:** #456

## Context

Our TypeScript configuration currently has:
- `strict: false`
- Mix of implicit `any` types
- Nullable values not handled consistently
- Type errors caught late in CI

This leads to:
- Runtime errors that TypeScript should catch
- Unclear function contracts
- Difficult refactoring

## Decision

We will enable strict mode in `tsconfig.json`:

```json
{
  "compilerOptions": {
    "strict": true,
    "noImplicitAny": true,
    "strictNullChecks": true,
    "strictFunctionTypes": true,
    "strictBindCallApply": true,
    "strictPropertyInitialization": true,
    "noImplicitThis": true,
    "alwaysStrict": true,
    "noUncheckedIndexedAccess": true,
    "exactOptionalPropertyTypes": true
  }
}
```

## Consequences

### Positive
- **Fewer runtime errors**: Catch nulls, undefined access
- **Better refactoring**: Type system catches breaking changes
- **Clearer contracts**: Function signatures more explicit
- **Improved IDE support**: Better autocomplete and hints

### Negative
- **Migration effort**: Fix ~500 existing type errors
- **Slower initial development**: More type annotations needed
- **Learning curve**: Team needs to understand strict mode

### Neutral
- **More verbose**: Some type annotations required
- **Third-party types**: May need `@ts-expect-error` for bad types

## Alternatives Considered

### Alternative 1: Gradual adoption
- Enable one strict flag at a time
- **Why rejected**: Piecemeal approach drags out migration

### Alternative 2: Stay non-strict
- Continue with `strict: false`
- **Why rejected**: Technical debt accumulates

## Compliance

- **Migration plan**: Fix errors by module over 4 sprints
- **CI enforcement**: Block merges with type errors
- **Documentation**: Create strict mode guide
- **Pair programming**: Help team learn strict patterns

## References

- [TypeScript Strict Mode](https://www.typescriptlang.org/tsconfig#strict)
- [Migrating to Strict](https://www.typescriptlang.org/docs/handbook/migrating-from-javascript.html)
```

## Best Practices

1. **Number ADRs sequentially** - Easy to reference
2. **Update status** - Mark when superseded
3. **Document context fully** - Explain constraints and forces
4. **Be specific** - Concrete decisions, not vague statements
5. **List alternatives** - Show due diligence
6. **Include code examples** - Show how decision manifests in code
7. **Define compliance** - Make decision enforceable
8. **Link to references** - Provide supporting material
9. **Keep concise** - 1-2 pages maximum
10. **Commit to repo** - Version control decisions
