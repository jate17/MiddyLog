use crate::parser;


/*Count disconnected from log file*/
pub fn count_disconnected(blocks: &Vec<String>) -> usize {
    let mut counter = 0;
    for line in blocks {
        let f = parser::extract_connection_status(line).unwrap_or_default();
        if f == "Disconnected" {
            counter += 1;
        }
    }
    counter
}


/*
Find all ip and port and type of workflow 

*/


pub fn find_ip_channels(blocks: &Vec<String>, workflow: i32) -> Vec<Vec<String>> {
    let mut channels: Vec<Vec<String>> = Vec::new();
    
    for line in blocks {
        let f = parser::extract_connection_status(&line).unwrap_or_default();
        if f.contains("Connected to") {
            let wf = parser::extract_type_workflow(&line).unwrap_or_default();
            let parts: Vec<&str> = f.split(' ').collect();
            if let Some(ip) = parts.get(2) {
                let new_row = vec![ip.to_string(), wf];
                if !channels.iter().any(|row| row == &new_row) {
                    channels.push(new_row);
                }
            }
        }
    }
    channels
}

