# Basic Type Guards

## Overview
Type guards are TypeScript constructs that allow narrowing types within conditional blocks using runtime checks.

## typeof Type Guards

```typescript
function processValue(value: string | number) {
  if (typeof value === "string") {
    // value is narrowed to string
    return value.toUpperCase();
  }
  // value is narrowed to number
  return value.toFixed(2);
}
```

## instanceof Type Guards

```typescript
class Dog {
  bark() { console.log("Woof!"); }
}

class Cat {
  meow() { console.log("Meow!"); }
}

function makeSound(animal: Dog | Cat) {
  if (animal instanceof Dog) {
    animal.bark();
  } else {
    animal.meow();
  }
}
```

## in Operator Type Guards

```typescript
interface Fish {
  swim: () => void;
}

interface Bird {
  fly: () => void;
}

function move(animal: Fish | Bird) {
  if ("swim" in animal) {
    animal.swim();
  } else {
    animal.fly();
  }
}
```

## Truthiness Narrowing

```typescript
function printLength(str: string | null) {
  if (str) {
    // str is narrowed to string
    console.log(str.length);
  } else {
    // str is null
    console.log("No string provided");
  }
}
```

## Equality Narrowing

```typescript
function example(x: string | number, y: string | boolean) {
  if (x === y) {
    // x and y are both narrowed to string
    x.toUpperCase();
    y.toUpperCase();
  }
}
```

## Best Practices
- Use typeof for primitives (string, number, boolean, symbol)
- Use instanceof for class instances
- Use in operator for discriminating object shapes
- Combine guards for complex narrowing
- Consider custom type predicates for reusable logic

## Common Pitfalls
- typeof null returns "object" (use === null instead)
- typeof array returns "object" (use Array.isArray instead)
- instanceof doesn't work across realm boundaries
