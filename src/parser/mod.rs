use graphql_parser::parse_query;
use graphql_parser::query::Definition;
use graphql_parser::query::Mutation;
use graphql_parser::query::OperationDefinition;
use graphql_parser::query::ParseError as QueryParseError;
use graphql_parser::query::Query;
use graphql_parser::query::Selection;
use graphql_parser::Style;
use std::error::Error;

#[derive(Debug)]
enum Operation<'a> {
    Query(Query<'a, &'a str>),
    Mutation(Mutation<'a, &'a str>),
}

#[derive(Debug)]
pub struct QueryParser<'a> {
    operation: Operation<'a>,
}

impl<'a> QueryParser<'a> {
    /// This constructor truncates the Document that's returned from `graphql_parser::parse_query`.
    /// It only handles one Query or Mutation at a time. Subscription operations are not supported as
    /// of now.
    pub fn from_query_str(query: &'a str) -> Result<Self, Box<dyn Error>> {
        let ast = parse_query(query)?;

        // Extract the first definition
        let operation = ast
            .definitions
            .first()
            .ok_or_else(|| "No definitions found in the query")?;

        let operation = match operation {
            Definition::Operation(op) => match op {
                OperationDefinition::Query(q) => Operation::Query(q.clone()),
                OperationDefinition::Mutation(m) => Operation::Mutation(m.clone()),
                // Ignore Subscription and SelectionSet
                _ => return Err("Only Query and Mutation operations are supported".into()),
            },
            // Ignore Fragment definitions
            _ => return Err("First definition must be an operation".into()),
        };

        Ok(Self { operation })
    }

    pub fn parse_and_serialize(query: &str) -> Result<String, QueryParseError> {
        Ok(parse_query::<String>(query)?.format(&Style::default().indent(4)))
    }

    pub fn get_first_selectable(&'a self) -> Option<&'a Selection<'a, &'a str>> {
        let selection_set = match &self.operation {
            Operation::Query(q) => &q.selection_set,
            Operation::Mutation(m) => &m.selection_set,
        };
        if selection_set.items.is_empty() {
            return None;
        }
        Some(selection_set.items.first().unwrap())
    }
}
