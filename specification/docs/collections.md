# Collections
List operations assume a list shape has already been declared — see [Shapes](docs/shapes.md) for how to declare one and use it in function signatures and struct fields.

---
## Empty Lisy
To define an empty list use the ```empty``` keyword.  Comparisons can also be done on the empty keyword, [] is never valid for setting lists to empty ([, ] are only used for definint list content, as seen below)

```
listString list_names as empty
if list_names is empty
    print("list is empty")

if list_names is not empty
    print("list is not empty")

```

---
## Index Read
Elements are read by index using `at`. Zero-indexed, matching Rust's behavior.

```
intList scores = [10, 20, 30, 40]
int first = scores at 0    # 10
int last = scores at 3     # 40
```

```rust
let scores: Vec<i32> = vec![10, 20, 30, 40];
let first: i32 = scores[0];
let last: i32 = scores[3];
```

Dynamic computed indices are fine:

```
int idx = 2
int mid = scores at idx    # 30
```

Out-of-bounds access is a runtime panic. The transpiler inserts `as usize` casts on all index operations automatically.

---
## Index Write
Elements are replaced by index using `at` on the left side of an assignment. The right-hand side must be a named variable of the list's element type.

```
rooms at idx = new_room
scores at idx = updated_score
```

```rust
rooms[idx as usize] = new_room;
scores[idx as usize] = updated_score;
```

Out-of-bounds assignment is a runtime panic.

---
## Append
`at end` appends a new element to the end of the list. `end` is a reserved keyword meaning "the position after the last element" — it is only valid in this position.

```
result at end = item
rooms at end = new_room
```

```rust
result.push(item);
rooms.push(new_room.clone());
```

---
## Remove
`remove at` removes the element at a given index, shifting subsequent elements left.

```
result remove at 2
```

```rust
result.remove(2);
```

For removing multiple elements, remove from highest index to lowest to avoid index-shifting errors:

```
result remove at 5
result remove at 2
result remove at 1
```

```rust
result.remove(5);
result.remove(2);
result.remove(1);
```

---
## Slice
`in range(start, end)` extracts a contiguous sublist. Returns a new list of the same shape type. `end` is exclusive — the element at `end` is not included, this follows rust behavior.

```
roomList first_ten = rooms in range(0, 10)
```

```rust
let first_ten: Vec<Room> = rooms[0..10].to_vec();
```

`end` is a reserved keyword meaning "the length of this list" when used as the second argument to `range()` in a slice:

```
int mid = 5
roomList tail = rooms in range(mid, end)
```

```rust
let tail: Vec<Room> = rooms[5..].to_vec();
```

Both forms are valid:

```
start as 0
roomList head = rooms in range(start, mid)     # from 0 to mid
roomList tail = rooms in range(mid, end)       # from mid to end of list
```

The `range()` arguments follow the same rules as everywhere else — built-in function, so literals are valid directly.

---
## No Membership Test
Deor has no built-in membership operator. To check whether an element is in a list, write an explicit loop or define a reusable helper function:

```
shape matchFunc = func of Room to bool

fn bool any_match(roomList items, matchFunc predicate)
    for item in items
        if predicate(item)
            return true
        else
            return false
```

---
## Updating a Struct Inside a List
Deor does not allow in-place field mutation — `rooms at 0` followed by field assignment is a transpiler error. Struct values inside a list are replaced, not mutated. Extract the struct, build an updated copy with `with`, write it back.

```
# 1. Read the existing struct
Room old_room = rooms at idx

# 2. Build the updated version
Squarefeet area = 25
Room new_room = old_room with (area)

# 3. Write back
rooms at idx = new_room
```

Note how in rust the new_area variable does not need to match by name, but it does in Deor (it is how it is bound)
```rust
let old_room: Room = rooms[idx as usize].clone();
let new_area: Option<Squarefeet> = Squarefeet::new(25);
let new_room: Room = Room { area: new_area, ..old_room };
rooms[idx as usize] = new_room;
```
