use thiserror::Error;

use crate::{
    location::Location,
    ship::{AddShipError, Grid, Ship, ShipDir},
};
const SHIPS_ORDER: [i32; 5] = [2, 3, 3, 4, 5];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Player {
    Player1 = 0,
    Player2 = 1,
}

impl Player {
    pub fn other(&self) -> Player {
        match self {
            Player::Player1 => Player::Player2,
            Player::Player2 => Player::Player1,
        }
    }

    pub fn num(&self) -> usize {
        match self {
            Player::Player1 => 1,
            Player::Player2 => 2,
        }
    }
}

#[derive(Clone, Debug)]
enum GameState {
    CreateShips { grids: [Grid; 2] },
    PlayGame { grids: [Grid; 2], turn: Player },
    GameOver { grids: [Grid; 2] },
}
#[derive(Debug, Error)]
pub enum GameAddShipError {
    #[error("too many ships on the board")]
    TooManyShips,
    #[error(transparent)]
    Other(AddShipError),
}
#[derive(Debug, Error)]
pub enum StateOrOtherError<E> {
    #[error("the game is not in the correct state for this action")]
    WrongState,
    #[error(transparent)]
    Other(E),
}
#[derive(Debug, Error)]
pub enum ChangeToPlayingError {
    #[error("the game is not in the creating state")]
    WrongState,
    #[error("at least one player's grid is not complete")]
    NotEnoughShips,
}
#[derive(Debug, Error)]
pub enum GuessError {
    #[error("the game is not in the playing state")]
    WrongState,
    #[error("the wrong player has tried to guess")]
    WrongPlayer,
}

#[derive(Debug, Error)]
pub enum CheckWinError {
    #[error("the game is not in the playing state")]
    WrongState,
}

#[derive(Clone, Debug)]
pub struct Game {
    state: GameState,
}

impl Game {
    pub fn new(size: i32) -> Self {
        Self {
            state: GameState::CreateShips {
                grids: [Grid::new(size), Grid::new(size)],
            },
        }
    }

    pub fn get_grid(&self, player: Player) -> &Grid {
        match &self.state {
            GameState::CreateShips { grids } => &grids[player as usize],
            GameState::PlayGame { grids, .. } => &grids[player as usize],
            GameState::GameOver { grids } => &grids[player as usize],
        }
    }

    pub fn add_ship(
        &mut self,
        player: Player,
        start_point: Location,
        ship_dir: ShipDir,
    ) -> Result<(), StateOrOtherError<GameAddShipError>> {
        match &mut self.state {
            GameState::CreateShips { grids } => {
                let count = grids[player as usize].ships.len();

                let size = *SHIPS_ORDER
                    .get(count)
                    .ok_or(StateOrOtherError::Other(GameAddShipError::TooManyShips))?;

                let ship = Ship::new(start_point, ship_dir, size);
                grids[player as usize]
                    .add_ship(ship)
                    .map_err(GameAddShipError::Other)
                    .map_err(StateOrOtherError::Other)
            }
            _ => Err(StateOrOtherError::WrongState),
        }
    }

    pub fn change_to_playing(&mut self) -> Result<(), ChangeToPlayingError> {
        match &mut self.state {
            GameState::CreateShips { grids } => {
                if grids[0].ships.len() < SHIPS_ORDER.len()
                    || grids[1].ships.len() < SHIPS_ORDER.len()
                {
                    return Err(ChangeToPlayingError::NotEnoughShips);
                }
                self.state = GameState::PlayGame {
                    grids: grids.clone(),
                    turn: Player::Player1,
                };
                Ok(())
            }
            _ => Err(ChangeToPlayingError::WrongState),
        }
    }

    pub fn guess_position(&mut self, player: Player, coords: Location) -> Result<bool, GuessError> {
        match &mut self.state {
            GameState::PlayGame { grids, turn } => {
                if player != *turn {
                    return Err(GuessError::WrongPlayer);
                }
                let result = grids[1 - *turn as usize].guess_grid(coords);
                *turn = match player {
                    Player::Player1 => Player::Player2,
                    Player::Player2 => Player::Player1,
                };
                Ok(result)
            }
            _ => Err(GuessError::WrongState),
        }
    }

    pub fn check_if_win(&mut self) -> Result<Option<Player>, CheckWinError> {
        match &self.state {
            GameState::PlayGame { grids, .. } => {
                if grids[0].get_all_found().len() == grids[0].get_all().len() {
                    self.state = GameState::GameOver {
                        grids: grids.clone(),
                    };
                    Ok(Some(Player::Player2))
                } else if grids[1].get_all_found().len() == grids[1].get_all().len() {
                    Ok(Some(Player::Player1))
                } else {
                    Ok(None)
                }
            }
            _ => Err(CheckWinError::WrongState),
        }
    }
}
