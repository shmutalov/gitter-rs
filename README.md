# gitter-rs

[![license](https://img.shields.io/badge/license-MIT_or_Apache_2.0-blue.svg "License")](#license)
[![](https://tokei.rs/b1/github/shmutalov/gitter-rs)](https://github.com/shmutalov/gitter-rs)
[![](https://travis-ci.org/shmutalov/gitter-rs.svg?branch=master)](https://travis-ci.org/shmutalov/gitter-rs)
[![gitter-rs on crates.io](https://img.shields.io/crates/v/gitter.svg)](https://crates.io/crates/gitter)
[![Join the chat at https://gitter.im/gitter-rs/general](https://badges.gitter.im/gitter-rs/general.svg)](https://gitter.im/gitter-rs/general?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

Gitter API in Rust 

 [**`WIP`**](#roadmap)

https://developer.gitter.im

# Building
```sh
cargo build
```

# Install

### Current version
```toml
[dependencies]
gitter = "0.1.3"
```

### Development version
```toml
[dependencies]
gitter = { git = "https://github.com/shmutalov/gitter-rs" }
```

- [Initialize](#initialize)
- [Users](#users)
- [Rooms](#rooms)
- [Messages](#messages)
- [Documentation](http://docs.rs/gitter)
- [Roadmap](#roadmap)
- [License](#license)

# Initialize

```rust
extern crate gitter;
use gitter::Gitter;

...

let api = Gitter::new("YOUR_ACCESS_TOKEN");
```

_Following code snippets did not updated yet_

# Users

- Get current user

```rust
let user = api.get_user().unwrap();
```

# Rooms


```rust
// Get all rooms
let rooms = api.get_rooms().unwrap();

// Get room by id
let room = api.get_room("roomID").unwrap();

// Get rooms of some user
let rooms = api.get_rooms("userID").unwrap();

// Join room
let room = api.join_room("roomID", "userID").unwrap();

// Leave room
let room = api.leave_room("roomID", "userID").unwrap();

// Get room id
let room_id = api.get_room_id("room/uri").unwrap();

// Search gitter rooms
let rooms = api.search_rooms("search/string").unwrap();
```

# Messages

```rust
// Get messages of room
let messages = api.get_messages("roomID", None).unwrap();

// Get one message
let message = api.get_message("roomID", "messageID").unwrap();

// Send message
api.send_message("roomID", "free chat text").unwrap();
```

# Roadmap

This project is a work in progress.

- [x] Implement base Rest API's and add some tests
- [x] Test `send_message` method
- [x] Implement all Rest API methods
- [ ] Add support for Streaming API
- [ ] Add support for Faye [_Optional_]
- [ ] Add support for automated access token retreive via Github authentification
- [ ] Cover with tests
- [ ] Add examples

# License

`gitter-rs` is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [`LICENSE-APACHE`](LICENSE-APACHE) and [`LICENSE-MIT`](LICENSE-APACHE) for details.
