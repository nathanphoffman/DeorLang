# Full Worked Example

## Source

```
(sqrt, floor, rand) in math

type Squarefeet(int n)
    n >= 0 and sqrt(n) == floor(sqrt(n))

struct Room
    Squarefeet area
    string name
    bool occupied

struct House
    string address
    List<Room> rooms

fn int total_area(List<Room> rooms)
    sum as 0
    for room in rooms
        area in room
        sum = sum + area
    return sum

fn List<Room> occupied_rooms(List<Room> rooms)
    List<Room> result = []
    for room in rooms
        occupied in room
        if occupied
            result append room
    return result

fn string random_room_name(List<Room> rooms)
    int idx = rand(0, len(rooms) - 1)
    name in rooms[idx]
    return name

fn main()
    area as 9
    name as "Kitchen"
    occupied as true
    kitchen as {area, name, occupied}

    area as 16
    name as "Office"
    occupied as false
    office as {area, name, occupied}

    area as 25
    name as "Bedroom"
    occupied as true
    bedroom as {area, name, occupied}

    room_list as [kitchen, office, bedroom]

    address as "12 Main St"
    rooms as room_list
    house as {address, rooms}

    rooms in house
    print(total_area(rooms))

    List<Room> occ = occupied_rooms(rooms)
    for room in occ
        name in room
        print(name)

    area as 25
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
    fn new(n: i32) -> Self {
        assert!(n >= 0 && (n as f64).sqrt().fract() == 0.0, "invalid Squarefeet: {}", n);
        Squarefeet(n)
    }
}

#[derive(Clone, PartialEq, Debug)]
struct Room {
    area: Squarefeet,
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
        sum += area.0;
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
    let idx: i32 = rand(0, (rooms.len() as i32) - 1);
    let name = rooms[idx as usize].name.clone();
    return name;
}

fn main() {
    let kitchen = Room { area: Squarefeet::new(9), name: "Kitchen".to_string(), occupied: true };
    let office = Room { area: Squarefeet::new(16), name: "Office".to_string(), occupied: false };
    let bedroom = Room { area: Squarefeet::new(25), name: "Bedroom".to_string(), occupied: true };

    let room_list = vec![kitchen.clone(), office.clone(), bedroom.clone()];
    let house = House { address: "12 Main St".to_string(), rooms: room_list };

    let rooms = house.rooms.clone();
    println!("{}", total_area(&rooms));

    let occ: Vec<Room> = occupied_rooms(&rooms);
    for room in &occ {
        let name = room.name.clone();
        println!("{}", name);
    }

    let bigger_kitchen = Room { area: Squarefeet::new(25), ..kitchen };

    let pick: String = random_room_name(&rooms);
    println!("{}", pick);
}
```

---

## Notable Conversion Decisions

- `House` contains an unsized `List<Room>`, so per the structs spec it would normally default to `struct*` (`Rc<House>`). In this particular `main`, `house` isn't shared across multiple owners, so the transpiler may reasonably keep it a plain value here — the heuristic is a default, not an absolute.
- `Room` contains a `String` field, so it can never be `Copy` — only `Clone`. Every place a `Room` is duplicated (`vec![kitchen.clone(), ...]`, `result.push(room.clone())`) needs an explicit `.clone()` in Rust, even though source never writes anything special.
- `rooms[idx]` requires an `as usize` cast, since Rust indexes with `usize` but `idx` is `i32` — the transpiler inserts this cast on every list-index operation.
- `print(...)` → `println!("{}", ...)`. Values that aren't already `Display` (like `Room`) would need `{:?}` and `#[derive(Debug)]` instead — already included above for safety.
