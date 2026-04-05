use crate::dataset::Value;
use crate::dataset::Row;
use crate::dataset::Dataset;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug}; //, Display};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Condition {
    Equal(String, Value),
    Not(Box<Condition>),
    And(Box<Condition>, Box<Condition>),
    Or(Box<Condition>, Box<Condition>),
}

impl Condition {
    pub fn check_filter_condition(&self, row: &Row, dataset: &Dataset) -> bool { //this function serves to check if a row matches the condition and returns a true/false val
        match self {
            Condition::Equal(column, value) => {
                if row.get_value(dataset.column_index(column)) == value {
                    return true; // if the condition is equal, and the value of a column in a row of interest from a dataset matches
                                 // the condition's column and value, then return true
                } 
                return false;
            },
            Condition::Not(condition) => {
                if condition.check_filter_condition(row, dataset) == false {
                    return true;    // if the inner condition is true, return false, and if the inner condition is false, return true
                } 
                return false;
            },
            Condition::And(condition1, condition2) => {
                if condition1.check_filter_condition(row, dataset) == true { //check if condition1 is true
                    if condition2.check_filter_condition(row, dataset) == true {//if it is, check condition 2
                        return true; //and if condition 2 is true, return true
                    }
                }
                return false;
            },
            Condition::Or(condition1,condition2) => {
                if condition1.check_filter_condition(row, dataset) == true { //if condition 1 is true return true
                    return true;
                }
                if condition2.check_filter_condition(row, dataset) == true { //if condition 2 is true return true
                    return true;
                }
                return false;
            },
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Aggregation {
    Count(String),
    Sum(String),
    Average(String),
}
impl Aggregation {
    pub fn get_result_column_name(&self) -> String {
        match self {
            Aggregation::Count(column_name) => format!("Count({column_name})"),
            Aggregation::Sum(column_name) => format!("Sum({column_name})"),
            Aggregation::Average(column_name) => format!("Average({column_name})"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Query {
    filter: Condition,
    group_by: String,
    aggregate: Aggregation,
}
impl Query {
    pub fn new(filter: Condition, group_by: String, aggregate: Aggregation) -> Query {
        return Query {
            filter,
            group_by,
            aggregate,
        };
    }

    pub fn get_filter(&self) -> &Condition {
        return &self.filter;
    }
    pub fn get_group_by(&self) -> &String {
        return &self.group_by;
    }
    pub fn get_aggregate(&self) -> &Aggregation {
        return &self.aggregate;
    }
}