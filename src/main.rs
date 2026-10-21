use std::{collections::HashMap};
use chrono::{Datelike, NaiveDate, TimeDelta, Utc};
//giving each date its own day number 

// fn date_defenition(date: u32) -> u32{



//     let defined_date = date.
// }


fn main() {
    // counddown will begin from 01.01 as day 1, and will go till 31.12(28-29.12) as 365th
    

    let utc_date = Utc::now().date_naive();
    
    let birthday_list = HashMap::from([
            ("09.04", "дед Вася"),
            ("11.04", "Настя"),
            ("25.04", "Андрій"),
            ("20.05", "Мама"),
            ("05.06", "дядя Саша"),
            ("09.04", "Марк"),
            ("16.07", "дядя Паша"),
            ("20.08", "Дядя Діма"),
            ("10.09", "Папа"),
            ("25.09", "Віталік"),
            ("29.09", "Я"),
            ("09.10", "Річард"),
            ("04.11", "Тьотя Оля"),
            ("23.11", "Бабушка Катя"),
            ("10.01", "Дед Ігорь"),
            ("19.01", "Кристина"),
            ("21.01", "Юра"),
        ]);
    
    
        // main logic

        for (keys, values) in birthday_list.into_iter(){
            let mut full_date = String::from(keys);
            full_date.push_str(".2026");
            let full_date_format = NaiveDate::parse_from_str(&full_date, "%d.%m.%Y").unwrap();

            let difference: TimeDelta = full_date_format - utc_date;
            let days_away: i64 = difference.num_days();
            

            if days_away > 0{
                println!("{:?}, {:?}", days_away, values)
            }
                
            
        }
        
}