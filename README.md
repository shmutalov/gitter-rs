# gitter-rs

[![license](https://img.shields.io/badge/license-MIT_or_Apache_2.0-blue.svg "License")](#license)
[![Join the chat at https://gitter.im/gitter-rs/general](https://badges.gitter.im/gitter-rs/general.svg)](https://gitter.im/gitter-rs/general?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

Gitter API in Rust 

 **`WIP`**

https://developer.gitter.im

#### Install

```toml
[dependencies]
gitter = { git = "https://github.com/shmutalov/gitter-rs" }
```

- [Initialize](#initialize)
- [Users](#users)
- [Rooms](#rooms)
- [Messages](#messages)
- [License](#license)

##### Initialize

```rust
extern crate gitter;
use gitter::Gitter;

...

let api = Gitter::new("YOUR_ACCESS_TOKEN");
```

##### Users

- Get current user

```rust
let user = api.get_user().unwrap();
```

##### Rooms

- Get all rooms
```rust
let rooms = api.get_rooms().unwrap();
```

- Get room by id
```rust
let room = api.get_room("roomID").unwrap();
```

- Get rooms of some user
```rust
let rooms = api.get_rooms("userID").unwrap();
```

- Join room
```rust
let room = api.join_room("roomID", "userID").unwrap();
```
	
- Leave room
```rust
let room = api.leave_room("roomID", "userID").unwrap();
```

- Get room id
```rust
let room_id = api.get_room_id("room/uri").unwrap();
```

- Search gitter rooms
```rust
let rooms = api.search_rooms("search/string").unwrap();
```

##### Messages

- Get messages of room
```rust
let messages = api.get_messages("roomID", None).unwrap();
```

- Get one message
```rust
let message = api.get_message("roomID", "messageID").unwrap();
```

- Send message
```rust
api.send_message("roomID", "free chat text").unwrap();
```

##### License

`gitter-rs` is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [`LICENSE-APACHE`](LICENSE-APACHE) and [`LICENSE-MIT`](LICENSE-APACHE) for details.