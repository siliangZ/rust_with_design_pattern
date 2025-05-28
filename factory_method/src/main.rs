/// rust has a really good support on factory method

trait Manipulator {}

trait ManipulatorFactory {
    fn create_manipulator(&self) -> Box<dyn Manipulator>;
}

struct LineManipulatorFactory;
struct LineManipulator;
impl Manipulator for LineManipulator {}
impl ManipulatorFactory for LineManipulatorFactory {
    fn create_manipulator(&self) -> Box<dyn Manipulator> {
        Box::new(LineManipulator {})
    }
}

struct TextManipulatorFactory;
struct TextManipulator;
impl Manipulator for TextManipulator {}
impl ManipulatorFactory for TextManipulatorFactory {
    fn create_manipulator(&self) -> Box<dyn Manipulator> {
        Box::new(TextManipulator {})
    }
}

struct FactoryMethod;

impl FactoryMethod {
    fn create_factory(&self, kind: &str) -> Box<dyn ManipulatorFactory> {
        match kind {
            "line" => Box::new(LineManipulatorFactory {}),
            "text" => Box::new(TextManipulatorFactory {}),
            _ => panic!("Invalid factory type"),
        }
    }
}

fn main() {
    let factory_method = FactoryMethod {};
    let factory: Box<dyn ManipulatorFactory> = factory_method.create_factory("line");
    let manipulator = factory.create_manipulator();
}
