use crate::hl7_utils;
use crate::parser;


//func::trace_patient(&blocks,"6261959745".to_string());
pub fn trace_patient(blocks: &Vec<String>, pid: String) {

    let blockpid = parser::extract_blocks_by_pid(&blocks, &pid);
    let extractack = hl7_utils::extract_control_id_from_pid(&blockpid);
    let blockack = hl7_utils::find_control_id_form_msa(&blocks, &extractack);
    let history = hl7_utils::merge_blockpid_and_blockack(&blockpid, &blockack);
    for b in &history {


        let msh = parser::extract_msh(&b).unwrap_or_default();
        if !msh.is_empty(){
            println!("\n-----------\n");
            let workflow = parser::extract_type_workflow(&b).unwrap_or_default();
            if !workflow.is_empty(){
                println!("Workflow: {:?}", workflow);
            }
            let data = parser::extract_date(&b).unwrap_or_default();
            if !data.is_empty() {
                println!("Date: {:?}", data)
            }
            let msh_collect: Vec<_> = msh.split("|").collect();
            let control_id = msh_collect[9];
            let from = msh_collect[2];
            let to = msh_collect[4];
            let types = &msh_collect[8][..3];
            println!("From: {:?}\nTo: {:?}\nType: {:?}\nControl ID: {:?}", from, to, types, control_id);
            if types == "QRY"{
                let qrd = parser::extract_qrd(&b).unwrap_or_default();
                if !qrd.is_empty(){
                        let l: Vec<_> = qrd.split("|").collect();
                        let pid = l[8];
                        println!("Request data for PID: {:?} from {:?}", pid, from);

                }


            }else if types == "ADR"{
                let pidex = parser::extract_pid(&b).unwrap_or_default();
                let err =  parser::extract_err(&b).unwrap_or_default();
                if !pidex.is_empty(){
                        let l: Vec<_> = pidex.split("|").collect();
                        let pid = l[3];
                        println!("Response data for PID: {:?} from {:?}", pid, from);

                }
                if !err.is_empty() {
                    let l: Vec<_> = err.split("|").collect();
                    let err: Vec<_> = l[1].split("^").collect();
                    println!("Error: {:?}", err[3]);
                }

            }else if types == "ORU" {
                let obx = parser::extract_obx(&b).unwrap_or_default();
                let pidex = parser::extract_pid(&b).unwrap_or_default();
                if !obx.is_empty(){
                        let l: Vec<_> = obx.split("|").collect();
                        let poct = &l[18].replace("^"," ")[2..];
                        println!("POCT: {:?}", poct);
                        if !pidex.is_empty() {
                            let l: Vec<_> = pidex.split("|").collect();
                            let pid = l[3];
                            println!("Push data for PID: {:?} to {:?}", pid, to);
                        }

                }
            }else if types == "ACK" {
                let msa = parser::extract_msa(&b).unwrap_or_default();
                if !msa.is_empty() {
                    let l: Vec<_> = msa.split("|").collect();
                    let ackcode = l[1];
                    let id = l[2];
                    let error = l[3];
                    println!("Ack Code: {:?}\nID:{:?}\nError: {:?}",ackcode,id, error )
                }
            }else if types == "ORM" {
                let orc = parser::extract_orc(&b).unwrap_or_default();
                let pidex = parser::extract_pid(&b).unwrap_or_default();
                if !orc.is_empty() {
                    let l: Vec<_> = orc.split("|").collect();
                    let orderid = l[2];
                    println!("Order ID: {:?} from {:?}", orderid, from);
                    if !pidex.is_empty() {
                        let l: Vec<_> = pidex.split("|").collect();
                        let pid = l[3];
                        println!("PID: {:?}", pid);
                    }
                }
            }
            println!("\n-----------\n");
        }
    }
}
