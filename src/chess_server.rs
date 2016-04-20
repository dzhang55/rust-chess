//! A module for the server logic of Chess. Handles all WebSocket clients using
//! a relay MPSC channel. Sends information about game logic through Actions.

use rustc_serialize::json;
use std::str;
use std::thread;
use std::sync::{Arc, Mutex, mpsc};
use websocket::{Receiver, Sender, Server, Message, WebSocketStream};
use websocket::sender;
use websocket::receiver;
use websocket::message::Type;

use super::board::{Board, Cell};

const WS_ADDR: &'static str = "0.0.0.0:1981";

#[derive(Debug, Clone)]
#[derive(RustcDecodable, RustcEncodable)]
/// Represents a single, atomic action taken by a client.
///
/// DO NOT MODIFY: the JavaScript relies on this!
enum Action {
    Connect { addr: String },
    Disconnect { addr: String },
    Select { cell: Cell },
    Move { cell: Cell },
    Msg {user: String, text: String},
}

#[derive(RustcDecodable, RustcEncodable)]
struct Payload {
	variant: String,
	fields: Vec<String>
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
	let clients = Arc::new(Mutex::new(Vec::new()));
	let clients_clone = clients.clone();
	thread::spawn(move || relay_thread(clients_clone, rx));

	for connection in server {
		let tx = tx.clone();
		let request = connection.unwrap().read_request().unwrap(); // Get the request

		request.validate().unwrap(); // Validate the request

		let response = request.accept(); // Form a response

		let mut client = response.send().unwrap(); // Send the response

		let ip = client.get_mut_sender()
				.get_mut()
				.peer_addr()
				.unwrap();

		let ip_string = format!("{}", ip);

		let (mut sender, receiver) = client.split();
		sender.send_message(&Message::text(String::from("Welcome!"))).unwrap();

		// add new client sender to list of clients
		let ref mut clients_vec = *clients.lock().unwrap();
		clients_vec.push(sender);

		thread::spawn(move || client_thread(ip_string.clone(), tx, receiver));
	}
}

/// The relay thread handles all `Action`s received on its MPSC channel
/// by sending them out to all of the currently connected clients.
/// Also emits the potential moves to the client making a selection, and
/// emits any change in board state to all clients
fn relay_thread(clients: Arc<Mutex<Vec<sender::Sender<WebSocketStream>>>>,
			    mpsc_receiver: mpsc::Receiver<String>) {
	let mut board = Board::new();
	for action in mpsc_receiver {
		let mut clients_vector = clients.lock().unwrap();
		let message = Message::text(action);
		// relay message to all of the clients
		for client_sender in &mut *clients_vector {
			client_sender.send_message(&message).unwrap();
		}
	}
}

/// Each client thread waits for input (or disconnects) from its respective clients
/// and relays the appropriate messages via the relay MPSC channel.
///
/// The messages received-from and sent-to the client should be JSON objects with the same
/// form as rustc_serialize's serialization of the `Action` type.
///
/// * If the client connects, a `Action::Connect` will be relayed with their IP address.
///
/// * If the client disconnects, a `Action::Disconnect` will be relayed with their IP address.
///
/// * If the client sends any other message (i.e. `Action::Msg`), it will be relayed verbatim.
///   (But you should still deserialize and reserialize the `Action` to make sure it is valid!)
fn client_thread(ip: String, mpsc_sender: mpsc::Sender<String>,
                 mut client_receiver: receiver::Receiver<WebSocketStream>) {

	// Send connect message to MPSC channel
	let ca = Action::Connect{addr: ip.clone()};
	let encoded_ca = json::encode(&ca).unwrap();
	mpsc_sender.send(encoded_ca).unwrap();

	for message in client_receiver.incoming_messages() {
		let message: Message = message.unwrap();
		match message.opcode {
			// disconnect
		 	Type::Close => {
				let ca = Action::Disconnect{addr: ip.clone()};
		 		let encoded_ca = json::encode(&ca).unwrap();
		 		mpsc_sender.send(encoded_ca).unwrap();
		 	},
		 	_ => {
		 		// json object with username and message
		 		let payload: Payload = json::decode(str::from_utf8(&message.payload).unwrap()).unwrap();
		 		let ca = Action::Msg{
		 			user: payload.fields[0].clone(),
		 			text: payload.fields[1].clone()
		 		};
		 		let encoded_ca = json::encode(&ca).unwrap();
		 		mpsc_sender.send(encoded_ca).unwrap();
		 	}
		}
	}
}
