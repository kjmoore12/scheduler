# Project 3 Scheduler Recreated Using Rust
### By Kenneth Moore

This program is formatted to function exactly as my Project 3 program that was written in C++ does. It is able to perform the four algorithms: First-In First-Out (FIFO), Shortest Job First (SJF), Shortest Time To Complete First (STCF), and Round Robin (RR).
The code itself is split across 3 .rs files, in an attempt to mimic the structure from the C++ project. However, header files are not used, so the structs are defined directly in a .rs file.

## main.rs

This is where the program enters. It begins by collecting the user's args and ensuring the arg length is correct, then properly sets each argument to a variable. Following this it calls methods from **scheduling.rs** to create a workload binary heap from the workload file, then select and run the correct algorithm based on the user's arguments.

## process_structs.rs

This is where the structures used are defined. There are two structures used, **ArrivalSortedProcess** and **DurationSortedProcess**. These two structs contain the same fields, both mimicking the Process struct from the C++ variant of the project. The reason there are two is that custom ordering functionality is defined for each process, so that when they are used in a binary heap, they can be correctly ordered. **ArrivalSortedProcess** is defined to be sorted by its arrival, allowing a binary heap to select first the processes with the lowest arrival time. **DurationSortedProcess** is defined to be sorted by its duration field, allowing a binary heap to select first the processes with the shortest duration.

## scheduling.rs

This is where all the algorithm, reading, and printing methods exist. The file format follows the same format as my C++ variant. The included methods are:
### read_workload
This method takes in a String reference for a filename. It then opens the file, and reads each line, using the space as a delimiter to isolate out the arrival and duration times. It then creates an **ArrivalSortedProcess** with this data and adds it to a binary heap. Once all lines are parsed to create an **ArrivalSortedProcess**, a Result is returned containing the binary heap if successful.
### show_workload
This method takes in a binary heap of **ArrivalSortedProcess** and prints each one's arrival and duration time.
### show_processes
This method takes in a vector reference of **ArrivalSortedProcess** and prints all fields from each one.
### fifo
This method takes in a mutable binary heap of **ArrivalSortedProcess** and returns a vector of **ArrivalSortedProcess**. The vector stores the processes in order of completion (ordered using First-In First-Out). It is populated by iterating through each processes from the heap (which is already sorted by arrival time). It then runs the process, increasing the time to reflect the duration the task process, and updating the fields of the process.
### sjf
This method takes in a mutable binary heap of **ArrivalSortedProcess** and returns a vector of **ArrivalSortedProcess**. The vector stores the processes in order of completion (ordered using Shortest Job First). It does this by looping continously. In an innner loop, all processes that have already arrived at the time are converted to **DurationSortedProcess** and added to a binary heap. The inner loop terminates when no more processes have arrived at the time. A second inner loop runs, which selects the shortest task (top of the binary heap), runs it, and completes its fields before adding it to the complete vector. Time is increased to reflect the run. If another task has arrived, the inner loop ends and the outer loop repeats. Otherwise, this inner loop runs until all tasks have completed, then breaks returns the complete vector. 
### stcf
This method takes in a mutable binary heap of **ArrivalSortedProcess** and returns a vector of **ArrivalSortedProcess**. The vector stores the processes in order of completion (ordered using Shortest Time to Complete First). It does this by looping continously. In an innner loop, all processes that have already arrived at the time are converted to **DurationSortedProcess** and added to a binary heap. The first run field is set to -1 (to indicate the process has yet to run), and the top.completion field is set to top.duration (to store this value). The inner loop terminates when no more processes have arrived at the time. A second inner loop runs by selecting the process with the shortest time to complete (taking the top of the binary heap sorted by duration). However, it will preempt running tasks if another processs arrives during its execution. In this case, the process that was running's duration is update to reflect the time remaining, then it is added back to the binary heap before the second loop breaks and the outer loop repeats. Once a process successfully completes, its duration is set to be its completion (which was set to its duration originally to retain the original value), its completion is updated, and its added to the complete vector. Once all processes have run the complete vector is returned.
### rr
This method takes in a mutable binary heap of **ArrivalSortedProcess** and returns a vector of **ArrivalSortedProcess**. The vector stores the processes in order of completion (ordered using Round Robin). It does this by looping continously in an outer loop. In the first inner loop, any processes that has arrived at the time is added to VecDeque. Once again the first_run value is set to -1 and the completion value is set to the duration of the process at this stage. Once all processes that have arrived are added to this VecDeque, the first inner loop breaks and the second inner loop begins. This loop iterates through the VecDeque, popping the process from the front. If no other task has arrived, the process is ran for 1 time slot and its fields are updated accordingly. If the process is complete it is pushed to complete, otherwise it is pushed to the back of the VecDeque. Once the VecDeque is empty, the complete vector is returned.
