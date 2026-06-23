# Full Worked Example

## Source

```
shape roomList = list of Room

type Squarefeet(int val)
    val > 0

struct Room
    Squarefeet area
    string name
    bool occupied

struct House
    string address
    roomList rooms

fn int total_area(roomList rooms)
    sum as 0
    for room in rooms
        area in room
        int sqm as 0
        if area is not bad
            sqm = (avow area)
        sum = sum + sqm
    return sum

fn roomList occupied_rooms(roomList rooms)
    roomList result = empty
    for room in rooms
        occupied in room
        if occupied
            result at end = room
    return result

# random is a shim — copy from shims.md
fn string random_room_name(roomList rooms)
    int count = len(rooms)
    int last = count - 1
    start as 0
    int idx = random(start, last)
    Room picked = rooms at idx
    (name) in picked
    return name

fn main()
    Squarefeet area = 9
    name as "Kitchen"
    occupied as true
    Room kitchen = (area, name, occupied)

    area = 16
    name = "Office"
    occupied = false
    Room office = (area, name, occupied)

    area = 25
    name = "Bedroom"
    occupied = true
    Room bedroom = (area, name, occupied)

    rooms as [kitchen, office, bedroom]

    address as "12 Main St"
    House house = (address, rooms)

    int area_sum = total_area(rooms)
    print(area_sum)

    roomList occ = occupied_rooms(rooms)
    for room in occ
        name in room
        print(name)

    area = 25
    bigger_kitchen as kitchen with (area)

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
        if val > 0 {
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
        let mut sqm: i32 = 0;
        if area != None {
            sqm = area.unwrap().0;
        }
        sum = sum + sqm;
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

    let rooms: Vec<Room> = vec![kitchen.clone(), office.clone(), bedroom.clone()];
    let address = "12 Main St".to_string();
    let house = House { address, rooms: rooms.clone() };
    let _ = house;

    let area_sum: i32 = total_area(&rooms);
    println!("{}", area_sum);

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

- `shape roomList = list of Room` compiles to `type RoomList = Vec<Room>` — the shape name is a type alias used throughout the generated Rust.
- `House` contains a `roomList rooms` field (`Vec<Room>`). Structs in Deor are plain value types — the transpiler emits a standard Rust struct with `#[derive(Clone, PartialEq, Debug)]`.
- `area`, `name`, and `occupied` are declared once with `as` then reassigned with `=` to build multiple rooms. The transpiler emits them as `let mut` because they are reassigned after first declaration.
- `Room` contains a `String` field so it can never be `Copy` — only `Clone`. Every place a `Room` is duplicated needs an explicit `.clone()` in Rust, even though source never writes it.
- `Squarefeet` is a validator type, so `room.area` is `Option<Squarefeet>`. `total_area` checks `if area is not bad` and uses `(avow area)` to safely extract the inner `i32` with a default of `0`.
- `rooms[idx]` requires an `as usize` cast — the transpiler inserts this on every list-index operation.
- `print(...)` → `println!("{}", ...)`.
