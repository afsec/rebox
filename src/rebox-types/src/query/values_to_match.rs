use std::fmt::Display;

use crate::{
    schema::column::model::{ColumnName, ColumnValue},
    ReboxResult,
};

#[derive(Debug, Default)]
pub enum ValuesToMatch {
    #[default]
    All,
    Where(ValuesToMatchExpr),
}

impl Display for ValuesToMatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            ValuesToMatch::All => "".into(),
            ValuesToMatch::Where(expr) => format!("WHERE: ({})", expr.to_string()),
        };
        write!(f, "{output}")
    }
}
impl ValuesToMatch {
    pub fn new() -> ValuesToMatchBuilder {
        ValuesToMatchBuilder
    }

    pub fn match_against(
        &self,
        column_name: &ColumnName,
        retrieved_column_value: &ColumnValue,
    ) -> bool {
        match self {
            Self::All => true,
            Self::Where(matches) => matches.against(column_name, retrieved_column_value),
        }
    }
}

#[derive(Debug)]
pub struct ValuesToMatchExpr {
    column_name: ColumnName,
    criteria: ValuesToMatchCriteria,
    value: ColumnValue,
    // TODO
    // expr_chain: Option<ValuesToMatchExprChain>
}

impl Display for ValuesToMatchExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value_str = match &self.value {
            ColumnValue::Bool(b) => format!("{b}"),
            ColumnValue::Integer(i) => format!("'{i}'"),
            ColumnValue::Natural(u) => format!("'{u}'"),
            ColumnValue::Text(s) => format!("'{s}'"),
        };
        write!(f, "{} {} {}", self.column_name, self.criteria, value_str)
    }
}

impl ValuesToMatchExpr {
    pub fn against(&self, column_name: &ColumnName, column_value: &ColumnValue) -> bool {
        if &self.column_name == column_name {
            match self.criteria {
                ValuesToMatchCriteria::Is => &self.value == column_value, // ValuesToMatchCriteria::Contains => ,
            }
        } else {
            false
        }
    }
}

// TODO
// #[derive(Debug)]
// pub enum ValuesToMatchExprChain{
//     And(ValuesToMatchExpr),
//     Or(ValuesToMatchExpr),
//     Not(ValuesToMatchExpr),
// }

impl ValuesToMatchExpr {
    pub fn column_name(&self) -> &ColumnName {
        &self.column_name
    }

    pub fn criteria(&self) -> &ValuesToMatchCriteria {
        &self.criteria
    }

    pub fn value(&self) -> &ColumnValue {
        &self.value
    }
}

// TODO: Implement for all ColumnValue items
#[derive(Debug)]
pub enum ValuesToMatchCriteria {
    Is,
    // TODO
    // Contains,
}

impl Display for ValuesToMatchCriteria {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let signal = match self {
            ValuesToMatchCriteria::Is => '=',
        };
        write!(f, "{signal}")
    }
}

#[derive(Debug)]
pub struct ValuesToMatchBuilder;
impl ValuesToMatchBuilder {
    pub fn set_where(self, column_name: ColumnName) -> ValuesToMatchBuilderS1 {
        ValuesToMatchBuilderS1 { column_name }
    }
}

#[derive(Debug)]
pub struct ValuesToMatchBuilderS1 {
    column_name: ColumnName,
}

impl ValuesToMatchBuilderS1 {
    pub fn is(self) -> ValuesToMatchBuilderS2 {
        let Self { column_name } = self;
        ValuesToMatchBuilderS2 {
            column_name,
            criteria: ValuesToMatchCriteria::Is,
        }
    }
    // TODO
    // pub fn contains(self) -> ValuesToMatchBuilderS2 {
    //     let Self { column_name } = self;
    //     ValuesToMatchBuilderS2 {
    //         column_name,
    //         criteria: ValuesToMatchCriteria::Contains,
    //     }
    // }
}

#[derive(Debug)]
pub struct ValuesToMatchBuilderS2 {
    column_name: ColumnName,
    criteria: ValuesToMatchCriteria,
}

impl ValuesToMatchBuilderS2 {
    pub fn value(self, value: ColumnValue) -> ValuesToMatchBuilderS3 {
        let Self {
            column_name,
            criteria,
        } = self;
        ValuesToMatchBuilderS3 {
            column_name,
            criteria,
            value,
        }
    }
}

#[derive(Debug)]
pub struct ValuesToMatchBuilderS3 {
    column_name: ColumnName,
    criteria: ValuesToMatchCriteria,
    value: ColumnValue,
}
impl ValuesToMatchBuilderS3 {
    pub fn build(self) -> ReboxResult<ValuesToMatch> {
        let Self {
            column_name,
            criteria,
            value,
        } = self;
        // check_value_from_schema()
        Ok(ValuesToMatch::Where(ValuesToMatchExpr {
            column_name,
            criteria,
            value,
        }))
    }
}
