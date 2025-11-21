//! Expression system for lazy evaluation

/// Expression builder for DataFrame operations
#[derive(Debug, Clone)]
pub enum Expr {
    /// Column reference
    Column(String),

    /// Literal value
    Literal(LiteralValue),

    /// Binary operation
    BinaryOp {
        /// Left operand
        left: Box<Expr>,
        /// Operator
        op: Operator,
        /// Right operand
        right: Box<Expr>,
    },

    /// Aggregation
    Agg {
        /// Input expression
        input: Box<Expr>,
        /// Aggregation function
        func: AggFunc,
    },

    /// Alias
    Alias {
        /// Expression to alias
        expr: Box<Expr>,
        /// New name
        name: String,
    },
}

/// Literal value types
#[derive(Debug, Clone)]
pub enum LiteralValue {
    /// Float
    Float64(f64),
    /// Integer
    Int64(i64),
    /// Boolean
    Bool(bool),
    /// String
    String(String),
}

/// Binary operators
#[derive(Debug, Clone, Copy)]
pub enum Operator {
    /// Addition
    Add,
    /// Subtraction
    Sub,
    /// Multiplication
    Mul,
    /// Division
    Div,
    /// Greater than
    Gt,
    /// Greater than or equal
    GtEq,
    /// Less than
    Lt,
    /// Less than or equal
    LtEq,
    /// Equal
    Eq,
    /// Not equal
    NotEq,
    /// Logical AND
    And,
    /// Logical OR
    Or,
}

/// Aggregation functions
#[derive(Debug, Clone, Copy)]
pub enum AggFunc {
    /// Sum
    Sum,
    /// Mean
    Mean,
    /// Min
    Min,
    /// Max
    Max,
    /// Count
    Count,
    /// Standard deviation
    Std,
    /// Variance
    Var,
    /// Median
    Median,
}

impl Expr {
    /// Create an alias for this expression
    pub fn alias(self, name: impl Into<String>) -> Self {
        Self::Alias {
            expr: Box::new(self),
            name: name.into(),
        }
    }

    /// Apply sum aggregation
    pub fn sum(self) -> Self {
        Self::Agg {
            input: Box::new(self),
            func: AggFunc::Sum,
        }
    }

    /// Apply mean aggregation
    pub fn mean(self) -> Self {
        Self::Agg {
            input: Box::new(self),
            func: AggFunc::Mean,
        }
    }

    /// Apply std aggregation
    pub fn std(self) -> Self {
        Self::Agg {
            input: Box::new(self),
            func: AggFunc::Std,
        }
    }

    /// Apply median aggregation
    pub fn median(self) -> Self {
        Self::Agg {
            input: Box::new(self),
            func: AggFunc::Median,
        }
    }
}

// Operator overloading for expressions
impl std::ops::Add for Expr {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::BinaryOp {
            left: Box::new(self),
            op: Operator::Add,
            right: Box::new(rhs),
        }
    }
}

impl std::ops::Sub for Expr {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::BinaryOp {
            left: Box::new(self),
            op: Operator::Sub,
            right: Box::new(rhs),
        }
    }
}

impl std::ops::Mul for Expr {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self::BinaryOp {
            left: Box::new(self),
            op: Operator::Mul,
            right: Box::new(rhs),
        }
    }
}

impl std::ops::Div for Expr {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self::BinaryOp {
            left: Box::new(self),
            op: Operator::Div,
            right: Box::new(rhs),
        }
    }
}

// Comparison operators
impl Expr {
    /// Greater than
    pub fn gt(self, rhs: Expr) -> Self {
        Self::BinaryOp {
            left: Box::new(self),
            op: Operator::Gt,
            right: Box::new(rhs),
        }
    }

    /// Less than
    pub fn lt(self, rhs: Expr) -> Self {
        Self::BinaryOp {
            left: Box::new(self),
            op: Operator::Lt,
            right: Box::new(rhs),
        }
    }

    /// Equal
    pub fn eq(self, rhs: Expr) -> Self {
        Self::BinaryOp {
            left: Box::new(self),
            op: Operator::Eq,
            right: Box::new(rhs),
        }
    }
}

/// Create a column expression
pub fn col(name: impl Into<String>) -> Expr {
    Expr::Column(name.into())
}

/// Create a literal expression
pub fn lit<T: Into<LiteralValue>>(value: T) -> Expr {
    Expr::Literal(value.into())
}

// Conversions to LiteralValue
impl From<f64> for LiteralValue {
    fn from(v: f64) -> Self {
        Self::Float64(v)
    }
}

impl From<i64> for LiteralValue {
    fn from(v: i64) -> Self {
        Self::Int64(v)
    }
}

impl From<bool> for LiteralValue {
    fn from(v: bool) -> Self {
        Self::Bool(v)
    }
}

impl From<String> for LiteralValue {
    fn from(v: String) -> Self {
        Self::String(v)
    }
}

impl From<&str> for LiteralValue {
    fn from(v: &str) -> Self {
        Self::String(v.to_string())
    }
}
