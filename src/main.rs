use std::{collections::HashMap};
use chrono::{NaiveDate, TimeDelta, Utc};
use std::fs;
use serde_json;
use windows::core::w;
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_OK, MB_ICONINFORMATION};
use windows::core::PCWSTR;

fn main() {
    // counddown will begin from 01.01 as day 1, and will go till 31.12(28-29.12) as 365th
    

    let utc_date = Utc::now().date_naive();

    // getting path to file
    let content = fs::read_to_string("dates.json").unwrap();

    // reading file(json)
    let config: HashMap<String, String> = serde_json::from_str(&content).unwrap();

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
                let message = format!("{} has birthday in {} days at {}", values, days_away, keys); // 

                let wide_text: Vec<u16> = message.encode_utf16().chain(std::iter::once(0)).collect(); // UTF-8 (Rust)  →  UTF-16 codes  →  + null byte  →  Vec<u16>  →  Windows reads it


                unsafe { // telling rust that i know what im doing
                    MessageBoxW(
                        None,                    // no parent window
                        PCWSTR(wide_text.as_ptr()),     // the text
                        w!("Birthday Reminder"), // the title bar
                        MB_OK | MB_ICONINFORMATION, // OK button + info icon
                    );
                }
                break;
            }
        }
        
    }
        
}
