use crate::process_structs::ArrivalSortedProcess;
use crate::process_structs::DurationSortedProcess;
use std::collections::BinaryHeap;
use std::collections::VecDeque;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::env;
use std::process;
use std::cmp;

/// Generates a BinaryHeap of ArrivalSortedProcess from the workload file
pub fn read_workload(filename: &String) -> std::io::Result<BinaryHeap<ArrivalSortedProcess>>{
    // Open the file
    let workload_file = File::open(filename)?;
    let reader = BufReader::new(workload_file);
    let mut workload = BinaryHeap::new();

    // Read and process each line of the file
    for line in reader.lines(){
        let line = line?;
        let split = line.split(" ");
        let parts = split.collect::<Vec<&str>>();
        // Skip any lines that don't consist of two strings seperated by a space
        if parts.len() != 2{
            continue;
        }

        // Create the process to be pushed to the binary heap
        let mut process = ArrivalSortedProcess{
            arrival : 0,
            first_run : 0,
            duration : 0,
            completion : 0
        };
        
        // Parse out the arrival and duration times and update process' fields accordingly
        match parts[0].trim().parse::<i32>(){
            Ok(arrival_result) => {
                process.arrival = arrival_result;
            }
            Err(e) => {
                e;
            }
        }
        match parts[1].trim().parse::<i32>(){
            Ok(duration_result) => {
                process.duration = duration_result;
            }
            Err(e) => {
                e;
            }
        }
        workload.push(process);
    }
    Ok(workload)
}

/// Prints the arrival and duration time of each process in a workload
pub fn show_workload(workload: BinaryHeap<ArrivalSortedProcess>){
    // Iterate through workload, printing each process' arrival and duration times
    println!("Workload:");
    let some_ordered_vector = workload.into_sorted_vec();
    for process in some_ordered_vector{
        println!("   {} {}", process.arrival, process.duration);
    }
}

/// Prints the process information for each process in a vector
pub fn show_processes(processes: &Vec<ArrivalSortedProcess>){
    println!("Processes:");
    for process in processes{
        println!("  arrival={}, duration={}, first_run={}, completion={}", process.arrival, process.duration, process.first_run, process.completion);
    }
}

/// Performs FIFO algorithm on a workload
pub fn fifo(mut workload: BinaryHeap<ArrivalSortedProcess>) -> Vec<ArrivalSortedProcess>{
    let mut complete = Vec::new();
    let mut time : i32 = 0;

    // Keep time advancing while there are still tasks to do
    loop {
        // At the next available time, update the process running if there is a process thats arrived
        match workload.peek(){
            Some(top) => {
                if top.arrival > time{
                    time = top.arrival;
                }
                // Create a new process to prevent issues with ownership (more elegant solution available?)
                let mut process = ArrivalSortedProcess{
                    arrival: top.arrival,
                    duration: top.duration,
                    first_run: time,
                    completion: top.duration + time
                };
                time += process.duration;
                complete.push(process);
                workload.pop();
            }
            None => {
                break;
            }
        }
    }

    complete
}

/// Performs SJF algorithm on a workload
pub fn sjf(mut workload: BinaryHeap<ArrivalSortedProcess>) -> Vec<ArrivalSortedProcess>{
    let mut complete = Vec::new();
    let mut arrived_tasks : BinaryHeap<DurationSortedProcess> = BinaryHeap::new();
    let mut time : i32 = 0;
    let mut next_arrival : i32 = 0;
    loop {
        // Add all tasks that have arrived at the given time to arrived_tasks
        loop {
            match workload.peek(){
                Some(top) => {
                    if time >= top.arrival {
                        let mut process = DurationSortedProcess{arrival: top.arrival, duration: top.duration, first_run: 0, completion: 0};
                        arrived_tasks.push(process);
                        workload.pop();
                        
                    }
                    else {
                        next_arrival = top.arrival;
                        break;
                    }
                }
                None => {
                    // Use next_arrival to indicate that all tasks have completed
                    next_arrival = -1;
                    break;
                }
            }
        }
        // Complete the shortest task first until all are complete or a new tasks arrives. Add completed tasks to complete
        loop {
            match arrived_tasks.peek(){
                Some(top) => {
                    // Runs until a new task arrives
                    if next_arrival == -1 || time <= next_arrival{
                        // Update task info
                        let mut process = ArrivalSortedProcess{ arrival: top.arrival, duration: top.duration, first_run: time, completion: top.duration + time};
                        time += top.duration;
                        arrived_tasks.pop();
                        complete.push(process);
                    }
                    else{
                        time = next_arrival;
                        break;
                    }
                }
                None => {
                    // If all tasks have arrived and completed, return the complete vector
                    if next_arrival == -1 {
                        return complete;
                    }
                    // Update time to the arrival of the next task to arrive
                    time = next_arrival;
                    break;
                }
            }
        }
    }
}

/// Performs STCF algorithm on a workload
pub fn stcf(mut workload: BinaryHeap<ArrivalSortedProcess>) -> Vec<ArrivalSortedProcess>{
    let mut complete = Vec::new();
    let mut arrived_tasks : BinaryHeap<DurationSortedProcess> = BinaryHeap::new();
    let mut time : i32 = 0;
    let mut next_arrival : i32 = 0;
    loop{
        // Add all tasks that have arrived at the given time to arrived_tasks
        loop{
            match workload.peek(){
                Some(top) => {
                    if time >= top.arrival{
                        let mut process = DurationSortedProcess{ arrival: top.arrival, duration: top.duration, first_run: -1, completion: top.duration};
                        arrived_tasks.push(process);
                        workload.pop();
                    }
                    else{
                        next_arrival = top.arrival;
                        break;
                    }
                }
                None => {
                    next_arrival = -1;
                    break;
                }
            }
        }
        // Run the tasks with the shortest time to complete from the list of arrived tasks until a new task arrives or all complete
        loop {
            match arrived_tasks.pop() {
                Some(mut top) => {
                    // Runs until a new task arrives
                    if next_arrival == -1 || next_arrival - time > top.duration {
                        // Update task info
                        let mut process = ArrivalSortedProcess{ arrival: top.arrival, duration: top.duration, first_run: time, completion: top.duration + time};
                        // If the task has ran before, update its information to correctly reflect that
                        if top.first_run != -1 {
                            process.first_run = top.first_run;
                            // Duration has been temporarly stored in the completion field in this case, as duration is updated when task is preempted
                            process.duration = top.completion;
                        }
                        time += top.duration;
                        complete.push(process);
                    }
                    // If a new task arrives during execution, preempt execution
                    else{
                        // On the first run, update its first run time
                        if top.first_run == -1 {
                            top.first_run = time;
                        }
                        // Update duration to reflect time remaining
                        top.duration = top.duration - (next_arrival - time);
                        time = next_arrival;
                        arrived_tasks.push(top);
                        break;
                    }
                }
                None => {
                    // If all tasks have arrived and completed, return the complete vector
                    if next_arrival == -1 {
                        return complete;
                    }
                    // Update time to the arrival of the next task to arrive
                    time = next_arrival;
                    break;
                }
            }
        }
    }
}

/// Performs RR algorithm on a workload
pub fn rr(mut workload: BinaryHeap<ArrivalSortedProcess>) -> Vec<ArrivalSortedProcess>{
    let mut complete = Vec::new();
    let mut arrived_tasks = VecDeque::new();
    let mut time : i32 = 0;
    let mut next_arrival : i32 = 0;
    loop {
        // Add all tasks that have arrived at the given time to arrived_tasks
        loop{
            match workload.peek(){
                Some(top) => {
                    if time >= top.arrival{
                        let mut process = ArrivalSortedProcess{ arrival: top.arrival, duration: top.duration, first_run: -1, completion: top.duration};
                        arrived_tasks.push_back(process);
                        workload.pop();
                    }
                    else{
                        next_arrival = top.arrival;
                        break;
                    }
                }
                None => {
                    next_arrival = -1;
                    break;
                }
            }
        }

        loop{
            match arrived_tasks.pop_front(){
                Some(mut front) => {
                    if next_arrival == -1 || time < next_arrival {
                        if front.first_run == -1 {
                            front.first_run = time;
                            // Handles the case where a task that takes 0 time units was given
                            if front.duration == 0 {
                                front.duration = front.completion;
                                front.completion = time;
                                complete.push(front);
                                continue;
                            }
                        }
                        front.duration -= 1;
                        time += 1;
                        if front.duration == 0 {
                            front.duration = front.completion;
                            front.completion = time;
                            complete.push(front);
                            continue;
                        }
                        else{
                            arrived_tasks.push_back(front);
                        }
                    }
                    else{
                        arrived_tasks.push_front(front);
                        break;
                    }
                }
                None => {
                    // If all tasks have arrived and completed, return the complete vector
                    if next_arrival == -1 {
                        return complete;
                    }
                    // Update time to the arrival of the next task to arrive
                    time = next_arrival;
                    break;
                }
            }
        }
    }
}

fn avg_turnaround(processes: &Vec<ArrivalSortedProcess>) -> f32 {
    let mut total_turnaround : f32 = 0.0;
    let mut size : f32 = processes.len() as f32;
    for process in processes {
        total_turnaround += (process.completion - process.arrival) as f32;
    }
    total_turnaround/size
}

fn avg_response(processes: &Vec<ArrivalSortedProcess>) -> f32 {
    let mut total_response : f32 = 0.0;
    let mut size : f32 = processes.len() as f32;
    for process in processes {
        total_response += (process.first_run - process.arrival) as f32;
    }
    total_response/size
}

pub fn show_metrics(processes: &Vec<ArrivalSortedProcess>){
    let mut avg_t : f32 = avg_turnaround(processes);
    let mut avg_r : f32 = avg_response(processes);
    show_processes(processes);
    println!("\nAverage Turnaround Time: {}\nAverage Response Time:   {}", avg_t, avg_r);
}