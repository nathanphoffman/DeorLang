# Threads and Channels

Deor supports parallel execution via OS threads and message-passing channels. This maps directly to `std::thread` and `std::sync::mpsc` in Rust — no external crates required.

Threads are real OS threads. You can spawn more threads than CPU cores and the OS scheduler will time-slice between them. For CPU-bound work the sweet spot is near core count; for I/O-bound work (waiting on network, disk) you can go much higher since most threads spend their time blocked.

---

## Channel Shapes

Channels are declared using `sender of T` and `receiver of T` shapes. These must be declared before use, like any other shape.

```
shape intSender = sender of int
shape intReceiver = receiver of int
```

`sender of T` and `receiver of T` are the only valid forms. The element type `T` must be a known struct, primitive, or shape name.

---

## Creating a Channel

`channel()` returns a sender and receiver as a destructured pair.

```
(intSender work_channel, intReceiver listen_channel) = channel()
```

```rust
let (work_channel, listen_channel) = std::sync::mpsc::channel::<i32>();
```

The sender and receiver must be destructured immediately — `channel()` is not valid on the right side of a single binding.

---

## Sending

`send(value) to channel` is a built-in statement. The value must be a named variable of the channel's element type. The channel must be a `sender of T`.

```
send(result) to reply_channel
```

```rust
reply_channel.send(result).unwrap();
```

Send panics if the receiver has been dropped. In v1 there is no recoverable send — see [V2 Roadmap](v2.md).

---

## Receiving

`receive(channel)` is a built-in function that blocks the current thread until a value arrives. The channel must be a `receiver of T`.

```
int result = receive(listen_channel)
```

```rust
let result: i32 = listen_channel.recv().unwrap();
```

Receive panics if all senders have been dropped and the channel is empty.

---

## Spawning a Thread

`thread` spawns a named function on a new OS thread. Arguments follow the function name.

```
thread worker(id, work_channel)
```

```rust
let _work_channel_clone = work_channel.clone();
std::thread::spawn(move || worker(id, _work_channel_clone));
```

**Senders are automatically cloned** when passed to `thread`. The original sender remains valid in the calling thread after the spawn. Struct arguments are also cloned automatically, matching the behavior of function calls elsewhere in Deor.

Only named functions can be spawned — no anonymous functions, matching Deor's no-lambda rule.

---

## Spawning Multiple Threads

Spawn inside a `for` loop. Each iteration gets its own clone of the sender.

```
shape intSender = sender of int
shape intReceiver = receiver of int

fn void main()
    (intSender work_channel, intReceiver listen_channel) = channel()

    for i in range(5)
        thread worker(i, work_channel)

    for i in range(5)
        int result = receive(listen_channel)
        print(result)

fn void worker(int id, intSender reply_channel)
    int result = id * 2
    send(result) to reply_channel
```

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (work_channel, listen_channel) = mpsc::channel::<i32>();

    for i in 0..5 {
        let _work_channel_clone = work_channel.clone();
        thread::spawn(move || worker(i, _work_channel_clone));
    }

    for _ in 0..5 {
        let result = listen_channel.recv().unwrap();
        println!("{}", result);
    }
}

fn worker(id: i32, reply_channel: mpsc::Sender<i32>) {
    let result = id * 2;
    reply_channel.send(result).unwrap();
}
```

Results arrive in the order threads finish — not necessarily the order they were spawned. If ordering matters, include an id in the message and sort after collecting.

---

## Passing Results with Identity

When you need to know which thread produced which result, send a struct.

```
struct WorkResult
    int id
    int value

shape resultSender = sender of WorkResult
shape resultReceiver = receiver of WorkResult

fn void main()
    (resultSender work_channel, resultReceiver listen_channel) = channel()

    for i in range(5)
        thread worker(i, work_channel)

    for i in range(5)
        WorkResult res = receive(listen_channel)
        (id, value) in res
        print(id)
        print(value)

fn void worker(int id, resultSender reply_channel)
    int value = id * 2
    WorkResult res = (id, value)
    send(res) to reply_channel
```

---

## Constraints

- **One receiver per channel.** `receiver of T` cannot be cloned or passed to `thread`. Only the spawning thread may call `receive`.
- **Senders may be cloned freely.** Any number of threads can hold a sender for the same channel.
- **`thread` only accepts named functions.** Inline code blocks are not valid.
- **No `join`.** In v1 there is no way to wait for a thread to finish other than receiving from a channel. If a thread needs to signal completion without returning a value, send a `bool` or a unit struct.
- **Unbounded buffer.** `channel()` is always unbounded — sends never block. A bounded channel (backpressure) is not supported in v1.

---

**Conversion notes:** `channel()` transpiles to `mpsc::channel::<T>()` where `T` is inferred from the shape declaration. `send(value) to channel` transpiles to `channel.send(value).unwrap()`. `receive(channel)` transpiles to `channel.recv().unwrap()`. Each `thread fn(args)` transpiles to `thread::spawn(move || fn(cloned_args))` with sender args cloned immediately before the spawn. The transpiler must track which arguments are `sender of T` types to apply the clone-before-move correctly.
