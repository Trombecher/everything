use std::{fmt::Debug, sync::Arc};

use crate::model::{
    AnyComposite, AnyCompositeProperties, Atom, CompositeApi, CompositeSpecialization,
    Construction, Object,
};

use super::{fmt_composite, fmt_delimited};

#[cfg(test)]
mod tests;

/// Specialization for sets. A set is a composite whose items are members of
/// the form `contains ·· %item%`.
///
/// The set model is open: members that are not `contains` constructions do not
/// disqualify the composite from being a set. Those members are preserved in
/// [`SetComposite::rest`] instead of being rejected.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetComposite(Arc<SetCompositeInner>);

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct SetCompositeInner {
    items: AnyComposite,
    rest: AnyComposite,
}

impl SetComposite {
    /// Creates a set from its items and the other members of the composite.
    #[must_use]
    fn new(items: Vec<Object>, rest: Vec<Object>) -> Self {
        Self(Arc::new(SetCompositeInner {
            items: AnyComposite::from_sorted(sorted_deduplicated(items)),
            rest: AnyComposite::from_sorted(sorted_deduplicated(rest)),
        }))
    }

    /// Creates a set without other members from `items`, which must be sorted
    /// and deduplicated.
    ///
    /// This bypasses the `contains` constructions and the general
    /// specialization chain.
    #[must_use]
    pub(super) fn from_sorted_items(items: impl IntoIterator<Item = Object>) -> Self {
        Self(Arc::new(SetCompositeInner {
            items: AnyComposite::from_sorted(items),
            rest: AnyComposite::from_sorted([]),
        }))
    }

    /// The items of the set, sorted.
    #[must_use]
    pub fn items(&self) -> &[Object] {
        self.0.items.as_slice()
    }

    /// The members of the composite that are not `contains` constructions,
    /// sorted.
    #[must_use]
    pub fn rest(&self) -> &[Object] {
        self.0.rest.as_slice()
    }
}

fn sorted_deduplicated(mut objects: Vec<Object>) -> Vec<Object> {
    objects.sort_unstable();
    objects.dedup();
    objects
}

/// The `contains` construction that makes `item` an item of a set.
pub(super) fn contains_member(item: Object) -> Construction {
    Construction::new(Atom::CONTAINS.into(), item)
}

fn contains_construction(item: Object) -> Object {
    contains_member(item).into()
}

impl Debug for SetComposite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Braces are the documentation's sugar for sets, so they may only be
        // used when the whole composite consists of `contains` members. An
        // open set also has other members and is printed as a composite.
        if !self.rest().is_empty() {
            return fmt_composite(f, self.clone());
        }

        fmt_delimited(f, '{', '}', self.0.items.as_slice().iter())
    }
}

impl CompositeSpecialization for SetComposite {
    fn specialize_from(objects: &[Object]) -> Option<Self> {
        // The empty composite is the empty set.
        if objects.is_empty() {
            return Some(Self::new(Vec::new(), Vec::new()));
        }

        let mut items = Vec::new();
        let mut rest = Vec::new();
        let mut has_contains = false;

        for object in objects {
            if let Object::Construction(Construction { constructor, value }) = object
                && matches!(constructor.as_ref(), Object::Atom(Atom::CONTAINS))
            {
                has_contains = true;
                items.push(value.as_ref().clone());
            } else {
                rest.push(object.clone());
            }
        }

        // Without a `contains` member, nothing distinguishes a set from an
        // arbitrary composite.
        if !has_contains {
            return None;
        }

        Some(Self::new(items, rest))
    }
}

impl CompositeApi for SetComposite {
    fn size(&self) -> u64 {
        self.0.items.size() + self.0.rest.size()
    }

    fn has(&self, object: &Object) -> bool {
        if let Object::Construction(Construction { constructor, value }) = object
            && matches!(constructor.as_ref(), Object::Atom(Atom::CONTAINS))
        {
            self.0.items.has(value)
        } else {
            self.0.rest.has(object)
        }
    }

    fn has_item(&self, item: &Object) -> bool {
        self.0.items.has(item)
    }

    fn item_count(&self) -> u64 {
        self.0.items.size()
    }
}

impl IntoIterator for SetComposite {
    type Item = Object;
    type IntoIter = SetCompositeProperties;

    fn into_iter(self) -> Self::IntoIter {
        SetCompositeProperties::new(
            self.0.items.clone().into_iter(),
            self.0.rest.clone().into_iter(),
        )
    }
}

/// Iterates the members of a set in ascending order by merging the `contains`
/// constructions of the items with the other members of the composite.
#[derive(Clone)]
pub struct SetCompositeProperties {
    items: AnyCompositeProperties,
    rest: AnyCompositeProperties,
    next_item: Option<Object>,
    next_rest: Option<Object>,
}

impl SetCompositeProperties {
    fn new(items: AnyCompositeProperties, rest: AnyCompositeProperties) -> Self {
        let mut properties = Self {
            items,
            rest,
            next_item: None,
            next_rest: None,
        };

        properties.next_item = properties.items.next().map(contains_construction);
        properties.next_rest = properties.rest.next();

        properties
    }
}

impl Iterator for SetCompositeProperties {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        let item_is_next = match (&self.next_item, &self.next_rest) {
            (Some(item), Some(rest)) => item <= rest,
            (Some(_), None) => true,
            (None, Some(_)) => false,
            (None, None) => return None,
        };

        if item_is_next {
            let result = self.next_item.take();

            self.next_item = self.items.next().map(contains_construction);

            result
        } else {
            let result = self.next_rest.take();

            self.next_rest = self.rest.next();

            result
        }
    }
}
