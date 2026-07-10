# Higher-Order Functions in Python and Elixir

> Reference material extracted from the `fp-higher-order-functions` skill. Load only when you need it.

## Python Map Examples

```python
from typing import List, Callable, Dict

# Basic map
numbers = [1, 2, 3, 4, 5]
doubled = list(map(lambda x: x * 2, numbers))
# [2, 4, 6, 8, 10]

# Map with named function
def square(x: int) -> int:
    return x ** 2

squared = list(map(square, numbers))
# [1, 4, 9, 16, 25]

# List comprehension (more Pythonic)
doubled_lc = [x * 2 for x in numbers]
squared_lc = [x ** 2 for x in numbers]

# Map with objects
users = [
    {'id': 1, 'first_name': 'John', 'last_name': 'Doe'},
    {'id': 2, 'first_name': 'Jane', 'last_name': 'Smith'},
    {'id': 3, 'first_name': 'Bob', 'last_name': 'Johnson'}
]

full_names = [f"{u['first_name']} {u['last_name']}" for u in users]

# Map to dataclass
from dataclasses import dataclass

@dataclass
class UserCard:
    id: int
    display_name: str
    initials: str

def create_user_card(user: Dict) -> UserCard:
    return UserCard(
        id=user['id'],
        display_name=f"{user['first_name']} {user['last_name']}",
        initials=f"{user['first_name'][0]}{user['last_name'][0]}"
    )

user_cards = list(map(create_user_card, users))

# Or with list comprehension
user_cards_lc = [create_user_card(u) for u in users]
```

## Elixir Map Patterns

```elixir
# Map with Enum.map
numbers = [1, 2, 3, 4, 5]
doubled = Enum.map(numbers, fn x -> x * 2 end)
# [2, 4, 6, 8, 10]

# Using capture syntax
doubled_capture = Enum.map(numbers, &(&1 * 2))

# Map with structs
defmodule User do
  defstruct [:id, :first_name, :last_name]
end

users = [
  %User{id: 1, first_name: "John", last_name: "Doe"},
  %User{id: 2, first_name: "Jane", last_name: "Smith"},
  %User{id: 3, first_name: "Bob", last_name: "Johnson"}
]

full_names = Enum.map(users, fn user ->
  "#{user.first_name} #{user.last_name}"
end)

# Map to different structure
user_cards = Enum.map(users, fn user ->
  %{
    id: user.id,
    display_name: "#{user.first_name} #{user.last_name}",
    initials: "#{String.first(user.first_name)}#{String.first(user.last_name)}"
  }
end)

# Pipe operator for chaining
result = [1, 2, 3, 4, 5]
  |> Enum.map(&(&1 * 2))
  |> Enum.map(&(&1 + 1))
  |> Enum.map(&Integer.to_string/1)
# ["3", "5", "7", "9", "11"]
```

## Python Filter Examples

```python
from typing import List, Callable

numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

# Filter with lambda
evens = list(filter(lambda x: x % 2 == 0, numbers))
# [2, 4, 6, 8, 10]

# List comprehension (more Pythonic)
evens_lc = [x for x in numbers if x % 2 == 0]
greater_than_five = [x for x in numbers if x > 5]

# Filter objects
users = [
    {'id': 1, 'name': 'Alice', 'age': 25, 'active': True},
    {'id': 2, 'name': 'Bob', 'age': 30, 'active': False},
    {'id': 3, 'name': 'Charlie', 'age': 35, 'active': True},
    {'id': 4, 'name': 'Diana', 'age': 28, 'active': True}
]

active_users = [u for u in users if u['active']]
young_users = [u for u in users if u['age'] < 30]

# Complex predicates
active_young_users = [
    u for u in users
    if u['active'] and u['age'] < 30
]

# Combining filter and map
active_user_names = [
    u['name'] for u in users
    if u['active']
]

# Named predicate functions
def is_active(user: dict) -> bool:
    return user['active']

def is_young(user: dict) -> bool:
    return user['age'] < 30

active_users_func = list(filter(is_active, users))

# Remove None values
values = [1, None, 2, None, 3, None, 4]
defined = [v for v in values if v is not None]
```

## Elixir Filter Patterns

```elixir
numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

# Basic filter
evens = Enum.filter(numbers, fn x -> rem(x, 2) == 0 end)
# [2, 4, 6, 8, 10]

# Capture syntax
evens_capture = Enum.filter(numbers, &(rem(&1, 2) == 0))

# Filter structs
defmodule User do
  defstruct [:id, :name, :age, :active]
end

users = [
  %User{id: 1, name: "Alice", age: 25, active: true},
  %User{id: 2, name: "Bob", age: 30, active: false},
  %User{id: 3, name: "Charlie", age: 35, active: true},
  %User{id: 4, name: "Diana", age: 28, active: true}
]

active_users = Enum.filter(users, fn user -> user.active end)
young_users = Enum.filter(users, fn user -> user.age < 30 end)

# Complex predicates
active_young_users = Enum.filter(users, fn user ->
  user.active and user.age < 30
end)

# Combining filter and map
active_user_names = users
  |> Enum.filter(&(&1.active))
  |> Enum.map(&(&1.name))

# Reject (opposite of filter)
inactive_users = Enum.reject(users, &(&1.active))

# Remove nils
values = [1, nil, 2, nil, 3, nil, 4]
defined = Enum.filter(values, &(!is_nil(&1)))
# Or more concisely
defined = Enum.reject(values, &is_nil/1)
```

## Python Reduce Examples

```python
from functools import reduce
from typing import List, Dict, Any

numbers = [1, 2, 3, 4, 5]

# Sum (but use built-in sum())
total = reduce(lambda acc, x: acc + x, numbers, 0)
# 15

# Better: use built-in
total_builtin = sum(numbers)

# Product
product = reduce(lambda acc, x: acc * x, numbers, 1)
# 120

# Maximum (but use max())
maximum = reduce(lambda acc, x: max(acc, x), numbers)

# Building dictionaries
users = [
    {'id': 1, 'name': 'Alice'},
    {'id': 2, 'name': 'Bob'},
    {'id': 3, 'name': 'Charlie'}
]

# As dictionary
user_map = {user['id']: user for user in users}

# Or with reduce
user_map_reduce = reduce(
    lambda acc, user: {**acc, user['id']: user},
    users,
    {}
)

# Grouping
transactions = [
    {'category': 'food', 'amount': 50},
    {'category': 'transport', 'amount': 30},
    {'category': 'food', 'amount': 25},
    {'category': 'entertainment', 'amount': 100},
    {'category': 'transport', 'amount': 15}
]

def group_by_category(acc: Dict, transaction: Dict) -> Dict:
    category = transaction['category']
    if category not in acc:
        acc[category] = []
    acc[category].append(transaction)
    return acc

by_category = reduce(group_by_category, transactions, {})

# Better: use itertools.groupby or collections.defaultdict
from collections import defaultdict

by_category_dd = defaultdict(list)
for t in transactions:
    by_category_dd[t['category']].append(t)

# Count occurrences
fruits = ['apple', 'banana', 'apple', 'orange', 'banana', 'apple']

# With reduce
counts = reduce(
    lambda acc, fruit: {**acc, fruit: acc.get(fruit, 0) + 1},
    fruits,
    {}
)

# Better: use Counter
from collections import Counter
counts_counter = Counter(fruits)

# Flatten lists
nested = [[1, 2], [3, 4], [5, 6]]

# With reduce
flattened = reduce(lambda acc, lst: acc + lst, nested, [])

# Better: use itertools.chain or list comprehension
from itertools import chain
flattened_chain = list(chain.from_iterable(nested))
flattened_lc = [item for sublist in nested for item in sublist]
```

## Elixir Reduce Patterns

```elixir
numbers = [1, 2, 3, 4, 5]

# Sum
sum = Enum.reduce(numbers, 0, fn x, acc -> acc + x end)
# 15

# Or use Enum.sum
sum_builtin = Enum.sum(numbers)

# Product
product = Enum.reduce(numbers, 1, fn x, acc -> acc * x end)
# 120

# Maximum
max = Enum.reduce(numbers, fn x, acc -> max(x, acc) end)

# Or use Enum.max
max_builtin = Enum.max(numbers)

# Building maps from lists
users = [
  %{id: 1, name: "Alice"},
  %{id: 2, name: "Bob"},
  %{id: 3, name: "Charlie"}
]

user_map = Enum.reduce(users, %{}, fn user, acc ->
  Map.put(acc, user.id, user)
end)

# Or use Map.new
user_map_simple = Map.new(users, fn user -> {user.id, user} end)

# Grouping
transactions = [
  %{category: "food", amount: 50},
  %{category: "transport", amount: 30},
  %{category: "food", amount: 25},
  %{category: "entertainment", amount: 100},
  %{category: "transport", amount: 15}
]

by_category = Enum.reduce(transactions, %{}, fn transaction, acc ->
  Map.update(acc, transaction.category, [transaction], fn existing ->
    [transaction | existing]
  end)
end)

# Or use Enum.group_by
by_category_simple = Enum.group_by(transactions, & &1.category)

# Count occurrences
fruits = ["apple", "banana", "apple", "orange", "banana", "apple"]

counts = Enum.reduce(fruits, %{}, fn fruit, acc ->
  Map.update(acc, fruit, 1, &(&1 + 1))
end)

# Or use Enum.frequencies
counts_simple = Enum.frequencies(fruits)

# Flatten lists
nested = [[1, 2], [3, 4], [5, 6]]

flattened = Enum.reduce(nested, [], fn list, acc ->
  acc ++ list
end)

# Or use List.flatten
flattened_simple = List.flatten(nested)

# Function composition with reduce
compose = fn fns ->
  Enum.reduce(fns, fn f, g ->
    fn x -> f.(g.(x)) end
  end)
end

add_one = fn x -> x + 1 end
double = fn x -> x * 2 end
square = fn x -> x * x end

composed = compose.([square, double, add_one])
IO.inspect composed.(3)  # 64
```
