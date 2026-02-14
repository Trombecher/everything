use crate::{Database, Snapshot, pages::PageProvider};

pub enum TransactionBase<'db, 'snapshot, P: PageProvider> {
    LatestOnCommit,
    Specific(&'snapshot Snapshot<'db, P>),
}

pub struct Transaction<'db, 'snapshot, P: PageProvider> {
    base: TransactionBase<'db, 'snapshot, P>,
}
