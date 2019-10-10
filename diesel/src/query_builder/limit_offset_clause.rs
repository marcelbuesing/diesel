use super::QueryFragment;

/// A helper query node that contains both limit and offset clauses
///
/// This type is only relevant for implementing custom backends
#[derive(Debug, Clone, Copy, QueryId)]
pub struct LimitOffsetClause<Limit, Offset> {
    /// The limit clause
    pub limit_clause: Limit,
    /// The offset clause
    pub offset_clause: Offset,
}

/// A boxed variant of [`LimitOffsetClause`](../struct.LimitOffsetClause.html)
///
/// This type is only relevant for implementing custom backends
#[allow(missing_debug_implementations)]
pub struct BoxedLimitOffsetClause<'a, DB> {
    /// The limit clause
    pub limit: Option<Box<dyn QueryFragment<DB> + 'a>>,
    /// The offset clause
    pub offset: Option<Box<dyn QueryFragment<DB> + 'a>>,
}
