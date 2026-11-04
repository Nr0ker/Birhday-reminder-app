use std::{collections::HashMap};
use chrono::{NaiveDate, TimeDelta, Utc};
use std::fs;
use serde_json;



fn main() {
    // counddown will begin from 01.01 as day 1, and will go till 31.12(28-29.12) as 365th
    

    let utc_date = Utc::now().date_naive();

    // getting path to file
    let content = fs::read_to_string("dates.json").unwrap();

    // reading file(json)
    let config: HashMap<String, String> = serde_json::from_str(&content).unwrap();

    println!("{:?}", config);

    let days_away_static: [i64; 6] = [14,7,3,2,1,0];
        // main logic

    for (keys, values) in config.into_iter(){
        let mut full_date = String::from(&keys);
        full_date.push_str(".2026");
        let full_date_format = NaiveDate::parse_from_str(&full_date, "%d.%m.%Y").unwrap();

        let difference: TimeDelta = full_date_format - utc_date;
        let days_away: i64 = difference.num_days();
        

        for i in days_away_static{
            if days_away >= i{
                println!("{:?} has birthday in {:?} at {:?}", values, days_away, &keys);
                break;
            }
        }
        
    }
        
}
