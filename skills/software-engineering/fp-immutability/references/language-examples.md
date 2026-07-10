# Immutability in Python, Elixir, and Haskell

> Reference material extracted from the `fp-immutability` skill. Load only when you need it.

## Python: Immutability with Dataclasses and NamedTuples

```python
from dataclasses import dataclass, replace
from typing import List, NamedTuple, FrozenSet
from collections import namedtuple

# Using frozen dataclasses (Python 3.7+)
@dataclass(frozen=True)
class Point:
    x: float
    y: float

    def move(self, dx: float, dy: float) -> 'Point':
        """Returns new Point, doesn't modify existing"""
        return Point(self.x + dx, self.y + dy)

    def distance_from_origin(self) -> float:
        return (self.x ** 2 + self.y ** 2) ** 0.5

# Usage
p1 = Point(3, 4)
p2 = p1.move(1, 1)  # Creates new Point(4, 5)
# p1.x = 10  # Error! Point is frozen

# Using NamedTuple
class Address(NamedTuple):
    street: str
    city: str
    zip_code: str
    country: str = 'USA'

    def update_street(self, new_street: str) -> 'Address':
        return self._replace(street=new_street)

# Immutable collections
address = Address('123 Main St', 'Boston', '02101')
new_address = address.update_street('456 Oak Ave')

# Using dataclass replace
@dataclass(frozen=True)
class User:
    id: int
    name: str
    email: str
    active: bool = True

    def deactivate(self) -> 'User':
        return replace(self, active=False)

    def update_email(self, new_email: str) -> 'User':
        return replace(self, email=new_email)

# Immutable list operations
original_list = [1, 2, 3, 4, 5]

# Add element (returns new list)
def add_item(lst: List, item) -> List:
    return [*lst, item]

# Remove element
def remove_item(lst: List, item) -> List:
    return [x for x in lst if x != item]

# Update element
def update_at_index(lst: List, index: int, value) -> List:
    return [*lst[:index], value, *lst[index + 1:]]

# Working with tuples (immutable by default)
coords = (10, 20, 30)
# coords[0] = 15  # Error! Tuples are immutable

# Using frozenset for immutable sets
immutable_set: FrozenSet[int] = frozenset([1, 2, 3, 4])
new_set = immutable_set | {5, 6}  # Creates new frozenset
```

## Python: Immutable State Management

```python
from dataclasses import dataclass, replace
from typing import List, Dict, Optional
from datetime import datetime

@dataclass(frozen=True)
class CartItem:
    product_id: str
    name: str
    price: float
    quantity: int

    def update_quantity(self, new_quantity: int) -> 'CartItem':
        return replace(self, quantity=new_quantity)

@dataclass(frozen=True)
class ShoppingCart:
    items: tuple[CartItem, ...]  # Tuple is immutable
    created_at: datetime
    discount_code: Optional[str] = None

    def add_item(self, item: CartItem) -> 'ShoppingCart':
        # Check if item exists
        existing_index = next(
            (i for i, cart_item in enumerate(self.items)
             if cart_item.product_id == item.product_id),
            None
        )

        if existing_index is not None:
            # Update quantity of existing item
            existing_item = self.items[existing_index]
            updated_item = existing_item.update_quantity(
                existing_item.quantity + item.quantity
            )
            new_items = (
                *self.items[:existing_index],
                updated_item,
                *self.items[existing_index + 1:]
            )
        else:
            # Add new item
            new_items = (*self.items, item)

        return replace(self, items=new_items)

    def remove_item(self, product_id: str) -> 'ShoppingCart':
        new_items = tuple(
            item for item in self.items
            if item.product_id != product_id
        )
        return replace(self, items=new_items)

    def update_quantity(self, product_id: str, quantity: int) -> 'ShoppingCart':
        new_items = tuple(
            item.update_quantity(quantity) if item.product_id == product_id else item
            for item in self.items
        )
        return replace(self, items=new_items)

    def apply_discount(self, code: str) -> 'ShoppingCart':
        return replace(self, discount_code=code)

    def total(self) -> float:
        return sum(item.price * item.quantity for item in self.items)

# Usage
cart = ShoppingCart(items=(), created_at=datetime.now())
cart = cart.add_item(CartItem('p1', 'Widget', 10.0, 2))
cart = cart.add_item(CartItem('p2', 'Gadget', 15.0, 1))
cart = cart.update_quantity('p1', 3)
print(f"Total: ${cart.total()}")
```

## Elixir: Built-in Immutability

```elixir
# In Elixir, ALL data is immutable by default

defmodule UserManager do
  defstruct [:id, :name, :email, :active, :created_at]

  # Creating a new user
  def new(id, name, email) do
    %__MODULE__{
      id: id,
      name: name,
      email: email,
      active: true,
      created_at: DateTime.utc_now()
    }
  end

  # Updating returns new struct
  def update_email(user, new_email) do
    %{user | email: new_email}
  end

  def deactivate(user) do
    %{user | active: false}
  end

  def update_name(user, new_name) do
    %{user | name: new_name}
  end
end

# List operations (all immutable)
defmodule ListOps do
  # Add to list
  def add_item(list, item) do
    [item | list]  # Prepend (efficient)
    # or
    list ++ [item]  # Append (less efficient)
  end

  # Remove from list
  def remove_item(list, item) do
    List.delete(list, item)
  end

  # Update at index
  def update_at_index(list, index, value) do
    List.replace_at(list, index, value)
  end

  # Transform all elements
  def double_all(list) do
    Enum.map(list, fn x -> x * 2 end)
  end
end

# Map operations (all immutable)
defmodule MapOps do
  # Add/update key
  def put_value(map, key, value) do
    Map.put(map, key, value)
  end

  # Remove key
  def remove_key(map, key) do
    Map.delete(map, key)
  end

  # Update existing key
  def update_value(map, key, fun) do
    Map.update!(map, key, fun)
  end

  # Merge maps
  def merge_maps(map1, map2) do
    Map.merge(map1, map2)
  end
end

# Usage examples
user = UserManager.new(1, "Alice", "alice@example.com")
updated_user = UserManager.update_email(user, "alice@newdomain.com")
# `user` is unchanged
# `updated_user` is a new struct

# List transformations
numbers = [1, 2, 3, 4, 5]
doubled = Enum.map(numbers, &(&1 * 2))
filtered = Enum.filter(numbers, &(&1 > 2))
# `numbers` remains [1, 2, 3, 4, 5]

# Pipe operator leverages immutability
result = [1, 2, 3, 4, 5]
  |> Enum.map(&(&1 * 2))
  |> Enum.filter(&(&1 > 5))
  |> Enum.sum()
```

## Elixir: Persistent Data Structures

```elixir
defmodule ShoppingCart do
  defstruct items: [], created_at: nil, discount_code: nil

  defmodule Item do
    defstruct [:product_id, :name, :price, :quantity]
  end

  def new do
    %__MODULE__{
      items: [],
      created_at: DateTime.utc_now()
    }
  end

  def add_item(cart, item) do
    case find_item_index(cart.items, item.product_id) do
      nil ->
        # Item doesn't exist, add it
        %{cart | items: [item | cart.items]}

      index ->
        # Item exists, update quantity
        update_item_quantity(cart, index, fn qty ->
          qty + item.quantity
        end)
    end
  end

  def remove_item(cart, product_id) do
    new_items = Enum.reject(cart.items, fn item ->
      item.product_id == product_id
    end)
    %{cart | items: new_items}
  end

  def update_quantity(cart, product_id, new_quantity) do
    new_items = Enum.map(cart.items, fn item ->
      if item.product_id == product_id do
        %{item | quantity: new_quantity}
      else
        item
      end
    end)
    %{cart | items: new_items}
  end

  def apply_discount(cart, code) do
    %{cart | discount_code: code}
  end

  def total(cart) do
    Enum.reduce(cart.items, 0, fn item, acc ->
      acc + (item.price * item.quantity)
    end)
  end

  defp find_item_index(items, product_id) do
    Enum.find_index(items, fn item ->
      item.product_id == product_id
    end)
  end

  defp update_item_quantity(cart, index, fun) do
    new_items = List.update_at(cart.items, index, fn item ->
      %{item | quantity: fun.(item.quantity)}
    end)
    %{cart | items: new_items}
  end
end

# Usage demonstrates immutability
cart1 = ShoppingCart.new()
cart2 = ShoppingCart.add_item(cart1, %ShoppingCart.Item{
  product_id: "p1",
  name: "Widget",
  price: 10.0,
  quantity: 2
})
cart3 = ShoppingCart.add_item(cart2, %ShoppingCart.Item{
  product_id: "p2",
  name: "Gadget",
  price: 15.0,
  quantity: 1
})

# cart1, cart2, cart3 are all different, immutable values
IO.inspect(ShoppingCart.total(cart1))  # 0
IO.inspect(ShoppingCart.total(cart2))  # 20.0
IO.inspect(ShoppingCart.total(cart3))  # 35.0
```

## Haskell: Pure Immutability with Type Safety

```haskell
-- All data is immutable in Haskell

-- Simple immutable data types
data Point = Point
  { pointX :: Double
  , pointY :: Double
  } deriving (Show, Eq)

movePoint :: Point -> Double -> Double -> Point
movePoint (Point x y) dx dy = Point (x + dx) (y + dy)

-- Record update syntax
updateX :: Point -> Double -> Point
updateX p newX = p { pointX = newX }

-- User data type
data User = User
  { userId :: Int
  , userName :: String
  , userEmail :: String
  , userActive :: Bool
  } deriving (Show, Eq)

updateEmail :: User -> String -> User
updateEmail user newEmail = user { userEmail = newEmail }

deactivateUser :: User -> User
deactivateUser user = user { userActive = False }

-- Immutable collections
-- Lists are immutable and persistent
addToList :: a -> [a] -> [a]
addToList x xs = x : xs

removeFromList :: Eq a => a -> [a] -> [a]
removeFromList x xs = filter (/= x) xs

updateAtIndex :: Int -> a -> [a] -> [a]
updateAtIndex _ _ [] = []
updateAtIndex 0 y (_:xs) = y : xs
updateAtIndex n y (x:xs) = x : updateAtIndex (n-1) y xs

-- Shopping cart example
data CartItem = CartItem
  { itemProductId :: String
  , itemName :: String
  , itemPrice :: Double
  , itemQuantity :: Int
  } deriving (Show, Eq)

data ShoppingCart = ShoppingCart
  { cartItems :: [CartItem]
  , cartCreatedAt :: String  -- Simplified for example
  , cartDiscountCode :: Maybe String
  } deriving (Show, Eq)

newCart :: String -> ShoppingCart
newCart timestamp = ShoppingCart
  { cartItems = []
  , cartCreatedAt = timestamp
  , cartDiscountCode = Nothing
  }

addItem :: CartItem -> ShoppingCart -> ShoppingCart
addItem item cart =
  case findItem (itemProductId item) (cartItems cart) of
    Nothing ->
      cart { cartItems = item : cartItems cart }
    Just existingItem ->
      let updatedItem = existingItem
            { itemQuantity = itemQuantity existingItem + itemQuantity item }
          updatedItems = updateItem (itemProductId item) updatedItem (cartItems cart)
      in cart { cartItems = updatedItems }

removeItem :: String -> ShoppingCart -> ShoppingCart
removeItem productId cart =
  cart { cartItems = filter (\i -> itemProductId i /= productId) (cartItems cart) }

updateQuantity :: String -> Int -> ShoppingCart -> ShoppingCart
updateQuantity productId newQty cart =
  let updatedItems = map (\i ->
        if itemProductId i == productId
        then i { itemQuantity = newQty }
        else i) (cartItems cart)
  in cart { cartItems = updatedItems }

applyDiscount :: String -> ShoppingCart -> ShoppingCart
applyDiscount code cart = cart { cartDiscountCode = Just code }

calculateTotal :: ShoppingCart -> Double
calculateTotal cart = sum $ map itemTotal (cartItems cart)
  where
    itemTotal item = itemPrice item * fromIntegral (itemQuantity item)

-- Helper functions
findItem :: String -> [CartItem] -> Maybe CartItem
findItem productId = find (\i -> itemProductId i == productId)

updateItem :: String -> CartItem -> [CartItem] -> [CartItem]
updateItem productId newItem = map (\i ->
  if itemProductId i == productId then newItem else i)
```
