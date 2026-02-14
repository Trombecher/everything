use crate::{
    Error,
    associations::Association,
    hash::StructureHash,
    pages::{PageAllocatorSubsystem, PageProvider},
    structures::Structure,
    transactions::Transaction,
};

pub struct Database<P: PageProvider> {
    allocator: PageAllocatorSubsystem<P>,
}

impl<P: PageProvider> Database<P> {
    pub fn register_structure(
        &self,
        _associations: &[Association],
    ) -> Result<StructureHash, Error> {
        todo!()
    }
}

pub struct Snapshot<'db, P: PageProvider> {
    db: &'db Database<P>,
}

impl<'db, P: PageProvider> Snapshot<'db, P> {
    pub fn structure(&self, _hash: StructureHash) -> &Structure {
        todo!()
    }

    pub fn modify<'snap>(&'snap self) -> Transaction<'db, 'snap, P> {
        todo!()
    }
}
