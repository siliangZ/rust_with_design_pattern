trait Wall {}
struct EnchanedWall;
impl Wall for EnchanedWall {}

trait Room {}

struct EnchanedRoom;
impl Room for EnchanedRoom {}

trait Door {}

struct EnchanedDoor;
impl Door for EnchanedDoor {}

/// Abstract Factory
/// It's used to create a family of related objects
/// instead of creating a factory for each object
trait MazeFactory {
    fn make_wall() -> Box<dyn Wall>;
    fn make_room() -> Box<dyn Room>;
    fn make_door() -> Box<dyn Door>;
}
struct EnchanedFactory;
impl MazeFactory for EnchanedFactory {
    fn make_wall() -> std::boxed::Box<(dyn Wall + 'static)> {
        Box::new(EnchanedWall {})
    }
    fn make_room() -> std::boxed::Box<(dyn Room + 'static)> {
        Box::new(EnchanedRoom {})
    }
    fn make_door() -> std::boxed::Box<(dyn Door + 'static)> {
        Box::new(EnchanedDoor {})
    }
}
fn main() {
    let enchaned_factory = EnchanedFactory;
    let room1 = EnchanedFactory::make_room();
    let room2 = EnchanedFactory::make_room();
}
