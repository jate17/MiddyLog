use crate::parser;


/*
use std::collections::HashMap;

const ackcode: HashMap<&str, &str> = HashMap::from([
    ("AA", "Application Accept"),
    ("AE", "Application Error"),
    ("AR", "Application Reject")
]);
*/

fn split_hl7(line: &str) -> Vec<&str> {
    line.split('|').collect()
}

pub fn msa(blocks: &Vec<String>) -> Vec<String> {
    let mut msa: Vec<String> = Vec::new();
    for line in blocks {
        let t = parser::extract_msa(line).unwrap_or_default();
        if !t.is_empty(){
            msa.push(t)
        }
    }
    msa
}


pub fn find_AE(blocks: &Vec<String>) -> Vec<String>{
    
    blocks.iter()
        .filter(|s| {
            let parts: Vec<&str> = s.split("|").collect();
            parts.get(1).map_or(false, |p| p.contains("AE"))
        })  
        .cloned()
        .collect()

}


pub fn extract_control_id_from_pid(blocks: &Vec<String>) -> Vec<String>{
    let mut cids: Vec<String> = Vec::new(); 
    for line in blocks {
        let msh = parser::extract_msh(&line).unwrap_or_default();
        if !msh.is_empty(){
            let msh_collect: Vec<_> = msh.split("|").collect();
            let control_id = msh_collect[9];
            if !control_id.is_empty(){
                cids.push(control_id.to_string());
            }
        }
    }
    cids 
}

pub fn find_control_id_form_msa(blocks: &Vec<String>, control_id: &Vec<String>) -> Vec<String> {
    let mut msa: Vec<String> = Vec::new();
    for line in blocks {
        let t = parser::extract_msa(line).unwrap_or_default();
        if !t.is_empty() {
            let msa_collect: Vec<_> = t.split("|").collect();  // t, non msa
            if msa_collect.len() > 2 {                         // bounds check
                let control_id_msa = msa_collect[2];
                if control_id.iter().any(|s| s == control_id_msa) {
                    msa.push(line.to_string());
                }
            }
        }
    }
    msa
}

pub fn merge_blockpid_and_blockack(blockack: &Vec<String>, blockpid: &Vec<String>) -> Vec<String> {
    let mut combined: Vec<String> = blockack.iter().chain(blockpid.iter()).cloned().collect();
    combined.sort_by(|a, b| a[..23].cmp(&b[..23]));
    combined
}