use graphql_parser::parse_query;
use graphql_parser::query::Definition;
use graphql_parser::query::Document;
use graphql_parser::query::OperationDefinition;
use graphql_parser::query::ParseError as QueryParseError;
use graphql_parser::Style;

#[derive(Debug)]
enum Operation {
    Query,
    Mutation,
}

#[derive(Debug)]
pub struct QueryParser<'a> {
    maybe_ast: Result<Document<'a, &'a str>, QueryParseError>,
    operation: Option<Operation>,
}

impl<'a> QueryParser<'a> {
    pub fn from_query_str(query: &'a str) -> Self {
        Self {
            maybe_ast: parse_query(query),
            operation: None,
        }
    }

    pub fn parse_and_serialize(query: &str) -> Result<String, QueryParseError> {
        Ok(parse_query::<String>(query)?.format(&Style::default().indent(4)))
    }

    pub fn parse(&mut self, query: &'a str) {
        self.maybe_ast = parse_query(query);
    }

    pub fn serialize(&self) -> Result<String, &QueryParseError> {
        self.maybe_ast
            .as_ref()
            .map(|ast| ast.format(&Style::default().indent(4)))
    }

    pub fn set_operation(&mut self) {
        if let Ok(document) = &self.maybe_ast {
            if let Some(def) = document.definitions.first() {
                match def {
                    Definition::Operation(op) => match op {
                        OperationDefinition::Query(_) => self.operation = Some(Operation::Query),
                        OperationDefinition::Mutation(_) => {
                            self.operation = Some(Operation::Mutation)
                        }
                        // support only query and mutation ops for now
                        _ => self.operation = None,
                    },
                    // if the first definition is not an op, we won't handle this properly for now
                    _ => self.operation = None,
                }
            }
        }
    }
}
