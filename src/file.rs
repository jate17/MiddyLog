


use std::fs;


pub fn readlog(filename: &String) -> Result<Vec<String>, std::io::Error> {
    println!("[*] Read Log: {}", filename);
    let file = fs::read_to_string(filename)?;

    let blocks: Vec<String> = file
        .split('\n')
        .map(|line| {
            // Se la riga contiene HL7, espandi i segmenti \r in \n
            if let Some(pos) = line.find("MSH|") {
                let prefix = &line[..pos];
                let hl7_part = &line[pos..];
                let segments: Vec<&str> = hl7_part
                    .split('\r')
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .collect();
                format!("{}{}", prefix, segments.join("\n"))
            } else {
                line.trim_end_matches('\r').to_string()
            }
        })
        .filter(|line| !line.is_empty())
        .collect();

    Ok(blocks)
}