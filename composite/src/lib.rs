/// Applicability
/// * represent part-whole hierarchies of objects
/// * client ignore the difference between compositions of objects and individual objects.
trait Graphic {
    fn draw(&self);
    fn add(&mut self);
}

struct Line;
impl Graphic for Line {
    fn draw(&self) {
        println!("draw a line");
    }

    fn add(&mut self) {
        eprintln!("can't add to non-composite object");
    }
}

struct Picture {
    members: Vec<Box<dyn Graphic>>,
}
impl Graphic for Picture {
    fn draw(&self) {
        for member in self.members {
            member.draw()
        }
    }
    fn add(&mut self, member: Box<dyn Graphic>) {
        self.members.push(member)
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
