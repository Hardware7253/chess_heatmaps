# chess_heatmaps

The output of this program is an array of 12 sub arrays, each piece has its own sub array.
In the sub arrays the number of times the piece type has moved to a square is counted in each element of the array.
Index 0 of a sub array is the top left corner of the board from the white team perspective, and element 63 is the bottom right corner.

The moves are counted from lichess's [chess-openings data set](https://github.com/lichess-org/chess-openings)

The resulting output array is intended to be used for visualising the most common moves for each type of piece, during the opening of a chess game.

# Piece array indexes
Piece         | Index
------------- | -----
White Pawn    | 0
White Rook    | 1
White Knight  | 2
White Bishop  | 3
White Queen   | 4
White King    | 5
Black Pawn    | 6
Black Rook    | 7
Black Knight  | 8
Black Bishop  | 9
Black Queen   | 10
Black King    | 11
