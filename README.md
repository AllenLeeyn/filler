# filler

two robots fight with each other.
They take turns placing a game piece on the Anfield.
A game piece must be placed with exactly one cell overlapping the robot's exisiting area. (cannot overlap opponents)
If a robot cannot plave a piece, it stops there. The other robot continues placing to achieve maximum score.

## Anfield
Anfield is a 2d grid of size n x m.
Must have a starting position for each robot.
Anfield is passed as an argument to the game_engine

## Game pieces
Pieces are randomly managed by the game_engine.
We are not allowed to predict the size or shape untill the game_engine transmit them

## The Robots
- game_engine will write the Anfield, and the piece in the standard input.
- each turn, the game_engine rewrites the anfield and includes a new random piece to be placed
- first line of game_engine is player number in the following format $$$ exec p<number> : [<player path>]
- The robot write to standard output the coordinates of where to place the piece in this format X Y\n.
- A piece can only be place if one cell of the shape (in your piece) covers the cell of a shape placed previously (your territory).
- the Player 1 is represented by a and @. Player 2,is represented by s and $.
- The lowercases (s or a) highlights the piece last placed on the Anfield. At the following turns, that same piece will be represented by the symbols ($ or @), as it won’t be the piece last placed anymore.
- If the solution is wrong the game_engine won't make the robot play anymore but the game continues for the other player.
- If there is a timeout or any other kind of unexpected error from the player the game stops and the player loses.
- If your robot can't place anymore pieces he should still return a result (even if invalid), our robots for example return 0 0\n, when they can't place any more pieces.

## Task
- get input from standard input and write to standard output the coordinates
    - read standard input and output to  standard output
    - check for expected input format
    - handle error?
- test robot with the game_engine
- implement strategy to maximize score
- write test files to ensure solution provide is good