use std::collections::HashSet;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Range {
    pub source_start: u64,
    pub source_end: u64,
    pub destination_start: u64,
}

impl Range {
    pub fn new(source_start: u64, source_end: u64, destination_start: u64) -> Range {
        if source_start > source_end {
            panic!("WTF M8");
        }
        Range {
            source_start,
            source_end,
            destination_start,
        }
    }

    pub fn get_destination(&self, value: u64) -> Option<u64> {
        if self.source_start <= value && value <= self.source_end {
            return Some(self.destination_start + (value - self.source_start));
        }
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SeedRange {
    pub start: u64,
    pub end: u64,
}

impl SeedRange {
    pub fn new(start: u64, end: u64) -> SeedRange {
        if start > end {
            panic!("WTF M8");
        }
        SeedRange { start, end }
    }

    pub fn get_lenght(&self) -> u64 {
        return self.end - self.start;
    }
}

#[derive(Debug, Clone)]
pub struct RangeList {
    ranges: Vec<Range>,
}

impl RangeList {
    pub fn new() -> RangeList {
        RangeList { ranges: vec![] }
    }

    pub fn add_range(&mut self, range: Range) {
        self.ranges.push(range);
    }

    pub fn get_destination(&self, value: u64) -> u64 {
        for range in self.ranges.iter() {
            if let Some(destination) = range.get_destination(value) {
                return destination;
            }
        }

        value
    }
}
