use std::thread;
use std::sync::mpsc;
use std::collections::HashMap;
use websocket::{DataFrame, Client, Server, Sender, Receiver, Message, WebSocketStream};
use websocket::message::Type;
use websocket::header::WebSocketProtocol;

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
	let server = Server::bind(WS_ADDR).unwrap();
	let (tx, rx) = mpsc::channel();
	thread::spawn(|| relay_thread(rx));
	let mut clients = HashMap::<&str, Sender>::new();

	for connection in server {
		let tx = tx.clone();
		let request = connection.unwrap().read_request().unwrap(); // Get the request
		let headers = request.headers.clone(); // Keep the headers so we can check them

		request.validate().unwrap(); // Validate the request

		let mut response = request.accept(); // Form a response

		let mut client = response.send().unwrap(); // Send the response

		let ip = client.get_mut_sender()
				.get_mut()
				.peer_addr()
				.unwrap();

		let (mut sender, mut receiver) = client.split();
		for message in receiver.incoming_messages() {
				let message: Message = message.unwrap();
		}
		//thread::spawn(|| client_thread(receiver));
    // Spawn a client_thread.
	}
    // TODO
}

/// The relay thread handles all `ChatAction`s received on its MPSC channel
/// by sending them out to all of the currently connected clients.
fn relay_thread(receiver: mpsc::Receiver<ChatAction>) {
	for action in receiver {
		let message = try!(json::encode(action));

    // Send message to all clients.
	}
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
fn client_thread(tx: mpsc::Sender<ChatAction>, receiver: Receiver<WebSocketStream>) {
	for message in receiver.incoming_messages() {
		let action = try!(json::decode(message));
		tx.send(json::encode(action));
	}
    // TODO
}
