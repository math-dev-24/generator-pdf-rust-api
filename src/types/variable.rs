
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