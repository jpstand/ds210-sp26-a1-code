use std::collections::HashMap;
use crate::dataset::{ColumnType, Dataset, Value, Row};
use crate::query::{Aggregation, Condition, Query};

pub fn filter_dataset(dataset: &Dataset, filter: &Condition) -> Dataset {
    let iterator = dataset.iter(); //using iter rather than into_iter becuase we only need read permissions.
    let columns = dataset.columns(); // copy the structure of the OG dataset.
    let mut filtered_dataset = Dataset::new(columns.clone()); // data set that will be filled and returned
    
    for row in iterator{ // check each row.
       // because Condition has so many possible filters we must match each case. 
       // recursive function bec of the way it is set up in dataset 
        fn check(r: &Row, f: &Condition, ds: &Dataset) -> bool{ //helper function
            match f{
                // string = the name of the column we need to compare the value of
                // value = this is the element we need to compare with.   
                Condition::Equal(column_name ,value)=>{ 
                    let col_index = ds.column_index(column_name); //finding the column index of the column that matters
                    let self_value= r.get_value(col_index); // getting the value at that col_index and row
                    
                    return self_value == value; //compare the two values
                }
                Condition::Not(a,)=>{ 
                    return !check(r,a,ds);
                }
                Condition::And(a,b)=>{
                    return check(r,a,ds) && check(r,b,ds);
                }
                Condition::Or(a,b )=>{
                    return check(r,a,ds) || check(r,b,ds);
                }
            }
        }
        
        if check(row,filter,dataset){ // if it passes the filter, add it to the new dataset. 
            filtered_dataset.add_row(row.clone());
        }
       
    }
    return filtered_dataset;
}

pub fn group_by_dataset(dataset: Dataset, group_by_column: &String) -> HashMap<Value, Dataset> {
    todo!("Implement this!");
}

pub fn aggregate_dataset(dataset: HashMap<Value, Dataset>, aggregation: &Aggregation) -> HashMap<Value, Value> {
    let mut return_hash: HashMap<Value, Value> = HashMap::with_capacity(dataset.len());
    
    for (v,ds) in dataset.iter(){// do the following per dataset
            
        match aggregation{
            Aggregation::Count(s)=>{ 
                let ret_val = Value::Integer(ds.len() as i32); //converts a into a Value struct.
                return_hash.insert(v.clone(), ret_val); //inserts into hashmap
                
            },
            Aggregation::Sum(s)=>{
                 
                let col_index = ds.column_index(s); //finding the column index of the column that matters
                let mut sum = 0;
                for row in ds.iter(){ // go through rows
                    // googled how to use "if let", no AI used
                    if let Value::Integer(int) = row.get_value(col_index){ //for each row go to specific column and get the value, if it is of type Value::Integer()
                        sum += int;
                    }
                }
                    // go the column and add the elements together 
                let ret_val = Value::Integer(sum as i32); //converts a into a Value struct.
                return_hash.insert(v.clone(), ret_val);
            },
            Aggregation::Average(s)=>{
                let col_index = ds.column_index(s); //finding the column index of the column that matters
                let mut sum = 0;

                for row in ds.iter(){ // gets sum 
                    if let Value::Integer(int) = row.get_value(col_index){ //for each row go to specific column and get the value, if it is of type Value::Integer()
                        sum += int;
                    }
                }
            
                let count = ds.len() as i32; // num of rows
                
                let ret_val = Value::Integer(sum / count); //converts a into a Value struct.
                return_hash.insert(v.clone(), ret_val);
                
            }
        }
    }
    return return_hash;
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