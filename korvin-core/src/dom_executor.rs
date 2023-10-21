use crate::{
    data::{ElementId, TagName},
    element_builder::{ElementRecipe, ElementWithChildrenRecipe},
    mutation::{
        element::builder_mutation::{
            marker::finish::ElementFinishMutationLog,
            marker::{
                create::{ElementCreateMutation, ElementCreateMutationLog},
                finish::ElementFinishMutation,
            },
            modify::{ElementBuilderModifyMutation, ElementBuilderModifyMutationLog},
        },
        traits::{Perform, Revert},
    },
    raw_operations::{self},
    RuntimeError, RuntimeResult,
};
use itertools::Itertools;
use std::collections::{BTreeMap, HashMap};
use tracing::trace_span;

#[derive(Clone, Debug)]
pub struct DomExecutor {
    pub executed: Option<ElementWithChildrenSnapshot>,
}

#[derive(Debug, Clone)]
pub struct SnapshotEntryV2<Mutation, Log> {
    pub mutation: Mutation,
    pub log: Log,
}

type ElementCreateSnapshotEntry = SnapshotEntryV2<ElementCreateMutation, ElementCreateMutationLog>;
type ElementFinishSnapshotEntry = SnapshotEntryV2<ElementFinishMutation, ElementFinishMutationLog>;
type ElementBuilderModifySnapshotEntry =
    SnapshotEntryV2<ElementBuilderModifyMutation, ElementBuilderModifyMutationLog>;

#[derive(Debug, Clone)]
pub struct ElementSnapshot {
    pub key: Option<u64>,
    pub create: ElementCreateSnapshotEntry,
    pub modify: Vec<ElementBuilderModifySnapshotEntry>,
    pub finish: ElementFinishSnapshotEntry,
}

#[derive(Debug, Clone)]
pub struct ElementWithChildrenSnapshot {
    pub element: ElementSnapshot,
    pub children: Vec<Self>,
}

impl DomExecutor {
    pub fn new(current_root: ElementId) -> Self {
        let kind = TagName::from(current_root.as_ref().tag_name());

        Self {
            executed: Some(ElementWithChildrenSnapshot {
                element: ElementSnapshot {
                    key: None,
                    create: SnapshotEntryV2 {
                        mutation: ElementCreateMutation { kind: kind.clone() },
                        log: ElementCreateMutationLog {
                            kind,
                            element_id: current_root.clone(),
                        },
                    },
                    modify: Default::default(),
                    finish: SnapshotEntryV2 {
                        mutation: ElementFinishMutation {},
                        log: ElementFinishMutationLog {
                            element_id: current_root,
                        },
                    },
                },
                children: Default::default(),
            }),
        }
    }

    #[tracing::instrument(skip(self, new_mutations), level = "trace")]
    pub fn rebuild(&mut self, new_mutations: ElementWithChildrenRecipe) -> RuntimeResult<()> {
        crate::element_builder::value_cache::VALUE_CACHE
            .with(|value_cache| value_cache.borrow_mut().next_rebuild());
        let do_not_move = crate::DOCUMENT // TODO: this could be done once per rebuild
            .with(|document| document.active_element())
            .map(ElementId::new);
        self.executed
            .take()
            .ok_or(RuntimeError::RuntimeCrashedOnPreviousRedraw)
            .and_then(|old| {
                let current_root = old.element.create.log.element_id.clone();
                let _span = trace_span!("rebuilding whole app", app_root=?current_root).entered();
                let new = ElementWithChildrenRecipe {
                    element: ElementRecipe {
                        key: None,
                        create: old.element.create.mutation.clone(),
                        modify: old
                            .element
                            .modify
                            .iter()
                            .map(|e| e.mutation.clone())
                            .collect(),
                        finish: old.element.finish.mutation.clone(),
                    },
                    children: vec![new_mutations],
                };
                old.rebuild(current_root, new, do_not_move)
            })
            .map(|new_snapshot| {
                let _ = self.executed.insert(new_snapshot);
            })
    }
}

#[tracing::instrument(level = "trace")]
fn perform<M, L>(mutation: M, root_element: ElementId) -> RuntimeResult<SnapshotEntryV2<M, L>>
where
    M: Perform<Log = L> + Clone,
    L: Revert,
    L: std::fmt::Debug,
{
    mutation
        .clone()
        .perform(root_element)
        .map(|log| SnapshotEntryV2 { mutation, log })
        .map_err(RuntimeError::Mutation)
}

pub mod reorder_children;

fn build_new_child(
    root_element: ElementId,
    ElementWithChildrenRecipe {
        element:
            ElementRecipe {
                key,
                create,
                modify,
                finish,
            },
        children,
    }: ElementWithChildrenRecipe,
) -> RuntimeResult<ElementWithChildrenSnapshot> {
    let create = perform(create, root_element)?;
    let modify = modify
        .into_iter()
        .map(|mutation| perform(mutation, create.log.element_id.clone()))
        .collect::<Result<_, _>>()?;
    let children = children
        .into_iter()
        .map(|child| build_new_child(create.log.element_id.clone(), child))
        .collect::<Result<_, _>>()?;
    let finish = perform(finish, create.log.element_id.clone())?;
    Ok(ElementWithChildrenSnapshot {
        element: ElementSnapshot {
            key,
            create,
            modify,
            finish,
        },
        children,
    })
}
impl ElementWithChildrenSnapshot {
    #[tracing::instrument(skip(self, recipe), level = "trace")]
    pub fn rebuild(
        self,
        current_root: ElementId,
        recipe: ElementWithChildrenRecipe,
        do_not_move: Option<ElementId>,
    ) -> RuntimeResult<Self> {
        let create = match self.element.create.mutation.eq(&recipe.element.create) {
            true => self.element.create,
            false => return build_new_child(current_root, recipe),
        };
        let root_element = create.log.element_id.clone();
        let modify = {
            let _span = trace_span!("syncing attributes").entered();
            let mut previous_snapshots = self
                .element
                .modify
                .into_iter()
                .map(|old| (old.mutation.clone(), old))
                .collect::<HashMap<_, _>>();
            let already_performed = recipe
                .element
                .modify
                .into_iter()
                .map(|new| (previous_snapshots.remove(&new), new))
                .collect_vec();

            previous_snapshots
                .into_iter()
                .try_for_each(|(_, snapshot)| {
                    snapshot
                        .log
                        .revert()
                        .perform(root_element.clone())
                        .map_err(RuntimeError::UndoingTrailingMutations)
                        .map(|_| ())
                })
                .and_then(|_| {
                    already_performed
                        .into_iter()
                        .map(|(applied, m)| match applied {
                            Some(applied) => Ok(applied),
                            None => perform(m, root_element.clone()),
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?
        };
        let children = {
            let _span = trace_span!("rebuilding children", at_root=?root_element).entered();
            type ChildKey = (Option<u64>, TagName);
            type Child = ElementWithChildrenSnapshot;
            type MatchingChildren = Vec<Child>;

            type Children = BTreeMap<ChildKey, MatchingChildren>;

            let mut old_children: Children = {
                let child_key = |e: &Child| -> ChildKey {
                    (e.element.key, e.element.create.mutation.kind.clone())
                };
                self.children
                    .into_iter()
                    .fold(Children::default(), |mut acc, next| {
                        acc.entry(child_key(&next)).or_default().push(next);
                        acc
                    })
            };
            old_children
                .values_mut()
                .for_each(|children| children.reverse());
            recipe
                .children
                .into_iter()
                .enumerate()
                .map(|(index, new)| {
                    match old_children
                        .get_mut(&(new.element.key, new.element.create.kind.clone()))
                        .and_then(|e| e.pop())
                    {
                        Some(old) => old.rebuild(root_element.clone(), new, do_not_move.clone()),
                        None => build_new_child(root_element.clone(), new),
                    }
                    .map(|child| (index, child))
                })
                .collect::<RuntimeResult<Vec<_>>>()
                .map(|new_children| {
                    old_children.into_values().flatten().for_each(|e| {
                        raw_operations::remove_element_in_place(e.element.create.log.element_id);
                    });
                    new_children
                })
                .map(|children| {
                    reorder_children::SortableChildren {
                        do_not_move,
                        children,
                    }
                    .ordered()
                })?
        };

        let finish = SnapshotEntryV2 {
            mutation: ElementFinishMutation {},
            log: ElementFinishMutationLog {
                element_id: root_element,
            },
        };
        Ok(Self {
            element: ElementSnapshot {
                key: recipe.element.key,
                create,
                modify,
                finish,
            },
            children,
        })
    }
}
