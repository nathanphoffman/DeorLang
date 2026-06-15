# Full Worked Example

## Source

```
(sqrt, floor, rand) in math

type Squarefeet(int val)
    float flt = to_float(val)
    NonNegFloat root_nf = sqrt(flt)
    float root_f = root_nf else 0.0
    int root = floor(root_f)
    root * root is val

struct Room
    Squarefeet area
    string name
    bool occupied

struct House
    string address
    list<Room> rooms

fn int total_area(list<Room> rooms)
    sum as 0
    for room in rooms
        area in room
        int sqm = area else 0
        sum = sum + sqm
    return sum

fn list<Room> occupied_rooms(list<Room> rooms)
    list<Room> result = []
    for room in rooms
        occupied in room
        if occupied
            result insert room
    return result

fn string random_room_name(list<Room> rooms)
    int count = len(rooms)
    int last = count - 1
    start as 0
    int idx = rand(start, last)
    name in rooms[idx]
    return name

fn main()
    area as 9
    name as "Kitchen"
    occupied as true
    kitchen as (area, name, occupied)

    area = 16
    name = "Office"
    occupied = false
    office as (area, name, occupied)

    area = 25
    name = "Bedroom"
    occupied = true
    bedroom as (area, name, occupied)

    rooms as [kitchen, office, bedroom]

    address as "12 Main St"
    house as (address, rooms)

    print(total_area(rooms))

    list<Room> occ = occupied_rooms(rooms)
    for room in occ
        name in room
        print(name)

    area = 25
    biggerKitchen as kitchen with area

    string pick = random_room_name(rooms)
    print(pick)
```

---

## Rust Translation

```rust
#[derive(Clone, Copy, PartialEq, Debug)]
struct Squarefeet(i32);

impl Squarefeet {
    fn new(val: i32) -> Option<Self> {
        let flt: f64 = val as f64;
        let root_nf: Option<NonNegFloat> = NonNegFloat::new(flt);
        let root_f: f64 = root_nf.map(|v| v.0).unwrap_or(0.0);
        let root: i32 = root_f.floor() as i32;
        if root * root == val {
            Some(Squarefeet(val))
        } else {
            None
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
struct Room {
    area: Option<Squarefeet>,
    name: String,
    occupied: bool,
}

#[derive(Clone, PartialEq, Debug)]
struct House {
    address: String,
    rooms: Vec<Room>,
}

fn total_area(rooms: &Vec<Room>) -> i32 {
    let mut sum = 0;
    for room in rooms {
        let area = room.area;
        let sqm: i32 = area.map(|v| v.0).unwrap_or(0);
        sum += sqm;
    }
    return sum;
}

fn occupied_rooms(rooms: &Vec<Room>) -> Vec<Room> {
    let mut result: Vec<Room> = Vec::new();
    for room in rooms {
        let occupied = room.occupied;
        if occupied {
            result.push(room.clone());
        }
    }
    return result;
}

fn random_room_name(rooms: &Vec<Room>) -> String {
    let count: i32 = rooms.len() as i32;
    let last: i32 = count - 1;
    let start: i32 = 0;
    let idx: i32 = rand::thread_rng().gen_range(start..=last);
    let name = rooms[idx as usize].name.clone();
    return name;
}

fn main() {
    let mut area: Option<Squarefeet> = Squarefeet::new(9);
    let mut name: String = "Kitchen".to_string();
    let mut occupied: bool = true;
    let kitchen = Room { area, name: name.clone(), occupied };

    area = Squarefeet::new(16);
    name = "Office".to_string();
    occupied = false;
    let office = Room { area, name: name.clone(), occupied };

    area = Squarefeet::new(25);
    name = "Bedroom".to_string();
    occupied = true;
    let bedroom = Room { area, name: name.clone(), occupied };

    let rooms = vec![kitchen.clone(), office.clone(), bedroom.clone()];
    let house = House { address: "12 Main St".to_string(), rooms: rooms.clone() };
    let _ = house; // house constructed; rooms used directly below

    println!("{}", total_area(&rooms));

    let occ: Vec<Room> = occupied_rooms(&rooms);
    for room in &occ {
        let name = room.name.clone();
        println!("{}", name);
    }

    area = Squarefeet::new(25);
    let bigger_kitchen = Room { area, ..kitchen };

    let pick: String = random_room_name(&rooms);
    println!("{}", pick);
}
```

---

## Notable Conversion Decisions

- `House` contains an unsized `list<Room>`, so per the structs spec it would normally default to `struct*` (`Rc<House>`). In this particular `main`, `house` isn't shared across multiple owners, so the transpiler may reasonably keep it a plain value here — the heuristic is a default, not an absolute.
- `area`, `name`, and `occupied` are declared once with `as` then reassigned with `=` to build multiple rooms. The transpiler emits them as `let mut` because they are reassigned after first declaration. `name` is cloned on each struct construction because `String` is not `Copy`.
- `Room` contains a `String` field, so it can never be `Copy` — only `Clone`. Every place a `Room` is duplicated (`vec![kitchen.clone(), ...]`, `result.push(room.clone())`) needs an explicit `.clone()` in Rust, even though source never writes anything special.
- `Squarefeet` is a validator type, so `room.area` is `Option<Squarefeet>`. `total_area` uses `area else 0` (→ `.map(|v| v.0).unwrap_or(0)`) to safely extract the inner `i32` before adding to the sum.
- `rooms[idx]` requires an `as usize` cast, since Rust indexes with `usize` but `idx` is `i32` — the transpiler inserts this cast on every list-index operation.
- `print(...)` → `println!("{}", ...)`. Values that aren't already `Display` (like `Room`) would need `{:?}` and `#[derive(Debug)]` instead — already included above for safety.
