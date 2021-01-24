# Chess Engine

This project was created by Christopher Lang for his computer science EPQ.
The program displays a chess board and allows the user to input moves as white. The computer will respond with its own moves as black. Based of playing the chess computer on [chess.com](https://www.chess.com/home) at various lavels - I have concluded that this chess engine has an elo rating of about 1300-1500 (its about as good as an average club player).

## Features
- Easy to use GUI
- Max depth of 7
- Endgame tables
- Alpha beta pruning
- Special moves (En passant, Castling, Queening)

## Limitations
- Only plays black
- Evaluation does not adapt according to game phase
- Evaluation only considers material and [piece-square tables](https://www.chessprogramming.org/Piece-Square_Tables)
- Does not adapt depth according to how long its taking to play a move
- Pawns can be promoted only to queens
- No draw by repetition
- No fifty-move rule
- No draw by insufficient mating material