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

# Sub array visualisation
How the indexes of the sub arrays correspond to chess board coordinates

8|00|01|02|03|04|05|06|07|
-|--|--|--|--|--|--|--|--|
7|08|09|10|11|12|13|14|15|
6|16|17|18|19|20|21|22|23|
5|24|25|26|27|28|29|30|31|
4|32|33|34|35|36|37|38|39|
3|40|41|42|43|44|45|46|47|
2|48|49|50|51|52|53|54|55|
1|56|57|58|69|60|61|62|63|
-|A |B |C |D |E |F |G |H |
