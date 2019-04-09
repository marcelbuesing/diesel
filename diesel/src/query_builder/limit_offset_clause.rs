use super::QueryFragment;

#[doc(hidden)]
#[derive(Debug, Clone, Copy, QueryId)]
pub struct LimitOffsetClause<Limit, Offset> {
    pub(crate) limit_clause: Limit,
    pub(crate) offset_clause: Offset,
}

#[allow(missing_debug_implementations)]
pub struct BoxedLimitOffsetClause<'a, DB> {
    pub(crate) limit: Option<Box<dyn QueryFragment<DB> + 'a>>,
    pub(crate) offset: Option<Box<dyn QueryFragment<DB> + 'a>>,
}
