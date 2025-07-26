use serde_json::Value;
use crate::types::segment::{Segment, SegmentKind};
use crate::utils::variables::get_segments;

pub fn replace_variable(
    html_content: String,
    segments: Vec<Segment>,
    values: &Value
) -> String {
    let mut html_content = html_content;
    let mut sorted_segments = segments;

    sorted_segments.sort_by(|a, b| b.start.cmp(&a.start));

    for segment in sorted_segments {
        match segment.kind {
            SegmentKind::Variable => {
                update_variable_html(&mut html_content, &segment, &values);
            }
            SegmentKind::Condition | SegmentKind::Loop => {
                parse_segment_kind(&mut html_content, &segment, &values);
            }
        }
    }
    html_content
}

fn update_variable_html( content_html: &mut String, segment: &Segment, values: &Value) {
    if let Some(value) = values.get(&segment.key) {
        content_html.replace_range(segment.start..segment.end, value.as_str().unwrap());
    }
}

fn parse_segment_kind(
    content: &mut String,
    segment: &Segment,
    values: &Value
) {
    match segment.kind {
        SegmentKind::Condition => {
            if let Some(condition_value) = values.get(&segment.key) {
                if condition_value == "False" || condition_value == "false" {
                    content.replace_range(segment.start..segment.end, "");
                    return;
                }
            }
        }
        SegmentKind::Loop => {
            generate_loop_content(content, segment, values);
        }
        _ => {}
    }

    let range_out = segment.get_range_tag("out").unwrap();
    let range_in = segment.get_range_tag("in").unwrap();

    content.replace_range(range_out, "");
    for child in &segment.children {
        match child.kind {
            SegmentKind::Variable => {
                update_variable_html(content, &child, &values);
            }
            SegmentKind::Condition => {
                parse_segment_kind(content, &child, &values);
            }
            SegmentKind::Loop => {
                generate_loop_content(content, &child, &values);
            }
        }
    }
    content.replace_range(range_in, "");
}



fn generate_loop_content(
    content: &mut String,
    child: &Segment,
    values: &Value,
) {

    if let Some(list_child) = values.get(&child.key) {

        let content_loop = child.get_content_loop(content).unwrap();

        println!("DEBUG - LOOP - {}", content_loop);

        let range_out = child.get_range_tag("out").unwrap();
        content.replace_range(range_out, "");

        let variables = get_segments(&content_loop);

        let mut new_content: Vec<String> = Vec::new();

        for item in list_child.as_array().unwrap() {
            let tmp_content = replace_variable(content_loop.clone(), variables.clone(), item);
            new_content.push(tmp_content);
        }

        content.replace_range(child.start..child.end - child.len_out, &new_content.join("\n"));
    }
}
