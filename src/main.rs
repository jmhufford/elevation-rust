
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

fn check_in_bounds(to_check: &Hex) -> bool {
   let x_plus_y = to_check.location_x + to_check.location_y; 
   if x_plus_y > 9 || x_plus_y < 3 {
       return false;
   }
   true
}

fn move_elevation(from: Hex, to: Hex) -> (Hex, Hex, bool) {
    if !from.occupied && !to.occupied && from.elevation > -1 && to.elevation < 1 && check_in_bounds(&from) && check_in_bounds(&to) {
        return (
            Hex { elevation: from.elevation - 1 , occupied: false, location_x: from.location_x, location_y: from.location_y },
            Hex { elevation: to.elevation + 1 , occupied: false, location_x: to.location_x, location_y: to.location_y },
            true,
        )
    }
    (from, to, false)
}

fn main() {
    let board = make_board(7);
    let spot1 = Hex { elevation: 1, occupied: false, location_x: 6, location_y: 0 };
    let spot2 = Hex { elevation: 1, occupied: false, location_x: 4, location_y: 4 };
    let (spot3, spot4, success) = move_elevation(spot1, spot2);
    println!("{}", success);
    println!("Hello, world!");
}
