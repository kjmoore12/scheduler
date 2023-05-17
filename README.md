# Project 3 Scheduler Recreated Using Rust
### By Kenneth Moore

This program is formatted to function exactly as my Project 3 program that was written in C++ does. It is able to perform the four algorithms: First-In First-Out (FIFO), Shortest Job First (SJF), Shortest Time To Complete First (STCF), and Round Robin (RR).
The code itself is split across 3 .rs files, in an attempt to mimic the structure from the C++ project. However, header files are not used, so the structs are defined directly in a .rs file.

## main.rs

This is where the program enters. It begins by collecting the user's args and ensuring the arg length is correct, then properly sets each argument to a variable. Following this it calls methods from **scheduling.rs** to create a workload binary heap from the workload file, then select and run the correct algorithm based on the user's arguments.

## process_structs.rs

This is where the structures used are defined. There are two structures used, **ArrivalSortedProcess** and **DurationSortedProcess**. These two structs contain the same fields, both mimicking the Process struct from the C++ variant of the project. The reason there are two is that custom ordering functionality is defined for each process, so that when they are used in a binary heap, they can be correctly ordered. **ArrivalSortedProcess** is defined to be sorted by its arrival, allowing a binary heap to select first the processes with the lowest arrival time. **DurationSortedProcess** is defined to be sorted by its duration field, allowing a binary heap to select first the processes with the shortest duration.
