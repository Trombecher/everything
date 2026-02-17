use std::collections::BTreeSet;

use crate::{
    AssociationForm, IdPattern,
    associations::Association,
    change::ChangeSet,
    error::Error,
    objects::{Id, M_INFERRED, M_TAG, M_UNIQUE},
};

pub struct PotentialSnapshot<'base, 'changes> {
    pub base: &'base [Association],
    pub changes: ChangeSet<'changes>,
}

impl<'base, 'changes> PotentialSnapshot<'base, 'changes> {
    fn tag_is_inferred(&self, tag: Id) -> bool {
        self.iter_stored_values(tag, M_INFERRED).next().is_some()
    }

    pub fn iter_values(&self, target: Id, tag: Id) -> impl Iterator<Item = Id> {
        if self.tag_is_inferred(tag.clone()) {
            todo!("Inferred values")
        }

        self.iter_stored_values(target, tag)
    }

    pub fn iter_stored_values(&self, target: Id, tag: Id) -> impl Iterator<Item = Id> {
        self.base
            .iter()
            .filter(move |a: &&Association| a.tag == tag && a.target == target)
            .filter(|a| !self.changes.contains_removal_of(a))
            .map(|a| a.value.clone())
    }

    pub fn iter_stored(&self) -> impl Iterator<Item = &Association> {
        self.base
            .iter()
            .filter(|a| !self.changes.contains_removal_of(a))
    }

    pub fn iter_stored_targets_by_tag(&self, tag: Id) -> impl Iterator<Item = Id> {
        let mut visited_objects = Vec::new();

        self.iter_stored()
            .filter_map(move |Association { target, tag: t, .. }| {
                if t == &tag && !visited_objects.contains(target) {
                    None
                } else {
                    visited_objects.push(target.clone());
                    Some(target.clone())
                }
            })
    }

    pub fn iter_stored_targets_and_values(&self, tag: Id) -> impl Iterator<Item = (Id, Id)> {
        self.iter_stored()
            .filter_map(move |a| (a.tag == tag).then_some((a.target.clone(), a.value.clone())))
    }

    #[inline]
    fn tag_value_constraint(&self, tag: Id) -> Option<Id> {
        self.iter_stored_values(tag, M_TAG).next()
    }

    fn check(&self) -> Result<(), Error> {
        // Enforce that there exists no v such that (M_TAG, M_UNIQUE, v) in D.
        if self.iter_stored_values(M_TAG, M_UNIQUE).next().is_none() {
            return Err(Error::Missing(AssociationForm {
                tag: IdPattern::Specific(M_TAG),
                target: IdPattern::Specific(M_UNIQUE),
                value: IdPattern::Some,
            }));
        }

        // Constraint (1) -- tags and values
        for a in self.iter_stored() {
            let value_tag = match self.tag_value_constraint(a.tag.clone()) {
                None => {
                    return Err(Error::Missing(AssociationForm {
                        target: IdPattern::Specific(a.tag.clone()),
                        tag: IdPattern::Specific(M_TAG),
                        value: IdPattern::Some,
                    }));
                }
                Some(x) => x,
            };

            self.match_target_tag(a.target.clone(), a.tag.clone())?;

            // TODO: unsafe
            if self
                .iter_values(a.value.clone(), value_tag.clone())
                .next()
                .is_none()
            {
                return Err(Error::Missing(AssociationForm {
                    target: IdPattern::Specific(a.value.clone()),
                    tag: IdPattern::Specific(value_tag),
                    value: IdPattern::Some,
                }));
            }
        }

        // Constraint (2) -- uniqueness
        for tag in self.iter_stored_targets_by_tag(M_UNIQUE) {
            for target in self.iter_stored_targets_by_tag(tag.clone()) {
                let second_value = self.iter_values(target.clone(), tag.clone()).skip(1).next();

                if let Some(second_value) = second_value {
                    return Err(Error::Found(AssociationForm {
                        target: IdPattern::Specific(target.clone()),
                        tag: IdPattern::Specific(tag.clone()),
                        value: IdPattern::Specific(second_value),
                    }));
                }
            }
        }

        todo!()
    }

    fn match_target_tag(&self, target: Id, tag: Id) -> Result<(), Error> {
        todo!()
    }
}

pub struct Snapshot {
    associations: Vec<Association>,
}

impl Snapshot {
    pub fn from_associations(associations: &[Association]) -> Result<Self, Error> {
        let ps = PotentialSnapshot {
            base: associations,
            changes: ChangeSet::empty(),
        };

        ps.check()?;

        Ok(Self {
            associations: associations.into(),
        })
    }

    pub fn view<'a>(&'a self) -> PotentialSnapshot<'a, 'static> {
        PotentialSnapshot {
            base: &self.associations,
            changes: ChangeSet::empty(),
        }
    }
}
