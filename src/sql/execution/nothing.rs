use super::super::engine::Transaction;
use super::super::types::Row;
use super::{Context, Executor, Relation, ResultSet};
use crate::Error;

/// An executor that produces a single empty row
pub struct Nothing;

impl Nothing {
    pub fn new() -> Box<Self> {
        Box::new(Self)
    }
}

impl<T: Transaction> Executor<T> for Nothing {
    fn execute(self: Box<Self>, _ctx: &mut Context<T>) -> Result<ResultSet, Error> {
        Ok(ResultSet::Query {
            relation: Relation {
                columns: Vec::new(),
                rows: Some(Box::new(std::iter::once(Ok(Row::new())))),
            },
        })
    }
}