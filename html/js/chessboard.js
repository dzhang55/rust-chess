function load() {
	"use strict";
	var moves = [];
	var history = $("#history");
	var formuser = $("#user");
	var formserver = $("#server");
	var formtext = $("#text");
	var connectbtn = $("#connectbtn");
	var disconnectbtn = $("#disconnectbtn");
	var sendbtn = $("#sendbtn");
	connectbtn.prop("disabled", false);
	disconnectbtn.prop("disabled", true);
	sendbtn.prop("disabled", true);

	function err(value) {
		var elem = $("<div>");
		var em = $("<em>");
		em.text(value);
		elem.append(em);
		history.append(elem);
		history.scrollTop(history.prop("scrollHeight"));
	}

	function write(json) {
		try {
			var msg = JSON.parse(json);
			var elem = $("<div>");

			if (msg.variant == "Msg") {
				var user = $("<strong>");
				user.text(msg.fields[0] + ": ");
				var text = $("<span>");
				text.text(msg.fields[1]);
				elem.append(user);
				elem.append(text);
			} else if (msg.variant == "Connect") {
				var user = $("<strong>");
				user.text(msg.fields[0]);
				elem.append(user);
				elem.append(" connected.");
			} else if (msg.variant == "Disconnect") {
				var user = $("<strong>");
				user.text(msg.fields[0]);
				elem.append(user);
				elem.append(" disconnected.");
			} else if (msg.variant == "Moves") {
				console.log("MOVES!");
				moves = msg.fields[0];
				for (var i = 0; i < moves.length; i++) {
					var square = indexToNotation(moves[i].row, moves[i].col);
					console.log(square);
					greySquare(square);
				}
			} else if (msg.variant == "Board") {
				var state = msg.fields[0];
				console.log(state.board);
				setPosition(chessBoard, state.board);
				if (state.checkmate) {
					$("#check").html("Checkmate!");
				}
				else if (state.check) {
					$("#check").html("Check!");
				}
				else {
					$("#check").html("");
				}
			} else if (msg.variant == "Select") {
			} else if (msg.variant == "Move") {
				removeGreySquares();
			} else {
				err(msg);
				return;
			}

			history.append(elem);
			history.scrollTop(history.prop("scrollHeight"));
		} catch(e) {
			err(json);
		}
	}

	var socket;
	var sockprom;
	$("#connect").submit(function() {
		if (socket) {
			socket.close();
			socket = undefined;
		}

		socket = new WebSocket("ws://" + formserver.val());
		sockprom = new Promise(function(resolve, reject) {
			socket.onopen = resolve;
			socket.onerror = reject;
			$("#board").show();
		}).catch(function(e) {
			err("failed to connect.");
			socket.close();
			socket = undefined;
			sendbtn.prop("disabled", true);
			connectbtn.prop("disabled", false);
			disconnectbtn.prop("disabled", true);
		});

		socket.onmessage = function(ev) {
			write(ev.data);
			console.log("recv: " + ev.data);
		};

		var usr = formuser.val();
		sockprom = sockprom.then(function() {
			if (!socket) { return; }

			sendbtn.prop("disabled", false);
			connectbtn.prop("disabled", true);
			disconnectbtn.prop("disabled", false);
			err("Connected.");
		});
	});

	disconnectbtn.click(function() {
		if (!socket) { return; }

		socket.close();
		socket = undefined;
		sendbtn.prop("disabled", true);
		connectbtn.prop("disabled", false);
		disconnectbtn.prop("disabled", true);
		err("Disconnected.");
	});

	$("#send").submit(function() {
		if (!socket) { return; }

		var usr = formuser.val();
		var txt = formtext.val();
		formtext.val("");
		sockprom = sockprom.then(function() {
			var o = { variant: "Msg", fields: [usr, txt] };
			var s = JSON.stringify(o);
			socket.send(s);
			console.log("send: " + s);
		});
	});
	var onDragStart = function(source, piece, position, orientation) {
		var [row, col] = notationToIndex(source);
		sockprom = sockprom.then(function() {
			var o = { variant: "Select", fields: ["" + row, "" + col] };
			var s = JSON.stringify(o);
			socket.send(s);
			console.log("send: " + s);
		});
	};

	var onDrop = function(source, target, piece, newPos, oldPos, orientation) {
		var [fromRow, fromCol] = notationToIndex(source);
		var [toRow, toCol] = notationToIndex(target);
		removeGreySquares();

		if (!valid(toRow, toCol)) {
			return 'snapback';
		}
		sockprom = sockprom.then(function() {
			var o = { variant: "Move", fields: ["" + fromRow, "" + fromCol, "" + toRow, "" + toCol] };
			var s = JSON.stringify(o);
			socket.send(s);
			console.log("send: " + s);
		})
	};

	var valid = function(row, col) {
		for (var i = 0; i < moves.length; i++) {
			if (moves[i].row == row && moves[i].col == col) {
				return true;
			}
		}
		return false;
	}

	var cfg = {
		draggable: true,
		dropOffBoard: 'snapback', // this is the default
		onDragStart: onDragStart,
		onDrop: onDrop,
		position: 'start'
	};
	var chessBoard = ChessBoard('board', cfg);
}

var notationToIndex = function(source) {
	var col = source.charCodeAt(0) - 97;
	var row = 8 - parseInt(source.charAt(1));
	return [row, col];
};

var indexToNotation = function(row, col) {
	var letter = String.fromCharCode(col + 97);
	var number = 8 - row;
	return letter + number; 
};

var removeGreySquares = function() {
  $('#board .square-55d63').css('background', '');
};

var greySquare = function(square) {
  var squareEl = $('#board .square-' + square);
  
  var background = '#a9a9a9';
  if (squareEl.hasClass('black-3c85d') === true) {
    background = '#696969';
  }

  squareEl.css('background', background);
};

var setPosition = function(chessBoard, board) {
	console.log("IM IN SETPOSITION");
	var position = {};
	for (var i = 0; i < board.length; i++) {
		for (var j = 0; j < board[i].length; j++) {
			if (board[i][j] == null) {
				continue;
			}
			var cell = board[i][j].cell;
			var color;
			switch (board[i][j].color) {
				case "Black":
					color = "b";
					break;
				case "White":
					color = "w";
					break;
			}
			var piece;
			switch (board[i][j].piece_type) {
				case "Knight":
					piece = "N";
					break;
				case "Pawn":
					piece = "P";
					break;
				case "King":
					piece = "K";
					break;
				case "Queen":
					piece = "Q";
					break;
				case "Rook":
					piece = "R";
					break;
				case "Bishop":
					piece = "B";
					break;

			}
			position[indexToNotation(cell.row, cell.col)] = color + piece;
		}
	}
	console.log("setposition");
	console.log(position);
	chessBoard.position(position);
};
