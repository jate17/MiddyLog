use regex::Regex;

/*Data*/
pub fn extract_date(line: &str) -> Option<String> {
    let re = Regex::new(r"\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2},\d{3}").unwrap();
    re.find(line).map(|m| m.as_str().to_string())
}

/*Log Type: ERROR, DEBUG , INFO, WARN*/
pub fn extract_type_log(line: &str) -> Option<String> {
    let re = Regex::new(r"[A-Z][A-Z][A-Z][A-Z][A-Z]").unwrap();
    re.find(line).map(|m| m.as_str().to_string())
}


/*
Workflow type:
    - PatientQuery
    - ResultOut

*/
pub fn extract_type_workflow(line: &str) -> Option<String> {
    let re = Regex::new(r"[A-Z][a-z]+[A-Z][a-z]+\(([^)]+)\)").unwrap();
    re.find(line).map(|m| {
        let str_find = m.as_str().to_string();
        let types: Vec<_> = str_find.split("(").collect();
        types[0].to_string()
    })
}


/*
Error from the middleware same like RapidComm, POCCeleretor

*/
pub fn extract_error(line: &str) -> Option<String> {
    let re = Regex::new(r"- ([A-Z][a-zA-Z0-9_]+\([^)]*\)[^\-]*)$").unwrap();
    re.captures(line)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().trim().to_string())
}
/*
Connection status from middleware

*/
pub fn extract_connection_status(line: &str) -> Option<String> {
    let re = Regex::new(r"(Disconnected|Connection error:|Connected to).*").unwrap();
    re.find(line).map(|m| m.as_str().trim().to_string())
}




/*

Grep HL7 

*/

pub fn extract_msh(line: &str) -> Option<String> {
    let re = Regex::new(r"[A-Z][A-z][A-Z]\|.*").unwrap();
    re.find(line).map(|m| m.as_str().trim().to_string())
}

pub fn extract_msa(line: &str) -> Option<String> {
    let re = Regex::new(r"MSA\|.*").unwrap();
    re.find(line).map(|m| m.as_str().trim().to_string())
}
pub fn extract_qrd(line: &str) -> Option<String> {
    let re = Regex::new(r"QRD\|.*").unwrap();
    re.find(line).map(|m| m.as_str().trim().to_string())
}

pub fn extract_pid(line: &str) -> Option<String> {
    let re = Regex::new(r"PID\|.*").unwrap();
    re.find(line).map(|m| m.as_str().trim().to_string())
}

pub fn extract_obx(line: &str) -> Option<String> {
    let re = Regex::new(r"OBX\|.*").unwrap();
    re.find(line).map(|m| m.as_str().trim().to_string())
}

pub fn extract_orc(line: &str) -> Option<String> {
    let re = Regex::new(r"ORC\|.*").unwrap();
    re.find(line).map(|m| m.as_str().trim().to_string())
}

pub fn extract_err(line: &str) -> Option<String> {
    let re = Regex::new(r"ERR\|.*").unwrap();
    re.find(line).map(|m| m.as_str().trim().to_string())
}
/*Migliorare il mapping*/
pub fn extract_message(line: &str) -> Option<String> {
    let parts: Vec<&str> = line.split('-').collect();
    let re = Regex::new(r".*[.:]$").unwrap();
    re.find(parts[4]).map(|m| m.as_str().trim().to_string())
}

/*

Estract PID 

*/
/*

pub fn extract_blocks_by_pid(blocks: &Vec<String>, patient_id: &str) -> Vec<String> {
    blocks
        .iter()
        .filter(|block| block.contains(patient_id))
        .cloned()
        .collect()
}


    - 


*/


pub fn extract_blocks_by_pid(blocks: &[String], patient_id: &str) -> Vec<String> {
    blocks
        .iter()
        .filter(|block| contains_exact_pid(block, patient_id))
        .cloned()
        .collect()
}

fn contains_exact_pid(block: &str, patient_id: &str) -> bool {
    block.lines().any(|line| {
        // Cerca il patient_id come parola intera nella riga
        line.split(|c: char| !c.is_alphanumeric())
            .any(|token| token == patient_id)
    })
}
