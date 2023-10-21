use tracing::instrument;

use crate::{data::ElementId, raw_operations};

use super::ElementWithChildrenSnapshot;

pub struct SortableChildren {
    pub do_not_move: Option<ElementId>,
    pub children: Vec<(usize, ElementWithChildrenSnapshot)>,
}

impl SortableChildren {
    fn swap(&mut self, one: usize, other: usize) {
        raw_operations::swap_siblings(
            self.children[one].1.element.create.log.element_id.clone(),
            self.children[other].1.element.create.log.element_id.clone(),
        )
        .expect("swapping failed");
        self.children.swap(one, other)
    }
    #[instrument(skip(self), level = "trace")]
    pub fn ordered(mut self) -> Vec<ElementWithChildrenSnapshot> {
        self.reorder_children();
        self.children.into_iter().map(|(_, child)| child).collect()
    }
    /// yes, this is bubble sort
    fn reorder_children(&mut self) {
        let len = self.children.len();

        for i in (0..len).rev() {
            let mut has_swapped = false;
            for j in 0..i {
                if self.children[j].0 > self.children[j + 1].0 {
                    match self
                        .do_not_move
                        .as_ref()
                        .map(|do_not_move| {
                            self.children[j]
                                .1
                                .element
                                .create
                                .log
                                .element_id
                                .eq(do_not_move)
                        })
                        .unwrap_or_default()
                    {
                        false => self.swap(j, j + 1),
                        true => self.swap(j + 1, j),
                    };
                    has_swapped = true;
                }
            }
            if !has_swapped {
                break;
            }
        }
    }
}
