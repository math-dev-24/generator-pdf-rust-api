use std::collections::HashMap;
use crate::types::variable::{Segment, SegmentKind};


pub fn replace_variable(
    html_content: String, segments: Vec<Segment>, values: HashMap<String, String>
) -> String {

    let mut content = html_content;
    let mut sorted_segments = segments;
    sorted_segments.sort_by(|a, b| b.start.cmp(&a.start));

    for segment in sorted_segments {
        match segment.kind {
            SegmentKind::Variable => {
                update_variable_html(&mut content, &segment, &values);
            }
            SegmentKind::Condition => {
                parse_segment_kind(&mut content, &segment, &values);
            }
            SegmentKind::Loop => {
                println!("TODO !")
            }
        }
    }
    content
}

fn update_variable_html(
    content_html: &mut String,
    segment: &Segment,
    values: &HashMap<String, String>
) {
    if let Some(value) = values.get(&segment.key) {
        content_html.replace_range(segment.start..segment.end, value);
    }
}

fn parse_segment_kind(content: &mut String, segment: &Segment, values: &HashMap<String, String>) {

    if segment.kind == SegmentKind::Condition {
        if let Some(condition_value) = values.get(&segment.key) {
            if condition_value == "False" || condition_value == "false" {
                content.replace_range(segment.start..segment.end, "");
                return;
            }
        }
    }

    content.replace_range(segment.end - segment.len_out..segment.end, "");

    for child in &segment.children {
        match child.kind {
            SegmentKind::Variable => {
                update_variable_html(content, &child, &values);
            }
            SegmentKind::Condition => {
                parse_segment_kind(content, &child, &values);
            }
            SegmentKind::Loop => {
                println!("TODO !!!");
            }
        }
    }

    content.replace_range(segment.start..segment.start + segment.len_in, "");
}



fn generate_loop_content(
    children: &[Segment],
    loop_values: &str,
    values: &HashMap<String, String>,
) -> String {
    let mut result = String::new();
    let items: Vec<&str> = loop_values.split(',').collect();

    for item in items {
        let mut temp_values = values.clone(); // Créer une copie locale des valeurs
        temp_values.insert("item".to_string(), item.to_string());

        for child in children {
            if child.kind == SegmentKind::Variable {
                let mut temp_content = String::new();
                update_variable_html(&mut temp_content, child, &temp_values);
                result.push_str(&temp_content);
            } else {
                let mut temp_content = String::new();
                parse_segment_kind(&mut temp_content, child, &temp_values);
                result.push_str(&temp_content);
            }
        }
    }

    result
}