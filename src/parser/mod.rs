use graphql_parser::parse_query;
use graphql_parser::query::Definition;
use graphql_parser::query::Document;
use graphql_parser::query::OperationDefinition;
use graphql_parser::query::ParseError as QueryParseError;
use graphql_parser::query::Selection;
use graphql_parser::Style;

#[derive(Debug)]
enum Operation {
    Query,
    Mutation,
}

enum Selectable<'a> {
    Selection(Selection<'a, &'a str>),
    OperationDefinition(OperationDefinition<'a, &'a str>),
}

#[derive(Debug)]
pub struct QueryParser<'a> {
    ast: Document<'a, &'a str>,
    operation: Option<Operation>,
    cursor_path: Vec<usize>,
}

impl<'a> QueryParser<'a> {
    pub fn from_query_str(query: &'a str) -> Result<Self, QueryParseError> {
        return parse_query(query).map(|ast| Self {
            ast,
            operation: None,
            cursor_path: vec![],
        });
    }

    pub fn parse_and_serialize(query: &str) -> Result<String, QueryParseError> {
        Ok(parse_query::<String>(query)?.format(&Style::default().indent(4)))
    }

    pub fn set_operation(&mut self) {
        if let Some(def) = self.ast.definitions.first() {
            match def {
                Definition::Operation(op) => match op {
                    OperationDefinition::Query(_) => self.operation = Some(Operation::Query),
                    OperationDefinition::Mutation(_) => self.operation = Some(Operation::Mutation),
                    // support only query and mutation ops for now
                    _ => self.operation = None,
                },
                // if the first definition is not an op, we won't handle this properly for now
                _ => self.operation = None,
            }
        }
    }

    pub fn get_cursor_field(&self) -> Option<Selectable> {
        if let Some(def) = self.ast.definitions.first() {
            match def {
                Definition::Operation(op) => match op {
                    OperationDefinition::Query(query) => {
                        let mut selection: Selection<&str>;
                        if self.cursor_path.len() == 0 {
                            return Some(Selectable::OperationDefinition(
                                OperationDefinition::Query(*query),
                            ));
                        }
                        for (vec_index, select_index) in self.cursor_path.iter().enumerate() {
                            if vec_index == 0 {
                                if let Some(select) = query.selection_set.items.get(*select_index) {
                                    selection = *select;
                                }
                            }
                        }
                        return Some(Selectable::Selection(selection));
                    }
                    OperationDefinition::Mutation(_) => None,
                    _ => None,
                },
                _ => None,
            }
        } else {
            None
        }
    }
}
