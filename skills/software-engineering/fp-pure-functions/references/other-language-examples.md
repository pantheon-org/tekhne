# Pure Functions in Elixir and Haskell

> Reference material extracted from the `fp-pure-functions` skill. Load only when you need it.

## Pure Functions in Elixir

Elixir's immutable data structures make it naturally suited for pure functions:

```elixir
# All functions in Elixir are pure by default
defmodule Calculator do
  # Pure: No side effects, deterministic
  def add(a, b), do: a + b

  def multiply(a, b), do: a * b

  # Pure: Returns new list without mutation
  def double_list(list) do
    Enum.map(list, fn x -> x * 2 end)
  end

  # Pure: Transforms data without side effects
  def calculate_total(items) do
    items
    |> Enum.map(& &1.price * &1.quantity)
    |> Enum.sum()
  end
end

defmodule UserTransformer do
  # Pure: Returns new struct
  def update_email(user, new_email) do
    %{user | email: new_email}
  end

  # Pure: Filters and transforms
  def get_active_users(users) do
    users
    |> Enum.filter(& &1.active)
    |> Enum.map(&format_user/1)
  end

  defp format_user(user) do
    %{
      id: user.id,
      name: "#{user.first_name} #{user.last_name}",
      email: user.email
    }
  end
end

# Usage demonstrates immutability
original_list = [1, 2, 3]
doubled = Calculator.double_list(original_list)
# original_list is still [1, 2, 3]
# doubled is [2, 4, 6]
```

## Pure Functions in Haskell

Haskell enforces purity at the type system level:

```haskell
-- Pure functions (the default in Haskell)
add :: Int -> Int -> Int
add x y = x + y

multiply :: Int -> Int -> Int
multiply x y = x * y

-- Pure: List transformation
doubleList :: [Int] -> [Int]
doubleList xs = map (*2) xs

-- Pure: Complex transformation
calculateDiscount :: Double -> Double -> Double
calculateDiscount price discountPercent =
  price * (1 - discountPercent / 100)

-- Pure: Working with custom types
data User = User
  { userId :: Int
  , userName :: String
  , userEmail :: String
  , userActive :: Bool
  }

updateEmail :: User -> String -> User
updateEmail user newEmail = user { userEmail = newEmail }

getActiveUsers :: [User] -> [User]
getActiveUsers = filter userActive

formatUser :: User -> String
formatUser user = userName user ++ " <" ++ userEmail user ++ ">"

-- Impure operations are explicitly marked with IO
-- This is NOT a pure function
readConfig :: IO String
readConfig = readFile "config.json"

-- Pure function that processes the config
parseConfig :: String -> Maybe Config
parseConfig = ... -- parsing logic
```
