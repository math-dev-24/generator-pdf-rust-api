use regex::{Match, Regex};
use crate::types::variable::{Segment, SegmentKind};


pub fn get_segments(html_doc: &str) -> Vec<Segment> {
    let mut segments: Vec<Segment> = Vec::new();
    let re_segment = Regex::new(r"\{\{(#?/?)(.*?)\}\}").unwrap();

    for cap in re_segment.captures_iter(html_doc) {
        if let Some(s) = cap.get(0) {
            let raw_key = cap.get(2).unwrap().as_str().trim().to_string();
            let prefix = cap.get(1).unwrap().as_str();
            let variable = raw_key.split_whitespace().next().unwrap().to_string();

            if prefix.is_empty() {
                let tmp_var = Segment {
                    start: s.start(), end: s.end(), key: variable,
                    kind: SegmentKind::Variable, children: Vec::new(), len_in: 0, len_out: 0,
                };
                create_sub_segment_or_add(&mut segments, tmp_var);
            } else {
                let key: Vec<&str> = raw_key.split_whitespace().collect();

                if raw_key.starts_with("if") || raw_key.starts_with("loop") {
                    let kind: SegmentKind = if raw_key.starts_with("if") {
                        SegmentKind::Condition
                    }else {
                        SegmentKind::Loop
                    };
                    let tmp_segment = Segment {
                        start: s.start(), end: 0,
                        key: key[1].to_string(),
                        kind,
                        children: Vec::new(), len_in: s.end() - s.start(), len_out: 0
                    };
                    create_sub_segment_or_add(&mut segments, tmp_segment);

                } else if raw_key.starts_with("endif") || raw_key.starts_with("endloop") {

                    let kind: SegmentKind = if raw_key.starts_with("endif") {
                        SegmentKind::Condition
                    }else {
                        SegmentKind::Loop
                    };
                    end_sub(&mut segments, s, kind);
                }
            }
        }
    }
    segments
}


fn create_sub_segment_or_add(segments: &mut Vec<Segment>, segment: Segment) {
    if let Some(index) = segments.iter().position(|s| s.end == 0) {
            create_sub_segment_or_add(&mut segments[index].children, segment.clone());
    } else {
        segments.push(segment);
    }
}

fn end_sub(segments: &mut Vec<Segment>, s: Match, kind: SegmentKind) {
    if let Some(index) = segments.iter().position(|seg| seg.end == 0 && seg.kind == kind) {
        let segment = &mut segments[index];

        if !update_children(segment, s) {
            segment.end = s.end();
            segment.len_out = s.end() - s.start();
        }
    }
}

fn update_children(segment: &mut Segment, s: Match) -> bool {
    let mut child_updated = false;

    for child in &mut segment.children {
        match child.kind {
            SegmentKind::Condition if child.end == 0 => {
                child.end = s.end();
                child.len_out = s.end() - s.start();
                child_updated = true;
            }
            _ => {
                child_updated |= update_children(child, s);
            }
        }
    }
    child_updated
}