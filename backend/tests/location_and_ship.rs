use rust_learning::location::*;
use rust_learning::ship::*;

#[test]
fn create_ship_from_location() {
    let mut loc = Location::new(2, 5);
    loc.move_dir(Dir::from_input("UP").unwrap());
    let mut s = Ship::new(loc, ShipDir::Horz, 3);
    assert!(s.guess(Location::new(2, 6)));
    assert!(s.guess(Location::new(4, 6)));
    assert!(!s.guess(Location::new(5, 6)));
}
