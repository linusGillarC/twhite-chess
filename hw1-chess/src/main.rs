fn main() {

    
    enum Pieces {
        E, BP, BN, BB, BR, BQ, NK, WP, WN, WB, WR, WQ, WK
    }

    struct Piece {
        //type
    }
    
    //Unfinished visual board representation
    let board = [
        "br", "bn", "bb", "bq", "bk", "bb", "bn", "bw",
        "bp", "bp", "bp", "bp", "bp", "bp", "bp", "bp",
        "e", "e", "e", "e", "e", "e", "e", "e",
        "e", "e", "e", "e", "e", "e", "e", "e",
        "e", "e", "e", "e", "e", "e", "e", "e",
        "e", "e", "e", "e", "e", "e", "e", "e",
        "wp", "wp", "wp", "wp", "wp", "wp", "wp", "wp",
        "wr", "wn", "wb", "wq", "wk", "wb", "wn", "wr"
        ];

        for piece in board.into_iter() {
            println!("{}", piece)
        }

    //print_board(board);

    //TODO implement an array with board data

}

/* fn print_board(array: board) {
    //prints the initital board
    //TODO fix this to print each line correctly with rank and file next to the board 
    for piece in board.into_iter() {
        println!("{}", piece)
    }
} */