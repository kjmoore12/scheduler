// Including process struts that allow for use in binary heaps
mod process_structs;
use crate::process_structs::ArrivalSortedProcess;
use crate::process_structs::DurationSortedProcess;
mod scheduling;
use std::collections::BinaryHeap;
use std::env;
use std::process;
fn main() {
    // Take in command line arguments by the user
    let args: Vec<String> = env::args().collect();
    if args.len()!=3{
        println!("usage: [fifo|sjf|stcf|rr] workload_file");
        process::exit(1);
    }
    let algorithm = &args[1];
    let workload_file = &args[2];
    let mut workload = scheduling::read_workload(workload_file);
    match workload{
        Ok(heap) => {
            scheduling::show_workload(heap.clone());
            if algorithm.eq("fifo"){
                scheduling::show_metrics(&scheduling::fifo(heap));
            } else if algorithm.eq("sjf"){
                scheduling::show_metrics(&scheduling::sjf(heap));
            } else if algorithm.eq("stcf"){
                scheduling::show_metrics(&scheduling::stcf(heap));
            } else if algorithm.eq("rr"){
                scheduling::show_metrics(&scheduling::rr(heap));
            } else{ // Non-valid algorithm given
                println!("Error: Unknown algorithm: {}", algorithm);
                println!("usage: [fifo|sjf|stcf|rr] workload_file");
                process::exit(1);
            }
        }
        Err(e) => { // An error is caused by an invalidly formatted workload file
            println!("Workload file caused error. Format each line as: [arrival_value] [duration_value]");
            println!("usage: [fifo|sjf|stcf|rr] workload_file");
            process::exit(1);
        }
    }
}