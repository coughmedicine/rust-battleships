mod backwards;
mod game;
mod location;
mod ship;

use std::io::{BufRead, Write};

use game::{Game, Player};
use location::Location;
use ship::{Grid, Ship, ShipDir};

fn read_line_parse<T>(f: impl Fn(String) -> Result<T, String>) -> T {
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let res = f(line);
        match res {
            Ok(val) => {
                return val;
            }
            Err(e) => {
                print!("{}", e);
                std::io::stdout().flush().unwrap();
            }
        }
    }
    unreachable!()
}

fn create_ships_for_player(player: Player, game: &mut Game) {
    for _ in 0..5 {
        println!("{}", game.get_grid(player).get_display(true));
        loop {
            print!("Enter the starting X coordinate: ");
            std::io::stdout().flush().unwrap();
            let x = read_line_parse(|s| {
                s.parse::<i32>()
                    .map_err(|_| "Please enter a valid integer: ".to_string())
            }) - 1;

            print!("Enter the starting Y coordinate: ");
            std::io::stdout().flush().unwrap();
            let y = read_line_parse(|s| {
                s.parse::<i32>()
                    .map_err(|_| "Please enter a valid integer: ".to_string())
            }) - 1;

            print!("Do you want it to be horizontal ('H') or vertical ('V'): ");
            std::io::stdout().flush().unwrap();
            let dir = read_line_parse(|s| match &*s {
                "H" | "h" => Ok(ShipDir::Horz),
                "V" | "v" => Ok(ShipDir::Vert),
                _ => Err("Please enter either H or V: ".to_string()),
            });

            let res = game.add_ship(player, Location { x, y }, dir);

            match res {
                Ok(_) => break,
                Err(e) => println!("The ship could not be placed because: {e}"),
            }
        }
    }
}
fn main() {
    //let new_ship = Ship::new(Location::new(1, 1), ShipDir::Vert, 4);
    //let new_ship2 = Ship::new(Location::new(4, 2), ShipDir::Vert, 3);
    //let new_ship3 = Ship::new(Location::new(2, 3), ShipDir::Horz, 2);
    //let mut griddy = Grid::new(5);
    //griddy.add_ship(new_ship).unwrap();
    //griddy.add_ship(new_ship2).unwrap();
    //griddy.add_ship(new_ship3).unwrap();
    //print!("{}", griddy.get_display(true));
    let mut game = Game::new(10);
    println!("Player 1 please place your ships on the grid:");
    create_ships_for_player(Player::Player1, &mut game);
    println!("Player 2 please place your ships on the grid:");
    create_ships_for_player(Player::Player2, &mut game);

    game.change_to_playing().unwrap();
}
