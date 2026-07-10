---
name: fp-higher-order-functions
description: Master higher-order functions, map/filter/reduce patterns for expressive data transformations
allowed-tools:
  - Bash
  - Read
---

# Higher-Order Functions and Functional Patterns

Higher-order functions are functions that take other functions as arguments or return functions as results. They enable powerful abstractions, code reuse, and declarative programming styles. The classic trio of map, filter, and reduce form the foundation of functional data transformation.

## Understanding Higher-Order Functions

A higher-order function is any function that does at least one of the following:

1. **Accepts functions as parameters**
2. **Returns a function as its result**

This enables composition, abstraction, and powerful declarative patterns.

### JavaScript: Basic Higher-Order Functions

```javascript
// Function that takes a function as argument
function applyOperation(x, y, operation) {
  return operation(x, y);
}

const sum = applyOperation(5, 3, (a, b) => a + b);        // 8
const product = applyOperation(5, 3, (a, b) => a * b);    // 15

// Function that returns a function
function createMultiplier(factor) {
  return function(number) {
    return number * factor;
  };
}

const double = createMultiplier(2);
const triple = createMultiplier(3);

console.log(double(5));   // 10
console.log(triple(5));   // 15

// Practical example: Event handler creator
function createClickHandler(message) {
  return function(event) {
    console.log(message, event.target);
  };
}

const submitHandler = createClickHandler('Form submitted from:');
const cancelHandler = createClickHandler('Cancelled from:');

// Higher-order function for timing
function measureTime(fn) {
  return function(...args) {
    const start = performance.now();
    const result = fn(...args);
    const end = performance.now();
    console.log(`Execution time: ${end - start}ms`);
    return result;
  };
}

function slowCalculation(n) {
  let sum = 0;
  for (let i = 0; i < n; i++) {
    sum += i;
  }
  return sum;
}

const timedCalculation = measureTime(slowCalculation);
timedCalculation(1000000);  // Logs execution time
```

## Map: Transforming Collections

Map applies a function to each element in a collection, returning a new collection.

### JavaScript Map Patterns

```javascript
const numbers = [1, 2, 3, 4, 5];

// Basic map
const doubled = numbers.map(n => n * 2);
// [2, 4, 6, 8, 10]

// Map with objects
const users = [
  { id: 1, firstName: 'John', lastName: 'Doe' },
  { id: 2, firstName: 'Jane', lastName: 'Smith' },
  { id: 3, firstName: 'Bob', lastName: 'Johnson' }
];

const fullNames = users.map(user => `${user.firstName} ${user.lastName}`);
// ['John Doe', 'Jane Smith', 'Bob Johnson']

const userIds = users.map(user => user.id);
// [1, 2, 3]

// Map to different structure
const userCards = users.map(user => ({
  id: user.id,
  displayName: `${user.firstName} ${user.lastName}`,
  initials: `${user.firstName[0]}${user.lastName[0]}`
}));

// Map with index
const indexed = numbers.map((n, i) => ({ value: n, index: i }));

// Chaining maps
const result = numbers
  .map(n => n * 2)
  .map(n => n + 1)
  .map(n => n.toString());
// ['3', '5', '7', '9', '11']

// Practical: Price calculation
const products = [
  { name: 'Widget', price: 10, quantity: 2 },
  { name: 'Gadget', price: 15, quantity: 1 },
  { name: 'Doohickey', price: 7, quantity: 5 }
];

const lineItems = products.map(product => ({
  name: product.name,
  total: product.price * product.quantity
}));
```

## Filter: Selecting Elements

Filter creates a new collection containing only elements that satisfy a predicate.

### JavaScript Filter Patterns

```javascript
const numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

// Basic filter
const evens = numbers.filter(n => n % 2 === 0);
// [2, 4, 6, 8, 10]

const greaterThanFive = numbers.filter(n => n > 5);
// [6, 7, 8, 9, 10]

// Filter with objects
const users = [
  { id: 1, name: 'Alice', age: 25, active: true },
  { id: 2, name: 'Bob', age: 30, active: false },
  { id: 3, name: 'Charlie', age: 35, active: true },
  { id: 4, name: 'Diana', age: 28, active: true }
];

const activeUsers = users.filter(user => user.active);
const youngUsers = users.filter(user => user.age < 30);

// Complex predicates
const activeYoungUsers = users.filter(user =>
  user.active && user.age < 30
);

// Filter with negation
const inactiveUsers = users.filter(user => !user.active);

// Combining map and filter
const activeUserNames = users
  .filter(user => user.active)
  .map(user => user.name);
// ['Alice', 'Charlie', 'Diana']

// Practical: Search functionality
function searchUsers(users, query) {
  const lowerQuery = query.toLowerCase();
  return users.filter(user =>
    user.name.toLowerCase().includes(lowerQuery)
  );
}

// Remove nulls/undefined
const values = [1, null, 2, undefined, 3, null, 4];
const defined = values.filter(v => v != null);
// [1, 2, 3, 4]

// Or more strictly
const truthy = values.filter(Boolean);
```

## Reduce: Aggregating Values

Reduce (also called fold) processes a collection to produce a single value.

### JavaScript Reduce Patterns

```javascript
const numbers = [1, 2, 3, 4, 5];

// Sum
const sum = numbers.reduce((acc, n) => acc + n, 0);
// 15

// Product
const product = numbers.reduce((acc, n) => acc * n, 1);
// 120

// Maximum
const max = numbers.reduce((acc, n) => Math.max(acc, n), -Infinity);

// Minimum
const min = numbers.reduce((acc, n) => Math.min(acc, n), Infinity);

// Building objects from arrays
const users = [
  { id: 1, name: 'Alice' },
  { id: 2, name: 'Bob' },
  { id: 3, name: 'Charlie' }
];

const userMap = users.reduce((acc, user) => {
  acc[user.id] = user;
  return acc;
}, {});
// { 1: {id: 1, name: 'Alice'}, 2: {...}, 3: {...} }

// Grouping
const transactions = [
  { category: 'food', amount: 50 },
  { category: 'transport', amount: 30 },
  { category: 'food', amount: 25 },
  { category: 'entertainment', amount: 100 },
  { category: 'transport', amount: 15 }
];

const byCategory = transactions.reduce((acc, transaction) => {
  const category = transaction.category;
  if (!acc[category]) {
    acc[category] = [];
  }
  acc[category].push(transaction);
  return acc;
}, {});

// Count occurrences
const fruits = ['apple', 'banana', 'apple', 'orange', 'banana', 'apple'];
const counts = fruits.reduce((acc, fruit) => {
  acc[fruit] = (acc[fruit] || 0) + 1;
  return acc;
}, {});
// { apple: 3, banana: 2, orange: 1 }

// Flatten arrays
const nested = [[1, 2], [3, 4], [5, 6]];
const flattened = nested.reduce((acc, arr) => acc.concat(arr), []);
// [1, 2, 3, 4, 5, 6]

// Or use flat()
const flattenedModern = nested.flat();

// Composing functions
const compose = (...fns) =>
  fns.reduce((f, g) => (...args) => f(g(...args)));

const addOne = x => x + 1;
const double = x => x * 2;
const square = x => x * x;

const composed = compose(square, double, addOne);
console.log(composed(3));  // ((3 + 1) * 2)^2 = 64

// Pipeline (left to right)
const pipe = (...fns) =>
  fns.reduce((f, g) => (...args) => g(f(...args)));

const piped = pipe(addOne, double, square);
console.log(piped(3));  // ((3 + 1) * 2)^2 = 64
```

## Language-Specific Examples and Advanced Patterns

The guidance above uses JavaScript for the primary examples. For the same patterns in other languages and deeper material, see:

- [Higher-Order Functions in Python and Elixir](references/language-examples.md)
- [Advanced Higher-Order Function Patterns](references/advanced-patterns.md)

## Mindset

Express *what* a transformation produces, not *how* to iterate. Reach for `map`/`filter`/`reduce` and small composed functions before an index loop with a mutable accumulator. Know **when not to**: a genuinely side-effecting loop (streaming I/O) is clearer left imperative.

## Anti-Patterns

### NEVER hand-roll an index loop when map/filter/reduce expresses the intent

- WHY: imperative loops with mutable accumulators hide the transformation and invite off-by-one and aliasing bugs.
- BAD: `for (let i = 0; i < xs.length; i++) { out.push(xs[i] * 2); }`
- GOOD: `const out = xs.map((x) => x * 2);`

### NEVER cram filtering, mapping, and aggregation into one giant callback

- WHY: monolithic callbacks are hard to test and reuse; composition keeps each step verifiable.
- BAD: one `reduce` that filters, transforms, and groups at once.
- GOOD: compose small `filter`, then `map`, then `reduce` steps (or `pipe`).

### NEVER mutate the source collection inside a map/filter/reduce callback

- WHY: these are pure transformations; mutating the input breaks referential transparency.
- BAD: `xs.map((x) => { x.seen = true; return x; })`
- GOOD: `xs.map((x) => ({ ...x, seen: true }))`

### ALWAYS keep callbacks pure and side-effect-free

- WHY: pure callbacks make transformations predictable, testable, and safe to reorder.

## When to Use This Skill

- Transforming collections of data
- Building data processing pipelines
- Implementing business logic declaratively
- Creating reusable function utilities
- Abstracting common patterns
- Event handling and callbacks
- Middleware and plugin systems
- State management transformations
- API data processing
- Functional reactive programming

## Best Practices

1. **Prefer built-in functions** - Use language/library map, filter, reduce when available
2. **Chain operations** - Combine map, filter, reduce for clear data flows
3. **Keep functions pure** - Higher-order functions work best with pure functions
4. **Name intermediate functions** - Makes pipelines more readable
5. **Use meaningful parameter names** - Even in lambdas, clarity matters
6. **Compose small functions** - Build complex operations from simple ones
7. **Consider performance** - Multiple passes might be less efficient than reduce
8. **Use type signatures** - Document input/output types of higher-order functions
9. **Avoid deep nesting** - Extract lambdas to named functions when complex
10. **Leverage currying** - Create specialized functions from general ones
11. **Think declaratively** - Focus on what, not how
12. **Use consistent ordering** - Data last (point-free style) or data first
13. **Document side effects** - If unavoidable, make them explicit
14. **Test thoroughly** - Higher-order functions should have comprehensive tests
15. **Use appropriate abstractions** - Don't over-engineer simple iterations

## Common Pitfalls

1. **Overusing reduce** - Map/filter may be clearer for many operations
2. **Missing initial value** - Reduce without initial value can fail on empty arrays
3. **Mutating in callbacks** - Keep map/filter/reduce callbacks pure
4. **Performance negligence** - Multiple passes can be expensive on large datasets
5. **Over-chaining** - Too many operations reduce readability
6. **Ignoring short-circuits** - Some, every, find can be more efficient than filter
7. **Wrong composition order** - Compose is right-to-left, pipe is left-to-right
8. **Callback complexity** - Extract complex logic to named functions
9. **Type confusion** - Track types through transformation chains
10. **Side effects in predicates** - Filter/some/every predicates should be pure
11. **Reassigning accumulator** - In reduce, return new value, don't mutate
12. **Forgetting to return** - In reduce/map callbacks, must return value
13. **Mixing paradigms** - Inconsistent functional/imperative mixing
14. **Premature abstraction** - Don't create higher-order functions too early
15. **Ignoring alternatives** - Sometimes a for loop is clearer

## Resources

- "Functional-Light JavaScript" by Kyle Simpson
- "Professor Frisby's Mostly Adequate Guide to Functional Programming"
- MDN Web Docs: Array methods
- "JavaScript Allongé" by Reginald Braithwaite
- Ramda.js documentation
- Lodash/FP documentation
- "Haskell Programming from First Principles"
- "Learn You a Haskell for Great Good"
- "Programming Elixir" by Dave Thomas
- Python functools documentation
- "Functional Programming in Python" by David Mertz
