use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
    env,
};

use std::time::Instant;


fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn elapsed_ms(t1: Instant, t2: Instant) -> f64 {
    let t = t2 - t1;
    t.as_secs() as f64 * 1000. + t.subsec_nanos() as f64 / 1e6 
}

fn main() {
    println!("Repeated Lines Remover is starting");
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    
    
    let t1 = Instant::now();
    let mut lines = lines_from_file(file_name).expect("Could not load lines");
    
    lines.sort();

    let length_of_lines = lines.len() -1; 
    let mut cloned_lines = lines.clone();

    let mut start_index = 0;
    let mut length = 0;
    let mut current_common_string = "";
    let mut removed_count = 0;

    for (index, line) in lines.iter().enumerate() {
        let email_in_line = line.split(":").next().unwrap();
        
        if current_common_string != email_in_line || index == length_of_lines {
            if length >= 1 || index == length_of_lines  {
                let si =  start_index-removed_count;
                let mut li = start_index+length+1-removed_count;
                if index == length_of_lines{
                    li = li +1
                }
               cloned_lines.drain(si..li);
                removed_count = removed_count + length + 1;
            }
            start_index = index;
            length = 0;
        }    

        if current_common_string == email_in_line {
            length = length + 1
        }

       if current_common_string != email_in_line{
           current_common_string = email_in_line;
       }
    }

    let file_content = cloned_lines.iter().map(|x| format!("{}\n", x)).collect::<Vec<_>>();;

    let t2= Instant::now();
    std::fs::write(file_name, file_content.join("")).expect("Unable to write file");
    println!("Benchmark::: {} ms",elapsed_ms(t1,t2))    
}
