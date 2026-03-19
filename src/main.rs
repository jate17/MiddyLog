mod file;
mod parser;
mod connection_utils;
mod hl7_utils;
mod func;



fn main() {
   let blocks: Vec<String> = file::readlog(&"./Gateway-log.txt".to_string()).unwrap();

   let mut data: Vec<String> = Vec::new();
   let d = parser::extract_msh(&blocks[400]).unwrap_or_default();
   let l: Vec<_> = d.split("|").collect();
   println!("{:?}", l[8]);
   if l[8] == "ORM^001".to_string() {
      println!("{:?}", d);
      data.push(d.to_string())
   }

   println!("{:?}", data);



    

} 


