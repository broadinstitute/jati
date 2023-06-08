struct Tree<T> {
    kids: Vec<Tree<T>>,
    item: T
}

trait Reader<T> {
    fn visit_before_kids(&mut self, item: &T) -> () {}
    fn visit_leaf(&mut self, item: &T) -> () {
        self.visit_before_kids(item);
        self.visit_after_kids(item);
    }
    fn visit_after_kids(&mut self, item: &T) -> () {}
}

trait Annotator<T> {
    fn visit_before_kids(&mut self, item: &mut T) -> () {}
    fn visit_leaf(&mut self, item: &mut T) -> () {
        self.visit_before_kids(item);
        self.visit_after_kids(item);
    }
    fn visit_after_kids(&mut self, item: &mut T) -> () {}
}

