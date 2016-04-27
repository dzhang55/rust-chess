var searchIndex = {};
searchIndex["chess"] = {"doc":"","items":[[5,"main","chess","",null,{"inputs":[],"output":null}],[0,"page","","",null,null],[5,"main","chess::page","Serves the html page.",null,{"inputs":[],"output":null}],[0,"chess_server","chess","A module for the server logic of Chess. Handles all WebSocket clients using\na relay MPSC channel. Sends information about game logic through Actions.",null,null],[3,"Payload","chess::chess_server","",null,null],[12,"variant","","",0,null],[12,"fields","","",0,null],[4,"Action","","Represents a single, atomic action taken by a client.",null,null],[13,"Connect","","",1,null],[12,"addr","chess::chess_server::Action","",1,null],[13,"Disconnect","chess::chess_server","",1,null],[12,"addr","chess::chess_server::Action","",1,null],[13,"Select","chess::chess_server","",1,null],[12,"addr","chess::chess_server::Action","",1,null],[12,"cell","","",1,null],[13,"Board","chess::chess_server","",1,null],[12,"board","chess::chess_server::Action","",1,null],[12,"check","","",1,null],[12,"checkmate","","",1,null],[13,"Msg","chess::chess_server","",1,null],[12,"user","chess::chess_server::Action","",1,null],[12,"text","","",1,null],[13,"Moves","chess::chess_server","",1,null],[12,"cells","chess::chess_server::Action","",1,null],[13,"Move","chess::chess_server","",1,null],[12,"from","chess::chess_server::Action","",1,null],[12,"to","","",1,null],[5,"start","chess::chess_server","Spawn a WebSocket listener thread.",null,{"inputs":[],"output":null}],[5,"listen","","Create the relay MPSC (multi-producer/single-consumer) channel, spawn the\nrelay thread, then listen for WebSocket clients and spawn their threads.",null,{"inputs":[],"output":null}],[5,"relay_thread","","The relay thread handles all `Action`s received on its MPSC channel\nby sending them out to all of the currently connected clients.\nAlso emits the potential moves to the client making a selection, and\nemits any change in board state to all clients",null,{"inputs":[{"name":"arc"},{"name":"arc"},{"name":"receiver"}],"output":null}],[5,"client_thread","","Each client thread waits for input (or disconnects) from its respective clients\nand relays the appropriate messages via the relay MPSC channel.",null,{"inputs":[{"name":"arc"},{"name":"arc"},{"name":"arc"},{"name":"string"},{"name":"sender"},{"name":"receiver"}],"output":null}],[17,"WS_ADDR","","",null,null],[11,"clone","","",1,{"inputs":[{"name":"action"}],"output":{"name":"action"}}],[11,"fmt","","",1,{"inputs":[{"name":"action"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"encode","","",1,{"inputs":[{"name":"action"},{"name":"__s"}],"output":{"name":"result"}}],[11,"decode","","",1,{"inputs":[{"name":"__d"}],"output":{"name":"result"}}],[11,"encode","","",0,{"inputs":[{"name":"payload"},{"name":"__s"}],"output":{"name":"result"}}],[11,"decode","","",0,{"inputs":[{"name":"__d"}],"output":{"name":"result"}}],[0,"board","chess","A module for the game logic of Chess. Stores the Board object and contains\nmethods for finding potential moves, check/checkmate, etc.",null,null],[3,"Cell","chess::board","Represents the index of a single cell in the board.",null,null],[12,"row","","",2,null],[12,"col","","",2,null],[3,"Board","","Represents the board state.",null,null],[12,"color","","",3,null],[12,"board","","",3,null],[3,"Piece","","Represents a single chess piece.",null,null],[12,"piece_type","","",4,null],[12,"color","","",4,null],[12,"cell","","",4,null],[4,"PieceType","","Represents the possible types of pieces in a given cell.",null,null],[13,"Bishop","","",5,null],[13,"King","","",5,null],[13,"Knight","","",5,null],[13,"Pawn","","",5,null],[13,"Rook","","",5,null],[13,"Queen","","",5,null],[4,"Color","","Represents the color of a piece.",null,null],[13,"Black","","",6,null],[13,"White","","",6,null],[11,"clone","","",2,{"inputs":[{"name":"cell"}],"output":{"name":"cell"}}],[11,"fmt","","",2,{"inputs":[{"name":"cell"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"encode","","",2,{"inputs":[{"name":"cell"},{"name":"__s"}],"output":{"name":"result"}}],[11,"decode","","",2,{"inputs":[{"name":"__d"}],"output":{"name":"result"}}],[11,"clone","","",3,{"inputs":[{"name":"board"}],"output":{"name":"board"}}],[11,"fmt","","",3,{"inputs":[{"name":"board"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"encode","","",3,{"inputs":[{"name":"board"},{"name":"__s"}],"output":{"name":"result"}}],[11,"decode","","",3,{"inputs":[{"name":"__d"}],"output":{"name":"result"}}],[11,"assert_receiver_is_total_eq","","",5,null],[11,"eq","","",5,{"inputs":[{"name":"piecetype"},{"name":"piecetype"}],"output":{"name":"bool"}}],[11,"ne","","",5,{"inputs":[{"name":"piecetype"},{"name":"piecetype"}],"output":{"name":"bool"}}],[11,"clone","","",5,{"inputs":[{"name":"piecetype"}],"output":{"name":"piecetype"}}],[11,"fmt","","",5,{"inputs":[{"name":"piecetype"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"encode","","",5,{"inputs":[{"name":"piecetype"},{"name":"__s"}],"output":{"name":"result"}}],[11,"decode","","",5,{"inputs":[{"name":"__d"}],"output":{"name":"result"}}],[11,"assert_receiver_is_total_eq","","",6,null],[11,"eq","","",6,{"inputs":[{"name":"color"},{"name":"color"}],"output":{"name":"bool"}}],[11,"ne","","",6,{"inputs":[{"name":"color"},{"name":"color"}],"output":{"name":"bool"}}],[11,"clone","","",6,{"inputs":[{"name":"color"}],"output":{"name":"color"}}],[11,"fmt","","",6,{"inputs":[{"name":"color"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"encode","","",6,{"inputs":[{"name":"color"},{"name":"__s"}],"output":{"name":"result"}}],[11,"decode","","",6,{"inputs":[{"name":"__d"}],"output":{"name":"result"}}],[11,"clone","","",4,{"inputs":[{"name":"piece"}],"output":{"name":"piece"}}],[11,"fmt","","",4,{"inputs":[{"name":"piece"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"encode","","",4,{"inputs":[{"name":"piece"},{"name":"__s"}],"output":{"name":"result"}}],[11,"decode","","",4,{"inputs":[{"name":"__d"}],"output":{"name":"result"}}],[11,"new","","",2,{"inputs":[{"name":"i32"},{"name":"i32"}],"output":{"name":"cell"}}],[11,"symmetrical_pieces","","Helper function to put the four symmetrical pieces on the board.",3,{"inputs":[{"name":"i32"},{"name":"i32"},{"name":"vec"},{"name":"piecetype"}],"output":null}],[11,"new","","Initialize the board with starting positions.",3,{"inputs":[],"output":{"name":"board"}}],[11,"white_turn","","Helper function that checks if it is white&#39;s turn.",3,{"inputs":[{"name":"board"}],"output":{"name":"bool"}}],[11,"get_piece","","Get the piece associated with a given cell index.",3,{"inputs":[{"name":"board"},{"name":"cell"}],"output":{"name":"option"}}],[11,"inbounds","","Helper function to check if a proposed cell is in the bounds of the board.",3,{"inputs":[{"name":"board"},{"name":"cell"}],"output":{"name":"bool"}}],[11,"is_empty","","Helper function to check if a cell has a piece.",3,{"inputs":[{"name":"board"},{"name":"cell"}],"output":{"name":"bool"}}],[11,"is_enemy","","Helper function to check if a cell holds an enemy piece.",3,{"inputs":[{"name":"board"},{"name":"cell"},{"name":"cell"}],"output":{"name":"bool"}}],[11,"is_friendly_board","","Helper function to check if a cell holds a friendly piece relative to board.",3,{"inputs":[{"name":"board"},{"name":"cell"}],"output":{"name":"bool"}}],[11,"moves_until_collision","","Returns the potential moves in the given directions, stopping upon a collision.\nUsed for Bishop, Rook, and Queen.",3,{"inputs":[{"name":"board"},{"name":"vec"},{"name":"cell"}],"output":{"name":"vec"}}],[11,"pawn_moves","","Helper function to implement pawn logic.",3,{"inputs":[{"name":"board"},{"name":"cell"}],"output":{"name":"vec"}}],[11,"basic_moves","","Helper function to implement logic for pieces with set directions.\nPieces include the Knight and the King.",3,{"inputs":[{"name":"board"},{"name":"vec"},{"name":"cell"}],"output":{"name":"vec"}}],[11,"potential_moves","","Calculate the potential moves for a given cell index.",3,{"inputs":[{"name":"board"},{"name":"cell"}],"output":{"name":"vec"}}],[11,"self_check","","Helper function to check if a move would place the current player&#39;s king in danger.\nClones the board and makes the move, before running check() on the clone.",3,{"inputs":[{"name":"board"},{"name":"cell"},{"name":"cell"}],"output":{"name":"bool"}}],[11,"switch_color","","Helper function to swap the current player.",3,{"inputs":[{"name":"board"}],"output":null}],[11,"friendly_pieces","","Helper function to return cells of all friendly pieces.",3,{"inputs":[{"name":"board"}],"output":{"name":"vec"}}],[11,"enemy_pieces","","Helper function to return cells of all enemy pieces.",3,{"inputs":[{"name":"board"}],"output":{"name":"vec"}}],[11,"check","","Helper function that checks for check, i.e. if the enemy king is in danger.\nIterates through all friendly pieces and checks if the enemy king is in any\nof their potential moves.",3,{"inputs":[{"name":"board"}],"output":{"name":"bool"}}],[11,"checkmate","","Helper function that checks for checkmate.\nIterates through all enemy pieces and checks if any of their potential moves\ncan bring them out of check.",3,{"inputs":[{"name":"board"}],"output":{"name":"bool"}}],[11,"move_piece","","Helper function that moves a piece from a cell to the target cell",3,{"inputs":[{"name":"board"},{"name":"cell"},{"name":"cell"}],"output":{"name":"bool"}}],[11,"contains_enemy_king","","Helper function that checks if a list of potential\nmoves contains the enemy king.",3,{"inputs":[{"name":"board"},{"name":"vec"}],"output":{"name":"bool"}}]],"paths":[[3,"Payload"],[4,"Action"],[3,"Cell"],[3,"Board"],[3,"Piece"],[4,"PieceType"],[4,"Color"]]};
searchIndex['chess'] = {"items":[[0,"","chess","",null,null],[5,"main","","",null,{"inputs":[],"output":null}],[0,"webpage","","",null,null],[5,"req_handler","chess::webpage","",null,{"inputs":[{"name":"request"},{"name":"response"}],"output":null}],[5,"serve","","",null,{"inputs":[],"output":null}],[17,"HTTP_ADDR","","",null,null],[17,"HTML_DATA","","",null,null],[0,"chess_server","chess","A module for the server logic of Chess. Handles all WebSocket clients using\na relay MPSC channel. Sends information about game logic through Actions.",null,null],[3,"Payload","chess::chess_server","",null,null],[12,"variant","","",0,null],[12,"fields","","",0,null],[4,"Action","","Represents a single, atomic action taken by a client.",null,null],[13,"Connect","","",1,null],[12,"addr","chess::chess_server::Action","",1,null],[13,"Disconnect","chess::chess_server","",1,null],[12,"addr","chess::chess_server::Action","",1,null],[13,"Select","chess::chess_server","",1,null],[12,"cell","chess::chess_server::Action","",1,null],[13,"Move","chess::chess_server","",1,null],[12,"cell","chess::chess_server::Action","",1,null],[13,"Msg","chess::chess_server","",1,null],[12,"user","chess::chess_server::Action","",1,null],[12,"text","","",1,null],[5,"start","chess::chess_server","Spawn a WebSocket listener thread.",null,{"inputs":[],"output":null}],[5,"listen","","Create the relay MPSC (multi-producer/single-consumer) channel, spawn the\nrelay thread, then listen for WebSocket clients and spawn their threads.",null,{"inputs":[],"output":null}],[5,"relay_thread","","The relay thread handles all `Action`s received on its MPSC channel\nby sending them out to all of the currently connected clients.\nAlso emits the potential moves to the client making a selection, and\nemits any change in board state to all clients",null,{"inputs":[{"name":"arc"},{"name":"receiver"}],"output":null}],[5,"client_thread","","Each client thread waits for input (or disconnects) from its respective clients\nand relays the appropriate messages via the relay MPSC channel.",null,{"inputs":[{"name":"string"},{"name":"sender"},{"name":"receiver"}],"output":null}],[17,"WS_ADDR","","",null,null],[11,"clone","","",1,{"inputs":[{"name":"action"}],"output":{"name":"action"}}],[11,"fmt","","",1,{"inputs":[{"name":"action"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"encode","","",1,{"inputs":[{"name":"action"},{"name":"__s"}],"output":{"name":"result"}}],[11,"decode","","",1,{"inputs":[{"name":"action"},{"name":"__d"}],"output":{"name":"result"}}],[11,"encode","","",0,{"inputs":[{"name":"payload"},{"name":"__s"}],"output":{"name":"result"}}],[11,"decode","","",0,{"inputs":[{"name":"payload"},{"name":"__d"}],"output":{"name":"result"}}],[0,"board","chess","A module for the game logic of Chess. Stores the Board object and contains\nmethods for finding potential moves, check/checkmate, etc.",null,null],[3,"Cell","chess::board","Represents the index of a single cell in the board.",null,null],[12,"row","","",2,null],[12,"col","","",2,null],[3,"Board","","Represents the board state.",null,null],[12,"color","","",3,null],[12,"board","","",3,null],[3,"Piece","","Represents a single chess piece.",null,null],[12,"piece_type","","",4,null],[12,"color","","",4,null],[12,"cell","","",4,null],[4,"PieceType","","Represents the possible types of pieces in a given cell.",null,null],[13,"Bishop","","",5,null],[13,"King","","",5,null],[13,"Knight","","",5,null],[13,"Pawn","","",5,null],[13,"Rook","","",5,null],[13,"Queen","","",5,null],[4,"Color","","Represents the color of a piece.",null,null],[13,"Black","","",6,null],[13,"White","","",6,null],[11,"clone","","",2,{"inputs":[{"name":"cell"}],"output":{"name":"cell"}}],[11,"fmt","","",2,{"inputs":[{"name":"cell"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"encode","","",2,{"inputs":[{"name":"cell"},{"name":"__s"}],"output":{"name":"result"}}],[11,"decode","","",2,{"inputs":[{"name":"cell"},{"name":"__d"}],"output":{"name":"result"}}],[11,"clone","","",3,{"inputs":[{"name":"board"}],"output":{"name":"board"}}],[11,"fmt","","",3,{"inputs":[{"name":"board"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"assert_receiver_is_total_eq","","",5,null],[11,"eq","","",5,{"inputs":[{"name":"piecetype"},{"name":"piecetype"}],"output":{"name":"bool"}}],[11,"ne","","",5,{"inputs":[{"name":"piecetype"},{"name":"piecetype"}],"output":{"name":"bool"}}],[11,"clone","","",5,{"inputs":[{"name":"piecetype"}],"output":{"name":"piecetype"}}],[11,"fmt","","",5,{"inputs":[{"name":"piecetype"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"assert_receiver_is_total_eq","","",6,null],[11,"eq","","",6,{"inputs":[{"name":"color"},{"name":"color"}],"output":{"name":"bool"}}],[11,"ne","","",6,{"inputs":[{"name":"color"},{"name":"color"}],"output":{"name":"bool"}}],[11,"clone","","",6,{"inputs":[{"name":"color"}],"output":{"name":"color"}}],[11,"fmt","","",6,{"inputs":[{"name":"color"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",4,{"inputs":[{"name":"piece"}],"output":{"name":"piece"}}],[11,"fmt","","",4,{"inputs":[{"name":"piece"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"new","","Initialize the board with starting positions.",3,{"inputs":[{"name":"board"}],"output":{"name":"board"}}],[11,"get_piece","","Get the piece associated with a given cell index.",3,{"inputs":[{"name":"board"},{"name":"cell"}],"output":{"name":"option"}}],[11,"inbounds","","",3,{"inputs":[{"name":"board"},{"name":"cell"}],"output":{"name":"bool"}}],[11,"is_empty","","",3,{"inputs":[{"name":"board"},{"name":"cell"}],"output":{"name":"bool"}}],[11,"is_enemy","","",3,{"inputs":[{"name":"board"},{"name":"cell"}],"output":{"name":"bool"}}],[11,"moves_until_collision","","Returns the potential moves in the given directions, stopping upon a collision.",3,{"inputs":[{"name":"board"},{"name":"vec"},{"name":"cell"}],"output":{"name":"vec"}}],[11,"pawn_moves","","Helper function to implement pawn logic.",3,{"inputs":[{"name":"board"},{"name":"cell"}],"output":{"name":"vec"}}],[11,"king_moves","","Helper function to implement king logic",3,{"inputs":[{"name":"board"},{"name":"cell"}],"output":{"name":"vec"}}],[11,"self_check","","Helper function to check if a move would place the current player's king in danger.",3,{"inputs":[{"name":"board"},{"name":"piece"},{"name":"cell"}],"output":{"name":"bool"}}],[11,"switch_color","","Helper function to swap the current player.",3,{"inputs":[{"name":"board"}],"output":null}],[11,"potential_moves","","Calculate the potential moves for a given cell index.",3,{"inputs":[{"name":"board"},{"name":"cell"}],"output":{"name":"vec"}}],[11,"check","","Helper function that checks for check",3,{"inputs":[{"name":"board"}],"output":{"name":"bool"}}],[11,"checkmate","","Helper function that checks for checkmate",3,{"inputs":[{"name":"board"}],"output":{"name":"bool"}}],[11,"move_piece","","Helper function that moves a piece to a target cell",3,{"inputs":[{"name":"board"},{"name":"piece"},{"name":"cell"}],"output":null}],[11,"contains_enemy_king","","Helper function that checks if a list of potential\nmoves contains the king",3,{"inputs":[{"name":"board"},{"name":"vec"}],"output":{"name":"bool"}}]],"paths":[[3,"Payload"],[4,"Action"],[3,"Cell"],[3,"Board"],[3,"Piece"],[4,"PieceType"],[4,"Color"]]};
initSearch(searchIndex);
