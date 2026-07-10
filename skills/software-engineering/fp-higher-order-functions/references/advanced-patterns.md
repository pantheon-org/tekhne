# Advanced Higher-Order Function Patterns

> Reference material extracted from the `fp-higher-order-functions` skill. Load only when you need it.

## Advanced Higher-Order Function Patterns

### Currying and Partial Application

```javascript
// Currying: Transform f(a, b, c) into f(a)(b)(c)
function curry(fn) {
  return function curried(...args) {
    if (args.length >= fn.length) {
      return fn.apply(this, args);
    } else {
      return function(...nextArgs) {
        return curried.apply(this, args.concat(nextArgs));
      };
    }
  };
}

// Example usage
function add(a, b, c) {
  return a + b + c;
}

const curriedAdd = curry(add);
console.log(curriedAdd(1)(2)(3));  // 6
console.log(curriedAdd(1, 2)(3));  // 6
console.log(curriedAdd(1)(2, 3));  // 6

// Partial application
function partial(fn, ...fixedArgs) {
  return function(...remainingArgs) {
    return fn(...fixedArgs, ...remainingArgs);
  };
}

function greet(greeting, name) {
  return `${greeting}, ${name}!`;
}

const sayHello = partial(greet, 'Hello');
console.log(sayHello('Alice'));  // Hello, Alice!
console.log(sayHello('Bob'));    // Hello, Bob!

// Practical example: API helpers
function apiCall(method, endpoint, data) {
  return fetch(endpoint, {
    method,
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(data)
  });
}

const get = partial(apiCall, 'GET');
const post = partial(apiCall, 'POST');
const put = partial(apiCall, 'PUT');

// Usage
post('/api/users', { name: 'Alice' });
get('/api/users/1', null);
```

### Function Composition

```javascript
// Compose: right to left
const compose = (...fns) =>
  x => fns.reduceRight((acc, fn) => fn(acc), x);

// Pipe: left to right
const pipe = (...fns) =>
  x => fns.reduce((acc, fn) => fn(acc), x);

// Example functions
const addOne = x => x + 1;
const double = x => x * 2;
const square = x => x * x;

const composedCalc = compose(square, double, addOne);
console.log(composedCalc(3));  // square(double(addOne(3))) = 64

const pipedCalc = pipe(addOne, double, square);
console.log(pipedCalc(3));  // square(double(addOne(3))) = 64

// Practical example: Data processing pipeline
const users = [
  { name: 'alice', age: 25, active: true },
  { name: 'bob', age: 30, active: false },
  { name: 'charlie', age: 35, active: true }
];

const processUsers = pipe(
  users => users.filter(u => u.active),
  users => users.map(u => ({ ...u, name: u.name.toUpperCase() })),
  users => users.sort((a, b) => a.age - b.age)
);

console.log(processUsers(users));
```

### Haskell: Function Composition

```haskell
-- Function composition is built into Haskell with (.)
addOne :: Int -> Int
addOne x = x + 1

double :: Int -> Int
double x = x * 2

square :: Int -> Int
square x = x * x

-- Compose with (.) - right to left
composed :: Int -> Int
composed = square . double . addOne

-- Usage
result = composed 3  -- 64

-- Dollar operator ($) for function application
result2 = square $ double $ addOne 3  -- 64

-- Pipeline with (&) from Data.Function
import Data.Function ((&))

result3 = 3 & addOne & double & square  -- 64

-- Partial application (automatic currying)
add :: Int -> Int -> Int
add x y = x + y

addFive :: Int -> Int
addFive = add 5

result4 = addFive 10  -- 15

-- Map, filter, fold
numbers = [1, 2, 3, 4, 5]

doubled = map (*2) numbers
evens = filter even numbers
total = foldl (+) 0 numbers

-- Composing list operations
process :: [Int] -> Int
process = foldl (+) 0 . filter even . map (*2)

result5 = process [1, 2, 3, 4, 5]  -- 2*2 + 2*4 = 12
```
