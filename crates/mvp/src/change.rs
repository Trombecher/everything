use crate::associations::Association;

#[derive(PartialEq, Clone)]
pub enum Change {
    Add(Association),
    Remove(Association),
}

pub struct ChangeSet<'changes> {
    changes: &'changes [Change],
}

impl<'changes> ChangeSet<'changes> {
    pub const fn empty() -> Self {
        Self { changes: &[] }
    }

    pub fn contains_removal_of(&self, association: &Association) -> bool {
        self.changes
            .iter()
            .filter_map(|c| match c {
                Change::Add(_) => None,
                Change::Remove(a) => Some(a),
            })
            .any(|a| a == association)
    }
}
