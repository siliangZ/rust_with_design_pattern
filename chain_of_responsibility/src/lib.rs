trait Request {}

struct ConcreteRequest;
impl Request for ConcreteRequest {}
trait RequestHandler: RequestHandlerClone {
    fn get_successor(&self) -> Option<Box<dyn RequestHandler>>;
    fn handle(&self, request: Box<dyn Request>) {
        match self.get_successor() {
            Some(successor) => successor.handle(request),
            None => println!("chain breaks here"),
        }
    }
}

trait RequestHandlerClone {
    fn clone_box(&self) -> Box<dyn RequestHandler>;
}
impl<T> RequestHandlerClone for T
where
    T: RequestHandler + Clone + 'static,
{
    fn clone_box(&self) -> Box<dyn RequestHandler> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn RequestHandler> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Clone)]
struct Widget {
    successor: Option<Box<dyn RequestHandler>>,
}

impl RequestHandler for Widget {
    fn get_successor(&self) -> Option<Box<dyn RequestHandler>> {
        self.successor.clone()
    }

    fn handle(&self, _request: Box<dyn Request>) {
        println!("handler in Widget");
    }
}

#[derive(Clone)]
struct Application {
    successor: Option<Box<dyn RequestHandler>>,
}

impl RequestHandler for Application {
    fn get_successor(&self) -> Option<Box<dyn RequestHandler>> {
        self.successor.clone()
    }

    fn handle(&self, _request: Box<dyn Request>) {
        println!("handler in application")
    }
}

#[derive(Clone)]
struct LazyOne {
    successor: Option<Box<dyn RequestHandler>>,
}

impl RequestHandler for LazyOne {
    fn get_successor(&self) -> Option<Box<dyn RequestHandler>> {
        self.successor.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Application, ConcreteRequest, LazyOne, RequestHandler, Widget};

    #[test]
    fn create_success_chain() {
        let application = Application { successor: None };
        let widget = Widget {
            successor: Some(Box::new(application)),
        };
        let request = Box::new(ConcreteRequest);
        widget.handle(request);
    }

    #[test]
    fn create_break_chain() {
        let widget = Widget { successor: None };
        let lazy_one = LazyOne {
            successor: Some(Box::new(widget)),
        };
        let request = Box::new(ConcreteRequest);
        lazy_one.handle(request);
    }
}
