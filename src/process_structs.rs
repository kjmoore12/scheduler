use std::cmp::Ordering;
// Struct that holds a process, with custom ordering properties to order by arrival time

#[derive(Eq)]
pub struct ArrivalSortedProcess {
    pub arrival: i32,
    pub first_run: i32,
    pub duration: i32,
    pub completion: i32
}

// Set up ordering behavior

impl Ord for ArrivalSortedProcess {
    fn cmp(&self, other: &Self) -> Ordering {
        other.arrival.cmp(&self.arrival)
    }
}

impl PartialOrd for ArrivalSortedProcess {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ArrivalSortedProcess {
    fn eq(&self, other: &Self) -> bool {
        self.arrival == other.arrival
    }
}

// Implement ability to clone

impl Clone for ArrivalSortedProcess {
    fn clone(&self) -> Self {
        ArrivalSortedProcess {
            arrival: self.arrival,
            first_run: self.first_run,
            duration: self.duration,
            completion: self.completion
        }
    }
}

// Struct that holds a process, with custom ordering properties to order by duration time

#[derive(Eq)]
pub struct DurationSortedProcess {
    pub arrival: i32,
    pub first_run: i32,
    pub duration: i32,
    pub completion: i32
}

impl Ord for DurationSortedProcess {
    fn cmp(&self, other: &Self) -> Ordering {
        other.duration.cmp(&self.duration)
    }
}

impl PartialOrd for DurationSortedProcess {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for DurationSortedProcess {
    fn eq(&self, other: &Self) -> bool {
        self.duration == other.duration
    }
}