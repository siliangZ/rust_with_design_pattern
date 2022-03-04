// Goal
// * decouple the abstraction from its implementation so that two could vary independently
// Applicability
// 1. avoid permanent binding between an abstraction and its implementation
// 2. change in the implementation of an abstraction should have no impact on clients
// 3. make platform-independent code
// Difference
// 1. normal bridge pattern request the abstract class to hold the impl object, but Rust doesn't have that feature
// 2. each concrete object needs to duplicate

trait WindowImpl {
    fn dev_draw_text(&self);
    fn dev_draw_line(&self);
}
struct XWindowImpl;
impl WindowImpl for XWindowImpl {
    fn dev_draw_text(&self) {
        println!("Xwindow: draw text");
    }

    fn dev_draw_line(&self) {
        println!("Xwindow: draw line");
    }
}

struct PMWindowImpl;
impl WindowImpl for PMWindowImpl {
    fn dev_draw_text(&self) {
        println!("PMWindow: draw text");
    }

    fn dev_draw_line(&self) {
        println!("PMWindow: draw line");
    }
}

trait Window {
    fn get_impl_obj(&self) -> &Box<dyn WindowImpl>;
    fn draw_rect(&self) {
        let impl_obj = self.get_impl_obj();
        impl_obj.dev_draw_line();
        impl_obj.dev_draw_line();
        impl_obj.dev_draw_line();
        impl_obj.dev_draw_line();
    }
    fn draw_text(&self) {
        let impl_obj = self.get_impl_obj();
        impl_obj.dev_draw_text();
    }
}

trait IconWindow: Window {
    fn draw_boarder(&self) {
        self.draw_rect();
        self.draw_text();
    }
}

struct IconWindowConcrete {
    impl_obj: Box<dyn WindowImpl>,
}
impl IconWindowConcrete {
    fn new(impl_obj: Box<dyn WindowImpl>) -> Self {
        Self { impl_obj }
    }
}
impl Window for IconWindowConcrete {
    fn get_impl_obj(&self) -> &Box<dyn WindowImpl> {
        &self.impl_obj
    }
}
impl IconWindow for IconWindowConcrete {}

#[cfg(test)]
mod tests {
    use crate::{IconWindow, IconWindowConcrete, PMWindowImpl, XWindowImpl};

    #[test]
    fn test_pattern() {
        let xwindow = Box::new(XWindowImpl);
        let pmwindow = Box::new(PMWindowImpl);
        let icon_window_obj = IconWindowConcrete::new(pmwindow);
        icon_window_obj.draw_boarder();
    }
}
