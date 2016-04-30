# Report

![alt text](https://github.com/dzhang55/rust-chess/raw/master/chess.png "Chess")

## Summary

I built upon our WebSocket Chat homework to create a multiplayer game of Chess!
I implemented all the chess logic on the server side, then modified the Action
protocol in order to allow the client to communicate with the server in order
to highlight potential moves, make moves, and so on. 

Overall, I spent between 20-30 hours on this project.

## Accomplishments

* Fully functioning Chess game with all basic Piece logic implemented.
* Complete Action protocol encapsulating all necessary communiations between
the server and the client.
* Slightly improved chat (where the names of the users automatically match
whether they are White/Black/Spectator).
* Augmented MPSC logic to allow for relaying a message to a specific client.

## Structure and Design

**Board**

A module for the game logic of Chess. Stores the `Board` object and contains
methods for finding potential moves, check/checkmate, etc. It utilizes `Cell`
objects in order to index into the board, which is a `Vec<Vec<Option<Piece>>>`.
The `Piece` object contains an enum of `PieceType`, which is used to
determine the game behavior for each piece. 

For example, some pieces (`Queen`, `Rook`, `Bishop`) can move any number of
squares in a given direction provided there are no collisions. Others (`King`,
`Knight`) can only move to a set number of cells relative to their position.
These different behaviors are generalized into helper functions, and one can
pattern match against the `PieceType` in order to select the correct behavior.

The `Vec<Vec<>>` structure was necessary in order to capture the concept of an 
8x8 board, and I chose to populate it with `Piece` options because each cell
may or may not have a piece. Other enums like the `PieceType` and `Color` are 
pretty straightforward, as they are necessary to determine the valid moves
for a piece. Finally, I chose to use a `Cell` struct containing a row and a
column, and mainly passed around `Cell`s as opposed to `Piece`s because then I
could easily clone them and not worry about accidentally modifying a cloned
`Piece` instead of the `Piece` within the board, or vice versa.


**Chess Server**

The MPSC design followed from the HW, except with the adjustment that instead
of keeping a `Vec` of client senders, I would keep a `HashMap` from IP to
client, so that by keeping track of the black player IP and the white player IP,
I could implement the functionality to send a message to just one of them. This
was necessary for `Action::Moves`, because when the client received this message
it would highlight the potential moves on the board. This should only occur
for the player actually selecting the piece. In addition, both the relay thread
and the client threads had a reference to the board state, though only the relay
thread would take a mutable reference in order to modify the state if necessary.

Finally, the structure of the `Action` protocol was necessary in order to
maintain all logic on the server side. Since the client relied on the server to
generate the potential moves for a given piece, the Select command is important
to give the server the information to calculate the moves. The `Action::Move`
command then allows the server to adjust the board state after a move. Finally,
the `Action::Board` command lets the server relay the new board state to all
clients. It could have been possible for me to move some of this logic to the
client side, i.e. instead of passing an entire new `Board`, the client could
maintain a `Board` object and then I could just pass back the new `Move`.
However, I chose not to do this because in the case of an error the client's
state might not match the server's state, which could lead to strange behavior
across the clients.

## Testing

Because the project is inherently very visual, my testing consisted of verifying
behaviors given specific board states manually. For example, I would verify that 
a pawn could move only one space vertically (or two on its first move), and
could only take pieces diagonally. Then, I would make sure that check and
checkmate worked properly, e.g. if you are in check, the only potential moves
are ones that would take you out of check (and this can be done by moving the
king, taking the enemy piece, or moving another piece to block the path). 

## Limitations

I didn't implement some lesser known moves such as castling and en passant.

The game also doesn't allow for multiple game rooms, as the server just has 
one basic MPSC channel. Thus, any players who join after the first two are 
designated spectators, so they can talk in the chat and watch the game but
cannot make any moves. This could be fixed by creating a vector of the IP
to client maps, where each map would indicate a different game room.

Finally, the game does not handle the win condition very gracefully - I did not
implement a restart button so one would have to restart the server to restart
a game.

## Postmortem

Overall, I'm very satisfied with the modularity of the code for the game logic,
as well as the Action protocol. Because I refactored methods in the Board
so heavily, complex behaviors like check could be reduced to just 3-4 lines of
code. In particular, checkmate sounds unwieldy because you have to see if the
enemy can move any piece to possibly break out of check. However, I could just
clone the board, and leverage methods like `enemy_pieces()`,
`potential_moves()`, `move_piece()`, and `check()`, in order to iterate through
every possible enemy move to see if any of those new states would get out of
check.

In terms of improvements, I would definitely want to work on the limitations
listed above. Some fixes like castling and en passant are small additional
features I could have done (but aren't too high priority), but I think making
multiple game rooms would be an interesting nontrivial feature that could really
improve the quality of my game. In addition, testing board states manually
was doable, but if I could go back I would definitely have made some tests of 
board states - especially because this could be easily done by just checking
the output of `potential_moves()`, `check()`, and so on.
