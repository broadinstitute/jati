use std::cmp::Ordering;
use std::iter;

pub(crate) struct NonEmptySet<T: Ord> {
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
    pub fn iter(&self) -> Box<dyn Iterator<Item=&T> + '_> {
        let item_iter = iter::once(&self.item);
        match (&self.before, &self.after) {
            (Some(before), Some(after)) => {
                Box::new(before.iter().chain(item_iter).chain(after.iter()))
            },
            (Some(before), None) => {
                Box::new(before.iter().chain(item_iter))
            }
            (None, Some(after)) => {
                Box::new(item_iter.chain(after.iter()))
            },
            (None, None) => {
                Box::new(item_iter)
            }
        }
    }
}
