---
name: fp-immutability
description: Apply immutability principles across languages for safer, more predictable code
allowed-tools:
  - Bash
  - Read
---

# Immutability Principles

Immutability is a cornerstone of functional programming where data cannot be modified after creation. Instead of changing existing values, you create new values. This approach eliminates entire classes of bugs related to shared mutable state, makes code easier to reason about, and enables safe concurrent programming.

## Core Concepts of Immutability

Immutable data has several key benefits:

1. **Predictability**: Data doesn't change unexpectedly
2. **Thread Safety**: No race conditions with immutable data
3. **Easier Debugging**: State changes are explicit and traceable
4. **Temporal Logic**: Can keep historical versions of data
5. **Caching**: Safe to cache references to immutable data

### JavaScript: Immutability Patterns

```javascript
// MUTABLE APPROACH (avoid)
const user = {
  name: 'Alice',
  email: 'alice@example.com',
  addresses: []
};

function addAddress(user, address) {
  user.addresses.push(address);  // Mutation!
  return user;
}

// IMMUTABLE APPROACH (prefer)
const userImmutable = {
  name: 'Alice',
  email: 'alice@example.com',
  addresses: []
};

function addAddressImmutable(user, address) {
  return {
    ...user,
    addresses: [...user.addresses, address]
  };
}

// Usage
const user1 = { name: 'Bob', addresses: [] };
const user2 = addAddressImmutable(user1, '123 Main St');

console.log(user1.addresses.length);  // 0 - unchanged
console.log(user2.addresses.length);  // 1 - new object

// Updating nested structures
const state = {
  user: {
    profile: {
      name: 'Charlie',
      settings: {
        theme: 'dark',
        notifications: true
      }
    }
  }
};

// MUTABLE (avoid)
function toggleNotificationsMutable(state) {
  state.user.profile.settings.notifications = !state.user.profile.settings.notifications;
  return state;
}

// IMMUTABLE (prefer)
function toggleNotificationsImmutable(state) {
  return {
    ...state,
    user: {
      ...state.user,
      profile: {
        ...state.user.profile,
        settings: {
          ...state.user.profile.settings,
          notifications: !state.user.profile.settings.notifications
        }
      }
    }
  };
}

// Using Immer library for simpler deep updates
import { produce } from 'immer';

function toggleNotificationsImmer(state) {
  return produce(state, draft => {
    draft.user.profile.settings.notifications = !draft.user.profile.settings.notifications;
  });
}
```

### JavaScript: Immutable Collections

```javascript
// Array operations immutably
const numbers = [1, 2, 3, 4, 5];

// Adding elements
const withSix = [...numbers, 6];              // [1, 2, 3, 4, 5, 6]
const withZero = [0, ...numbers];             // [0, 1, 2, 3, 4, 5]

// Removing elements
const withoutThree = numbers.filter(n => n !== 3);  // [1, 2, 4, 5]
const withoutFirst = numbers.slice(1);              // [2, 3, 4, 5]
const withoutLast = numbers.slice(0, -1);           // [1, 2, 3, 4]

// Updating elements
const doubled = numbers.map(n => n * 2);      // [2, 4, 6, 8, 10]
const updateThird = [
  ...numbers.slice(0, 2),
  999,
  ...numbers.slice(3)
];  // [1, 2, 999, 4, 5]

// Object operations immutably
const person = {
  firstName: 'John',
  lastName: 'Doe',
  age: 30
};

// Adding/updating properties
const updated = { ...person, age: 31, city: 'Boston' };

// Removing properties
const { age, ...withoutAge } = person;

// Merging objects
const contact = { email: 'john@example.com', phone: '555-1234' };
const merged = { ...person, ...contact };

// Complex transformations
const users = [
  { id: 1, name: 'Alice', active: true },
  { id: 2, name: 'Bob', active: false },
  { id: 3, name: 'Charlie', active: true }
];

// Update specific user immutably
function updateUser(users, id, updates) {
  return users.map(user =>
    user.id === id ? { ...user, ...updates } : user
  );
}

const updatedUsers = updateUser(users, 2, { active: true });
```

## Immutability with Performance

Persistent data structures share structure for efficiency:

```elixir
# Elixir's persistent data structures share memory
defmodule Performance do
  # Adding to head is O(1)
  def prepend_efficient(list, item) do
    [item | list]
  end

  # Appending to tail is O(n)
  def append_slow(list, item) do
    list ++ [item]
  end

  # Building lists efficiently: collect then reverse
  def build_list_efficiently(n) do
    0..n
    |> Enum.reduce([], fn i, acc -> [i | acc] end)
    |> Enum.reverse()
  end

  # Maps are persistent hash trees - efficient updates
  def update_map_efficiently(map, key, value) do
    Map.put(map, key, value)  # O(log n)
  end
end

# Structural sharing example
original_map = %{a: 1, b: 2, c: 3, d: 4}
updated_map = %{original_map | a: 100}
# Only the changed part is new; rest is shared
```

## Language-Specific Examples and Advanced Patterns

The guidance above uses JavaScript for the primary examples. For the same patterns in other languages and deeper material, see:

- [Immutability in Python, Elixir, and Haskell](references/language-examples.md)

## Mindset

Treat data as values, not containers to mutate. Produce a new copy for every change and treat every argument as read-only. Know **when not to**: a local variable never shared outside its function is fine to mutate for a tight loop.

## Anti-Patterns

### NEVER mutate an object or array you were given

- WHY: callers share references; mutating in place corrupts state they still rely on.
- BAD: `state.user.name = "x";`
- GOOD: `return { ...state, user: { ...state.user, name: "x" } };`

### NEVER deep-clone a whole structure just to change one field

- WHY: full copies are wasteful; structural sharing copies only the changed path.
- BAD: `const next = JSON.parse(JSON.stringify(state)); next.a.b = 1;`
- GOOD: `const next = { ...state, a: { ...state.a, b: 1 } };`

### NEVER cache and hand out a shared mutable object

- WHY: any caller can mutate it and corrupt every other caller (an aliasing bug).
- BAD: return the same config object to all callers.
- GOOD: `Object.freeze` it, or return a fresh copy per caller.

### ALWAYS treat function arguments as read-only

- WHY: read-only arguments keep functions predictable and safe to reorder.

## When to Use This Skill

- Managing application state (Redux, state machines)
- Building concurrent or parallel systems
- Implementing undo/redo functionality
- Creating caching layers
- Developing time-travel debugging
- Writing predictable business logic
- Building reactive systems
- Implementing event sourcing
- Creating replayable operations
- Developing distributed systems

## Best Practices

1. **Default to immutability** - Make immutability the norm, mutability the exception
2. **Use language features** - Leverage frozen dataclasses, const, readonly, etc.
3. **Prefer immutable data structures** - Use libraries like Immutable.js, Immer, or pyrsistent
4. **Structural sharing** - Use persistent data structures for performance
5. **Update with new references** - Always return new objects rather than modifying
6. **Copy-on-write** - Only copy the parts that change
7. **Use builder patterns** - For complex object construction
8. **Avoid defensive copying** - Immutability eliminates the need
9. **Leverage type systems** - Use types to enforce immutability
10. **Document mutability** - If mutation is necessary, make it explicit
11. **Immutable by default** - Make immutability the path of least resistance
12. **Chain transformations** - Use pipelines with immutable steps
13. **Version your data** - Keep historical versions when needed
14. **Use lenses for deep updates** - Libraries like Ramda or Monocle
15. **Benchmark wisely** - Don't sacrifice immutability without profiling first

## Common Pitfalls

1. **Shallow copying** - Spread operator only copies one level deep
2. **Array methods confusion** - Some mutate (push, pop), others don't (map, filter)
3. **Reference sharing** - Forgetting that nested objects are still mutable
4. **Performance assumptions** - Immutability isn't always slower
5. **Over-copying** - Creating copies when references would suffice
6. **Ignoring structural sharing** - Not using persistent data structures
7. **Mixing paradigms** - Combining mutable and immutable approaches inconsistently
8. **Object.freeze() depth** - Only freezes top level in JavaScript
9. **Date/Regex mutability** - Some built-in objects are mutable in JavaScript
10. **Class instance mutation** - Methods that modify this
11. **Mutable default arguments** - In Python, mutable defaults are shared
12. **Tuple contents** - Tuples are immutable but can contain mutable objects
13. **Map/Set mutation** - Using mutating methods instead of immutable alternatives
14. **Unnecessary cloning** - Cloning when using persistent data structures
15. **Framework assumptions** - Some frameworks expect mutation (e.g., older React patterns)

## Resources

- "Functional Programming in JavaScript" by Luis Atencio
- Immutable.js documentation
- Immer library documentation
- "Persistent Data Structures" by Okasaki
- Redux documentation on immutability
- "Elixir in Action" by Saša Jurić
- "Haskell Programming from First Principles"
- ClojureScript rationale on persistent data structures
- "Purely Functional Data Structures" by Chris Okasaki
