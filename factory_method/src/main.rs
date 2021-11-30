/// rust has a really good support on factory method


/// Two Major varieties
trait Manipulator{}

trait Figure{
    fn create_manipulator(&self) -> Box<dyn Manipulator>;
}

struct LineFigure;
struct LineManipulator;
impl Manipulator for LineManipulator{}
impl Figure for LineFigure{
    fn create_manipulator(&self) -> Box<dyn Manipulator> { Box::new(LineManipulator{}) }
}

struct TextFigure;
struct TextManipulator;
impl Manipulator for TextManipulator{}
impl Figure for TextFigure{

    fn create_manipulator(&self) -> Box<dyn Manipulator> {
        Box::new(TextManipulator{})
    }
}

/// parameterized factory method
/// It is kind of hard to implement this in rust, we need to have dynamic dispatch

fn main() {
    let test_figure:Box<dyn Figure> = Box::new(TextFigure{}); 
    let manipulator = test_figure.create_manipulator();
}
