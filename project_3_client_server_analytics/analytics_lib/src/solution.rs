use std::collections::HashMap;
use crate::dataset::{ColumnType, Dataset, Value, Row};
use crate::query::{Aggregation, Condition, Query};

pub fn filter_dataset(dataset: &Dataset, filter: &Condition) -> Dataset { 
    //we are passing in by ref, so I'm going to make a new dataset while leaving the old one untouched
    let mut filtered_dataset =  Dataset::new(dataset.columns().clone()); //we make an empty dataset with the old datasets columns to be returned after filtering           
    for row in dataset.iter() { //we iterate over each row to check if we the row matches the conditions of interest 
        if filter.check_filter_condition(row, dataset) == true { //we use the helper function I added to query to return true or false if a condition is true,
                                                                 //and if the condition is true for this row, 
            filtered_dataset.add_row(row.clone()); //we add it to the empty dataset, which is now the filtered dataset
        }
    }
    return filtered_dataset; //return the new dataset
}

pub fn group_by_dataset(dataset: Dataset, group_by_column: &String) -> HashMap<Value, Dataset> {
    todo!("Implement this!");
}

pub fn aggregate_dataset(dataset: HashMap<Value, Dataset>, aggregation: &Aggregation) -> HashMap<Value, Value> {
    todo!("Implement this!");
}

pub fn compute_query_on_dataset(dataset: &Dataset, query: &Query) -> Dataset {
    let filtered = filter_dataset(dataset, query.get_filter());
    let grouped = group_by_dataset(filtered, query.get_group_by());
    let aggregated = aggregate_dataset(grouped, query.get_aggregate());

    // Create the name of the columns.
    let group_by_column_name = query.get_group_by();
    let group_by_column_type = dataset.column_type(group_by_column_name);
    let columns = vec![
        (group_by_column_name.clone(), group_by_column_type.clone()),
        (query.get_aggregate().get_result_column_name(), ColumnType::Integer),
    ];

    // Create result dataset object and fill it with the results.
    let mut result = Dataset::new(columns);
    for (grouped_value, aggregation_value) in aggregated {
        result.add_row(Row::new(vec![grouped_value, aggregation_value]));
    }
    return result;
}