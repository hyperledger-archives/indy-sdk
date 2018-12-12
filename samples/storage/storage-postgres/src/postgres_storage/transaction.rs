use postgres::{Connection, Result};
use std::ops::Deref;

pub struct Transaction<'conn> {
    conn: &'conn Connection,
    committed: bool,
}

impl<'conn> Transaction<'conn> {
    /// Begin a new transaction. Cannot be nested; see `savepoint` for nested transactions.
    pub fn new(conn: &Connection) -> Result<Transaction> {
        let query = "START TRANSACTION";
        conn.batch_execute(query)
            .map(move |_| {
                Transaction {
                    conn,
                    committed: false,
                }
            })
    }

    /// A convenience method which consumes and commits a transaction.
    pub fn commit(mut self) -> Result<()> {
        self.commit_()
    }

    fn commit_(&mut self) -> Result<()> {
        self.committed = true;
        self.conn.batch_execute("COMMIT")
    }

    /// A convenience method which consumes and rolls back a transaction.
    #[allow(dead_code)]
    pub fn rollback(mut self) -> Result<()> {
        self.rollback_()
    }

    fn rollback_(&mut self) -> Result<()> {
        self.committed = true;
        self.conn.batch_execute("ROLLBACK")
    }

    /// Consumes the transaction, committing or rolling back according to the current setting
    /// (see `drop_behavior`).
    ///
    /// Functionally equivalent to the `Drop` implementation, but allows callers to see any
    /// errors that occur.
    #[allow(dead_code)]
    pub fn finish(mut self) -> Result<()> {
        self.finish_()
    }

    fn finish_(&mut self) -> Result<()> {
        if self.committed {
            return Ok(());
        } else {
            self.rollback_()
        }
    }
}

impl<'conn> Deref for Transaction<'conn> {
    type Target = Connection;

    fn deref(&self) -> &Connection {
        self.conn
    }
}

#[allow(unused_must_use)]
impl<'conn> Drop for Transaction<'conn> {
    fn drop(&mut self) {
        self.finish_();
    }
}
