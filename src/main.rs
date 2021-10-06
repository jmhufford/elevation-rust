
struct Hex {
    pub elevation: i8,
    pub occupied: bool,
    location_x: u8,
    location_y: u8,
}
/*
+ 0 1 2 3 4 5 6
0 - - - * * * *
1 - - * * * * *
2 - * * * * * *
3 * * * * * * *
4 * * * * * * -
5 * * * * * - -
6 * * * * - - -
*/
fn make_board(size: u8) -> Vec<Vec<Hex> > {
    let height = size;
    let width = size;
    let mut board: Vec<Vec<Hex> > = Vec::new();
    {
        let mut row: Vec<Hex> = Vec::new();
        for x in 0..width {
            row.push(Hex { elevation: 1, occupied: true, location_x: x, location_y: 0, });
        }
        board.push(row);
    }
    for y in 1..height - 1 {
        let mut row: Vec<Hex> = Vec::new();
        for x in 0..width {
            row.push(Hex { elevation: 0, occupied: false, location_x: x, location_y: y, });
        }
        board.push(row);
    }
    {
        let mut row: Vec<Hex> = Vec::new();
        for x in 0..width {
            row.push(Hex { elevation: -1, occupied: true, location_x: x, location_y: height, });
        }
        board.push(row);
    }
    board
}

fn move_elevation(from: Hex, to: Hex) -> ( Hex, Hex ) {
    if from.occupied || to.occupied {
        (from, to)
    } else {
        (from, to)
    }
}

fn main() {
    let board = make_board(7);
    

    println!("Hello, world!");
}
