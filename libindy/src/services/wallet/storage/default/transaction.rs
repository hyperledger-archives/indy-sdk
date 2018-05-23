use rusqlite::{TransactionBehavior, Connection, DropBehavior, Result};
use std::ops::Deref;

/// Represents a transaction on a database connection.
///
/// ## Note
///
/// Transactions will roll back by default. Use `commit` method to explicitly commit the
/// transaction, or use `set_drop_behavior` to change what happens when the transaction
/// is dropped.
///
/// ## Example
///
/// ```rust,no_run
/// # use rusqlite::{Connection, Result};
/// # fn do_queries_part_1(_conn: &Connection) -> Result<()> { Ok(()) }
/// # fn do_queries_part_2(_conn: &Connection) -> Result<()> { Ok(()) }
/// fn perform_queries(conn: &mut Connection) -> Result<()> {
///     let tx = try!(conn.transaction());
///
///     try!(do_queries_part_1(&tx)); // tx causes rollback if this fails
///     try!(do_queries_part_2(&tx)); // tx causes rollback if this fails
///
///     tx.commit()
/// }
/// ```
pub struct Transaction<'conn> {
    conn: &'conn Connection,
    drop_behavior: DropBehavior,
    committed: bool,
}

impl<'conn> Transaction<'conn> {
    /// Begin a new transaction. Cannot be nested; see `savepoint` for nested transactions.
    pub fn new(conn: &Connection, behavior: TransactionBehavior) -> Result<Transaction> {
        let query = match behavior {
            TransactionBehavior::Deferred => "BEGIN DEFERRED",
            TransactionBehavior::Immediate => "BEGIN IMMEDIATE",
            TransactionBehavior::Exclusive => "BEGIN EXCLUSIVE",
        };
        conn.execute_batch(query)
            .map(move |_| {
                Transaction {
                    conn: conn,
                    drop_behavior: DropBehavior::Rollback,
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
        self.conn.execute_batch("COMMIT")
    }
}

impl<'conn> Deref for Transaction<'conn> {
    type Target = Connection;

    fn deref(&self) -> &Connection {
        self.conn
    }
}