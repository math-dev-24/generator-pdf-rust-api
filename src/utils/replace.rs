use std::collections::HashMap;
use serde_json::Value;
use crate::types::variable::{Segment, SegmentKind};
use crate::utils::variables::get_segments;

pub fn replace_variable(
    html_content: String, segments: Vec<Segment>, values: &Value
) -> String {

    let mut content = html_content;
    let mut sorted_segments = segments;
    sorted_segments.sort_by(|a, b| b.start.cmp(&a.start));

    for segment in sorted_segments {
        match segment.kind {
            SegmentKind::Variable => {
                update_variable_html(&mut content, &segment, &values);
            }
            SegmentKind::Condition | SegmentKind::Loop => {
                parse_segment_kind(&mut content, &segment, &values);
            }
        }
    }
    content
}

fn update_variable_html(
    content_html: &mut String,
    segment: &Segment,
    values: &Value
) {
    if let Some(value) = values.get(&segment.key) {
        content_html.replace_range(segment.start..segment.end, value.as_str().unwrap());
    }
}

fn parse_segment_kind(content: &mut String, segment: &Segment, values: &Value) {

    if segment.kind == SegmentKind::Condition {
        if let Some(condition_value) = values.get(&segment.key) {
            if condition_value == "False" || condition_value == "false" {
                content.replace_range(segment.start..segment.end, "");
                return;
            }
        }
    } else if segment.kind == SegmentKind::Loop {
        generate_loop_content(content, segment, values);
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
                generate_loop_content(content, &child, &values);
            }
        }
    }

    content.replace_range(segment.start..segment.start + segment.len_in, "");
}



fn generate_loop_content(
    content: &mut String,
    child: &Segment,
    values: &Value,
) {

    if let Some(list_child) = values.get(&child.key) {
        // contenu actuel avec variable
        let mut content_loop = content[child.start + child.len_in..child.end - child.len_out].to_string();
        // delete balise de fin
        content.replace_range(child.end - child.len_out..child.end, "");
        // récupérer les variables disponibles
        let variables = get_segments(&content_loop);

        let mut new_content: Vec<String> = Vec::new();


        for item in list_child.as_array().unwrap() {
            new_content.push(replace_variable(content_loop.to_string(), variables.clone(), item));
        }

        // suppresion du début et remplacement
        content.replace_range(child.start..child.end - child.len_out, &new_content.join("\n"));

    }
}