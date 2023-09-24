pub mod game;
pub mod location;
pub mod ship;

use std::{
    borrow::Cow,
    io::{BufRead, Write},
    net::SocketAddr,
    sync::Arc,
};

use anyhow::bail;
use axum::{
    extract::{
        ws::{CloseFrame, Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
    routing::get,
    Error, Json, Router,
};
use futures::{
    sink::SinkExt,
    stream::{select, SplitSink, SplitStream, StreamExt},
};
use game::{Game, Player, StateOrOtherError};
use location::Location;
use serde::{Deserialize, Serialize};
use ship::{Grid, Ship, ShipDir};
use tokio::sync::Mutex;

use crate::game::{ChangeToPlayingError, CheckWinError, GuessError};

struct WaitingState {
    websockets: Mutex<Vec<WebSocket>>,
}

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

fn turn(game: &mut Game, player: Player) -> Option<Player> {
    println!("================");
    println!("{}", game.get_grid(player.other()).get_display(false));

    println!(
        "Player {} please type your guess:",
        match player {
            Player::Player1 => 1,
            Player::Player2 => 2,
        }
    );

    print!("Enter the X coordinate: ");
    std::io::stdout().flush().unwrap();
    let x = read_line_parse(|s| {
        s.parse::<i32>()
            .map_err(|_| "Please enter a valid integer: ".to_string())
    }) - 1;

    print!("Enter the Y coordinate: ");
    std::io::stdout().flush().unwrap();
    let y = read_line_parse(|s| {
        s.parse::<i32>()
            .map_err(|_| "Please enter a valid integer: ".to_string())
    }) - 1;

    let success = game.guess_position(player, Location { x, y }).unwrap();
    match success {
        true => println!("You have hit an enemy ship!"),
        false => println!("You are not epic!"),
    }

    println!("{}", game.get_grid(player.other()).get_display(false));

    game.check_if_win().unwrap()
}

pub fn main_old() {
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

    let winner = loop {
        if let Some(p) = turn(&mut game, Player::Player1) {
            break p;
        }
        if let Some(p) = turn(&mut game, Player::Player2) {
            break p;
        }
    };

    println!(
        "Congratulations Player {}!",
        match winner {
            Player::Player1 => 1,
            Player::Player2 => 2,
        }
    );
}

pub async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/ws", get(ws_handler))
        .with_state(Arc::new(WaitingState {
            websockets: Mutex::new(vec![]),
        }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize)]
struct RootResponse {
    swag: i32,
    yippee_factor: f32,
    adjective: String,
}
async fn root() -> Json<RootResponse> {
    Json(RootResponse {
        swag: 10,
        yippee_factor: 99.0,
        adjective: "cool".to_string(),
    })
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<WaitingState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: Arc<WaitingState>) {
    let mut websockets = state.websockets.lock().await;

    if socket
        .send(Message::Text(format!(
            "You are player {}/2.",
            websockets.len() + 1,
        )))
        .await
        .is_err()
    {
        return;
    }

    websockets.push(socket);

    if websockets.len() == 2 {
        let (s, r): (Vec<_>, Vec<_>) = websockets.drain(..).map(|w| w.split()).unzip();
        let mut s: [_; 2] = s.try_into().unwrap();
        let r: [_; 2] = r.try_into().unwrap();

        let game = Game::new(10);
        if do_game(&mut s, r, game).await.is_err() {
            for mut w in s {
                let _ = w
                    .send(Message::Close(Some(CloseFrame {
                        code: axum::extract::ws::close_code::ERROR,
                        reason: Cow::from("Game Error"),
                    })))
                    .await;
            }
        }
    }
}

#[derive(Deserialize, Debug)]
struct AddShipCommand {
    loc: Location,
    dir: ShipDir,
}
#[derive(Deserialize, Debug)]
struct GuessPosCommand {
    loc: Location,
}

#[derive(Deserialize, Debug)]
enum Command {
    AddShip(AddShipCommand),
    GuessPos(GuessPosCommand),
}

#[derive(Serialize, Debug)]
enum GameState {
    Adding { ships: Vec<Vec<Location>> },
    Guessing {},
    Won(Player),
}

// this macro reduces boring code duplication, needs to be a macro because one of the arguments is the type of command to match
macro_rules! message_to_cmd {
    ($cmd:path, $m:expr) => {
        match $m {
            Message::Text(s) => {
                let parsed = serde_json::from_str::<Command>(&s);
                match parsed {
                    Ok($cmd(c)) => Some(c),
                    _ => None,
                }
            }
            _ => None,
        }
    };
}

async fn do_game(
    senders: &mut [SplitSink<WebSocket, axum::extract::ws::Message>; 2],
    [r1, r2]: [SplitStream<WebSocket>; 2],
    mut game: Game,
) -> anyhow::Result<()> {
    for s in senders.iter_mut() {
        s.send(Message::Text("Game started!".to_owned())).await?;
    }

    let stream1 = r1.map(|m| (Player::Player1, m));
    let stream2 = r2.map(|m| (Player::Player2, m));
    let mut combined_stream = select(stream1, stream2);

    macro_rules! send_adding {
        () => {
            for p in [Player::Player1, Player::Player2] {
                let grid = game.get_grid(p);
                let mut ships = vec![];
                for s in grid.ships.iter() {
                    let mut ship = vec![];
                    for c in s.get_coords() {
                        ship.push(*c);
                    }
                    ships.push(ship);
                }
                let msg = GameState::Adding { ships };
                let msg_str = serde_json::to_string(&msg);
                let msg_ws = Message::Text(msg_str.unwrap());
                senders[p as usize].send(msg_ws).await?;
            }
        };
    }

    // ADDING THE SHIPS
    send_adding!();
    while let Some((p, m)) = combined_stream.next().await {
        println!("Received message");
        let m = m?;
        let cmd = match message_to_cmd!(Command::AddShip, m) {
            Some(value) => value,
            None => continue,
        };
        println!("{:?}", cmd);
        match game.add_ship(p, cmd.loc, cmd.dir) {
            // "e @ p" means "if the variable matches the pattern p, give me the result, call it e"
            Err(e @ StateOrOtherError::WrongState) => {
                // bail comes from anyhow and means "return Err(e) from this function after converting it to an anyhow error"
                bail!(e);
            }
            _ => {}
        }

        send_adding!();

        match game.change_to_playing() {
            Err(e @ ChangeToPlayingError::WrongState) => {
                bail!(e);
            }
            Err(ChangeToPlayingError::NotEnoughShips) => continue,
            Ok(_) => break,
        }
    }

    println!("All ships received, game changed state");

    // GUESSING SHIPS
    while let Some((p, m)) = combined_stream.next().await {
        println!("Received message");
        let m = m?;
        let cmd = match message_to_cmd!(Command::GuessPos, m) {
            Some(value) => value,
            None => continue,
        };
        println!("{:?}", cmd);
        match game.guess_position(p, cmd.loc) {
            Err(e @ GuessError::WrongState) => {
                bail!(e);
            }
            Ok(b) => {
                // "for s in senders" moves out of senders so we need to not do that, ".iter()" is for getting references and ".iter_mut()" is for getting mutable references
                for s in senders.iter_mut() {
                    s.send(Message::Text(format!(
                        "Player {} has guessed {} and {} an enemy ship!",
                        p.num(),
                        cmd.loc,
                        match b {
                            false => "missed",
                            true => "destroyed",
                        }
                    )))
                    .await?;
                }
            }
            _ => {}
        }
        match game.check_if_win() {
            Err(e @ CheckWinError::WrongState) => {
                bail!(e);
            }
            Ok(None) => continue,
            Ok(Some(p)) => {
                for s in senders.iter_mut() {
                    s.send(Message::Text(format!(
                        "Player {} has won the game!",
                        p.num()
                    )))
                    .await?;
                }
                break;
            }
        }
    }

    for s in senders.iter_mut() {
        let _ = s
            .send(Message::Close(Some(CloseFrame {
                code: axum::extract::ws::close_code::NORMAL,
                reason: Cow::from("Game Finished"),
            })))
            .await;
    }

    Ok(())
}
