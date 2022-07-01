use std::cmp::Ordering;

struct NonEmptySet<T: Ord> {
    before: Option<Box<NonEmptySet<T>>>,
    item: T,
    after: Option<Box<NonEmptySet<T>>>
}

impl<T: Ord> NonEmptySet<T> {
    pub fn new(item: T) -> NonEmptySet<T> {
        let before: Option<Box<NonEmptySet<T>>> = None;
        let after: Option<Box<NonEmptySet<T>>> = None;
        NonEmptySet { before, item, after }
    }
    pub fn add(&mut self, item: T) {
        match self.item.cmp(&item) {
            Ordering::Less => {
                match &mut self.before {
                    None => {
                        self.before = Some(Box::new(NonEmptySet::new(item)))
                    }
                    Some(before) => {
                        before.add(item)
                    }
                }
            }
            Ordering::Equal => {}
            Ordering::Greater => {
                match &mut self.after {
                    None => {
                        self.after = Some(Box::new(NonEmptySet::new(item)))
                    }
                    Some(after) => {
                        after.add(item)
                    }
                }

            }
        }
    }
    pub fn iter(&self) -> NonEmptySetIter<T> {
        NonEmptySetIter::new(self)
    }
}

enum IterStage {
    BeforeIsNext,
    NowAtBefore,
    ItemIsNext,
    AfterIsNext,
    NowAtAfter,
    Exhausted
}

struct NonEmptySetIter<'a, T: Ord> {
    set: &'a NonEmptySet<T>,
    sub_iter: Option<Box<NonEmptySetIter<'a, T>>>,
    stage: IterStage
}

impl<T: Ord> NonEmptySetIter<'_, T>{
    fn new(set: &NonEmptySet<T>) -> NonEmptySetIter<T> {
        let sub_iter: Option<Box<NonEmptySetIter<T>>> = None;
        let stage = IterStage::BeforeIsNext;
        NonEmptySetIter { set, sub_iter, stage }
    }
}

impl<'a, T: Ord> Iterator for NonEmptySetIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.stage {
                IterStage::BeforeIsNext => {
                    match &self.set.before {
                        None => {
                            self.stage = IterStage::ItemIsNext
                        }
                        Some(before) => {
                            self.sub_iter = Some(Box::new(before.iter()));
                            self.stage = IterStage::NowAtBefore;
                        }
                    }
                }
                IterStage::NowAtBefore => {

                }
                IterStage::ItemIsNext => {}
                IterStage::AfterIsNext => {}
                IterStage::NowAtAfter => {}
                IterStage::Exhausted => {}
            }

        }
    }
}

