use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct QueryRequest<F, S, A: Default> {
    #[serde(rename = "accountId")]
    account_id: String,

    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Filter<F>>,

    #[serde(rename = "sort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<Vec<Comparator<S>>>,

    #[serde(rename = "position")]
    position: i32,

    #[serde(rename = "anchor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    anchor: Option<String>,

    #[serde(rename = "anchorOffset")]
    anchor_offset: i32,

    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<usize>,

    #[serde(rename = "calculateTotal")]
    calculate_total: bool,

    #[serde(flatten)]
    arguments: A,
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum Filter<T> {
    FilterOperator(FilterOperator<T>),
    FilterCondition(T),
}

#[derive(Debug, Clone, Serialize)]
pub struct FilterOperator<T> {
    operator: Operator,
    conditions: Vec<Filter<T>>,
}

#[derive(Debug, Clone, Serialize)]
pub enum Operator {
    #[serde(rename = "AND")]
    And,
    #[serde(rename = "OR")]
    Or,
    #[serde(rename = "NOT")]
    Not,
}

#[derive(Debug, Clone, Serialize)]
pub struct Comparator<A> {
    #[serde(rename = "isAscending")]
    is_ascending: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    collation: Option<String>,

    #[serde(flatten)]
    arguments: A,
}

#[derive(Debug, Clone, Deserialize)]
pub struct QueryResponse {
    #[serde(rename = "accountId")]
    account_id: String,

    #[serde(rename = "queryState")]
    query_state: String,

    #[serde(rename = "canCalculateChanges")]
    can_calculate_changes: bool,

    #[serde(rename = "position")]
    position: i32,

    #[serde(rename = "ids")]
    ids: Vec<String>,

    #[serde(rename = "total")]
    total: Option<usize>,

    #[serde(rename = "limit")]
    limit: Option<usize>,
}

impl<F, S, A: Default> QueryRequest<F, S, A> {
    pub fn new(account_id: String) -> Self {
        QueryRequest {
            account_id,
            filter: None,
            sort: None,
            position: 0,
            anchor: None,
            anchor_offset: 0,
            limit: None,
            calculate_total: false,
            arguments: A::default(),
        }
    }

    pub fn account_id(&mut self, account_id: impl Into<String>) -> &mut Self {
        self.account_id = account_id.into();
        self
    }

    pub fn filter(&mut self, filter: impl Into<Filter<F>>) -> &mut Self {
        self.filter = Some(filter.into());
        self
    }

    pub fn sort(&mut self, sort: impl IntoIterator<Item = Comparator<S>>) -> &mut Self {
        self.sort = Some(sort.into_iter().collect());
        self
    }

    pub fn position(&mut self, position: i32) -> &mut Self {
        self.position = position;
        self
    }

    pub fn anchor(&mut self, anchor: impl Into<String>) -> &mut Self {
        self.anchor = Some(anchor.into());
        self
    }

    pub fn anchor_offset(&mut self, anchor_offset: i32) -> &mut Self {
        self.anchor_offset = anchor_offset;
        self
    }

    pub fn limit(&mut self, limit: usize) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    pub fn arguments(&mut self) -> &mut A {
        &mut self.arguments
    }
}

impl QueryResponse {
    pub fn account_id(&self) -> &str {
        &self.account_id
    }

    pub fn ids(&self) -> &[String] {
        &self.ids
    }

    pub fn total(&self) -> Option<usize> {
        self.total
    }

    pub fn limit(&self) -> Option<usize> {
        self.limit
    }

    pub fn position(&self) -> i32 {
        self.position
    }

    pub fn query_state(&self) -> &str {
        &self.query_state
    }

    pub fn can_calculate_changes(&self) -> bool {
        self.can_calculate_changes
    }
}

impl<A> Comparator<A> {
    pub fn new(arguments: A) -> Self {
        Comparator {
            is_ascending: true,
            collation: None,
            arguments,
        }
    }

    pub fn is_ascending(mut self, is_ascending: bool) -> Self {
        self.is_ascending = is_ascending;
        self
    }

    pub fn collation(mut self, collation: String) -> Self {
        self.collation = Some(collation);
        self
    }
}

impl<T> From<FilterOperator<T>> for Filter<T> {
    fn from(filter: FilterOperator<T>) -> Self {
        Filter::FilterOperator(filter)
    }
}

impl<T> From<T> for Filter<T> {
    fn from(filter: T) -> Self {
        Filter::FilterCondition(filter)
    }
}

impl<T> Filter<T> {
    pub fn and<U, V>(conditions: U) -> Self
    where
        U: IntoIterator<Item = V>,
        V: Into<Filter<T>>,
    {
        Filter::FilterOperator(FilterOperator {
            operator: Operator::And,
            conditions: conditions.into_iter().map(|t| t.into()).collect(),
        })
    }

    pub fn or<U, V>(conditions: U) -> Self
    where
        U: IntoIterator<Item = V>,
        V: Into<Filter<T>>,
    {
        Filter::FilterOperator(FilterOperator {
            operator: Operator::Or,
            conditions: conditions.into_iter().map(|t| t.into()).collect(),
        })
    }

    pub fn not<U, V>(conditions: U) -> Self
    where
        U: IntoIterator<Item = V>,
        V: Into<Filter<T>>,
    {
        Filter::FilterOperator(FilterOperator {
            operator: Operator::Not,
            conditions: conditions.into_iter().map(|t| t.into()).collect(),
        })
    }
}
