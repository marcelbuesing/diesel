//! Types in this module are mostly internal and automatically generated. You
//! shouldn't need to interact with these types during normal usage, other than
//! the methods on [`Table`](trait.Table.html)
#[doc(hidden)]
pub mod filter;
#[doc(hidden)]
pub mod joins;

use backend::Backend;
use expression::{Expression, SelectableExpression, NonAggregate};
use query_builder::*;
#[doc(hidden)]
pub use self::joins::{InnerJoinSource, LeftOuterJoinSource};
use types::{FromSqlRow, HasSqlType};

pub use self::joins::JoinTo;

/// Trait indicating that a record can be queried from the database. This trait
/// can be derived automatically. See the [codegen
/// documentation](https://github.com/diesel-rs/diesel/tree/master/diesel_codegen#derivequeryable)
/// for more.
pub trait Queryable<ST, DB> where
    DB: Backend + HasSqlType<ST>,
{
    type Row: FromSqlRow<ST, DB>;

    fn build(row: Self::Row) -> Self;
}

#[doc(hidden)]
pub trait QuerySource {
    type FromClause;
    fn from_clause(&self) -> Self::FromClause;
}

/// A column on a database table. Types which implement this trait should have
/// been generated by the [`table!` macro](../macro.table!.html).
pub trait Column: Expression + Default {
    type Table: Table;

    fn name() -> &'static str;
}

/// A SQL database table. Types which implement this trait should have been
/// generated by the [`table!` macro](../macro.table!.html).
pub trait Table: QuerySource + AsQuery + Sized + Default {
    type PrimaryKey: Column<Table=Self> + Expression + NonAggregate;
    type AllColumns: SelectableExpression<Self> + NonAggregate;

    fn name() -> &'static str;
    fn primary_key(&self) -> Self::PrimaryKey;
    fn all_columns() -> Self::AllColumns;
}

impl<T: Table> UpdateTarget for T {
    type Table = Self;
    type WhereClause = ();

    fn where_clause(&self) -> Option<&Self::WhereClause> {
        None
    }
}
