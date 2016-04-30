//! A module for the server logic of Chess. Handles all WebSocket clients using
//! a relay MPSC channel. Sends information about game logic through Actions.

use rustc_serialize::json;
use std::str;
use std::thread;
use std::sync::{Arc, Mutex, mpsc};
use std::collections::HashMap;
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
    Select { addr: String, cell: Cell },
    Board { board: Board, check: bool, checkmate: bool},
    Msg { user: String, text: String},
    Moves { cells: Vec<Cell>},
    Move { from: Cell, to: Cell },
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
    let clients = Arc::new(Mutex::new(HashMap::new()));
    let clients_clone = clients.clone();
    let board = Arc::new(Mutex::new(Board::new()));
    let board_clone = board.clone();
    let black_ip = Arc::new(Mutex::new(String::new()));
    let white_ip = Arc::new(Mutex::new(String::new()));
    thread::spawn(move || relay_thread(board_clone, clients_clone, rx));

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

        let ref mut black_ip_mut = *black_ip.lock().unwrap();
        let ref mut white_ip_mut = *white_ip.lock().unwrap();

        // Keep track of the IP of both players.
        if white_ip_mut.is_empty() {
            white_ip_mut.push_str(ip_string.as_str());
        }
        else if black_ip_mut.is_empty() {
            black_ip_mut.push_str(ip_string.as_str());
        }

        let (mut sender, receiver) = client.split();
        sender.send_message(&Message::text(String::from("Welcome!"))).unwrap();

        // Add new client sender to list of clients.
        let ref mut clients_vec = *clients.lock().unwrap();
        clients_vec.insert(ip_string.clone(), sender);

        let black_ip_clone = black_ip.clone();
        let white_ip_clone = white_ip.clone();
        let board_clone = board.clone();
        thread::spawn(move || client_thread(board_clone, black_ip_clone, white_ip_clone,
                                            ip_string.clone(), tx, receiver));
    }
}

/// The relay thread handles all `Action`s received on its MPSC channel. It contains an `Arc<Mutex<Board>>`
/// in order to both access board state and modify it if necessary. Since the client threads also require
/// references to the board state, this must be locked in a Mutex.
///
/// * If it receives an `Action::Select`, it will calculate the potential_moves
/// that can be made by that cell, and then relay an `Action::Moves` to the
/// sender only. 
/// * If it receives an `Action::Move`, it will adjust the board state, check for
/// check and checkmate, switch turns, and then send an `Action::Board` with all
/// the state to all clients.
/// * If it receives any other `Action`, it will relay the `Action` verbatim to all clients.
fn relay_thread(mutex_board: Arc<Mutex<Board>>, clients: Arc<Mutex<HashMap<String, sender::Sender<WebSocketStream>>>>,
                mpsc_receiver: mpsc::Receiver<String>) {
    for action_string in mpsc_receiver {
        println!("{}", action_string);
        let action: Action = json::decode(action_string.as_str()).unwrap();
        let new_action;
        match action {
            Action::Select{ref addr, ref cell} => {
                let ref board = *mutex_board.lock().unwrap();
                let mut cells = board.potential_moves(cell);
                if !board.is_friendly_board(cell) {
                    continue;
                }
                cells.retain(|m| !board.self_check(cell.clone(), m.clone()));
                new_action = Action::Moves{cells: cells};
                let mut clients_map = clients.lock().unwrap();
                let message = Message::text(json::encode(&new_action).unwrap());
                // Relay message to only this client.
                for (client_addr, client_sender) in &mut *clients_map {
                    if addr == client_addr {
                        client_sender.send_message(&message).unwrap();
                    }
                }
                continue;
            },
            Action::Move{ref from, ref to} => {
                // Modify board state and check for check, checkmate.
                let ref mut board = *mutex_board.lock().unwrap();
                board.move_piece(from.clone(), to.clone());
                let check = board.check();
                let checkmate = board.checkmate();
                board.switch_color();
                new_action = Action::Board{board: board.clone(), check: check,
                                           checkmate: checkmate};
            },
            _ => new_action = action,
        }
        let mut clients_map = clients.lock().unwrap();
        let message = Message::text(json::encode(&new_action).unwrap());
        // relay message to all of the clients
        for (addr, client_sender) in &mut *clients_map {
            client_sender.send_message(&message).unwrap();
        }
    }
}

/// Each client thread waits for input (or disconnects) from its respective clients, checks
/// if the command is a valid given the game state, and if so, relays the appropriate messages
/// via the relay MPSC channel.
///
/// The messages received-from and sent-to the client should be JSON objects with the same
/// form as rustc_serialize's serialization of the `Action` type.
///
/// * If the client connects, a `Action::Connect` will be relayed with their IP address.
///
/// * If the client disconnects, a `Action::Disconnect` will be relayed with their IP address.
///
/// * If the client sends a `Action::Msg`, the message will be relayed and the user will be adjusted
/// to Black, White, or Spectator accordingly.
///
/// * If the client sends a `Action::Select`, if it is not their turn then this message will be discarded.
/// Otherwise, the `Action::Select` will be relayed with the selected cell and the client's IP address.
///
/// * If the client sends a `Action::Move`, if it is not their turn then this message will be discarded.
/// Otherwise, the `Action::Move` will be relayed with the from cell and to cell.
fn client_thread(mutex_board: Arc<Mutex<Board>>, black_ip: Arc<Mutex<String>>,
                 white_ip: Arc<Mutex<String>>, ip: String, mpsc_sender: mpsc::Sender<String>,
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
                let ref board = *mutex_board.lock().unwrap();
                match payload.variant.as_ref() {
                    "Select" => {
                        if !((board.white_turn() && ip == *white_ip.lock().unwrap()) ||
                                (!board.white_turn() && ip == *black_ip.lock().unwrap())) {
                            continue;
                        }
                        let action = Action::Select{
                            addr: ip.clone(),
                            cell: Cell::new(
                                payload.fields[0].clone().parse::<i32>().unwrap(),
                                payload.fields[1].clone().parse::<i32>().unwrap()
                            )
                        };
                        let encoded_action = json::encode(&action).unwrap();
                        mpsc_sender.send(encoded_action).unwrap();
                    },
                    "Move" => {
                        if !((board.white_turn() && ip == *white_ip.lock().unwrap()) ||
                                (!board.white_turn() && ip == *black_ip.lock().unwrap())) {
                            continue;
                        }
                        let action = Action::Move{
                            from: Cell::new(
                                payload.fields[0].clone().parse::<i32>().unwrap(),
                                payload.fields[1].clone().parse::<i32>().unwrap()
                            ),
                            to: Cell::new(
                                payload.fields[2].clone().parse::<i32>().unwrap(),
                                payload.fields[3].clone().parse::<i32>().unwrap()
                            )
                        };
                        let encoded_action = json::encode(&action).unwrap();
                        mpsc_sender.send(encoded_action).unwrap();
                    },
                    "Msg" => {
                        let user;
                        if ip == *white_ip.lock().unwrap() {
                            user = "White";
                        }
                        else if ip == *black_ip.lock().unwrap() {
                            user = "Black";
                        } else {
                            user = "Spectator";
                        }
                        let action = Action::Msg{
                            user: String::from(user), 
                            text: payload.fields[1].clone()
                        };
                        let encoded_action = json::encode(&action).unwrap();
                        mpsc_sender.send(encoded_action).unwrap();
                    },
                    _ => println!("should not happen"),
                }
            }
        }
    }
}
