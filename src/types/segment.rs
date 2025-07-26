use std::ops::Range;

#[derive(Clone, Debug, PartialEq)]
pub struct Segment {
    pub start: usize,
    pub end: usize,
    pub len_in: usize,
    pub len_out: usize,
    pub key: String,
    pub kind: SegmentKind,
    pub children: Vec<Segment>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SegmentKind {
    Variable,
    Condition,
    Loop
}

impl Segment {
    pub fn get_range_tag(&self, in_out: &str) -> Option<Range<usize>> {
        if let SegmentKind::Variable = self.kind {
            None
        }else {
            if in_out == "out" {
                Some(self.end - self.len_out..self.end)
            } else {
                Some(self.start..self.start + self.len_in)
            }
        }
    }
    fn get_range_content(&self) -> Option<Range<usize>> {
        if let SegmentKind::Loop = self.kind {
            Some(self.start + self.len_in..self.end - self.len_out)
        } else {
            None
        }
    }

    pub fn get_content_loop(&self, content: &String) -> Option<String> {
        if let Some(range) = self.get_range_content() {
            return Some(content[range].to_string());
        }
        None
    }
}