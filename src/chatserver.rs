use std::thread;

const WS_ADDR: &'static str = "0.0.0.0:1981";

#[derive(Debug, Clone)]
#[derive(RustcDecodable, RustcEncodable)]
/// Represents a single, atomic action taken by a chat member.
///
/// DO NOT MODIFY: the JavaScript relies on this!
enum ChatAction {
    Connect { addr: String },
    Disconnect { addr: String },
    Msg { user: String, text: String },
}

/// Spawn a WebSocket listener thread.
pub fn start() {
    thread::spawn(listen);
}

/// Create the relay MPSC (multi-producer/single-consumer) channel, spawn the
/// relay thread, then listen for WebSocket clients and spawn their threads.
fn listen() {
    // TODO
}

/// The relay thread handles all `ChatAction`s received on its MPSC channel
/// by sending them out to all of the currently connected clients.
fn relay_thread(/* TODO */) {
    // TODO
}

/// Each client thread waits for input (or disconnects) from its respective clients
/// and relays the appropriate messages via the relay MPSC channel.
///
/// The messages received-from and sent-to the client should be JSON objects with the same
/// form as rustc_serialize's serialization of the `ChatAction` type.
///
/// * If the client connects, a `ChatAction::Connect` will be relayed with their IP address.
///
/// * If the client disconnects, a `ChatAction::Disconnect` will be relayed with their IP address.
///
/// * If the client sends any other message (i.e. `ChatAction::Msg`), it will be relayed verbatim.
///   (But you should still deserialize and reserialize the `ChatAction` to make sure it is valid!)
fn client_thread(/* TODO */) {
    // TODO
}
