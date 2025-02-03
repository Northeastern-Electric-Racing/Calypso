#![allow(dead_code)] // TODO: Cleanup
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use crate::can_types::*;
use proc_macro2::Literal;
use proc_macro2::TokenStream as ProcMacro2TokenStream;
use quote::{format_ident, quote};
use regex::Regex;

/**
 *  Function to generate SimComponentAttrs and SimComponents for a CANMsg
 */
pub fn gen_simulate_canmsg(msg: &CANMsg) -> ProcMacro2TokenStream {
    let Some(sim_freq) = msg.sim_freq else {
        return quote! {};
    };

    if msg.points.iter().any(|point| point.sim.is_none()) {
        eprintln!(
            "[warning] sim_freq defined for {} ({}), but some points have no sim values",
            msg.id, msg.desc
        );
        return quote! {};
    }

    let sim_components_token = msg
        .fields
        .iter()
        .map(|field| gen_simulate_netfield(field, &msg.points, &msg))
        .collect::<ProcMacro2TokenStream>();

    quote! {
        #sim_components_token
    }
}

fn get_topic_format_value_indexes(field: &NetField) -> Vec<usize> {
    let topic_regex_pattern = Regex::new(r"\{(\d+)\}").unwrap(); // Basically, digits enclosed in braces
    let topic_format_value_indexes: Vec<usize> = topic_regex_pattern
        .captures_iter(&field.name.clone())
        .map(|cap| cap[1].parse::<usize>().unwrap())
        .collect();
    topic_format_value_indexes
}

/**
 * Convert a NetField to a SimComponent
 */
pub fn gen_simulate_netfield(field: &NetField, points: &Vec<CANPoint>, msg: &CANMsg) -> ProcMacro2TokenStream {
    let mut token_pts_val_buffer = ProcMacro2TokenStream::new();
    
    #[allow(unused_doc_comments)]
    /**
     * forEach(NetField)
     *      if (NetField has in-topic values)
     *          ( create vec<SimPoint> )
     *          forEach(in-topic values)
     *             ( create SimValue )
     *              ( create SimPoint, embedding the SimValue )
     *          ( add SimPoint to vec<SimPoint> )
     * 
     *      ( create vec<SimPoint> )
     *      forEach(normal canpoint values)
     *          ( create SimValue )
     *          ( create SimPoint, embedding the SimValue )
     *      ( add SimPoint to vec<SimPoint> )
     * 
     *      ( create SimComponent, embedding the vec<SimPoint> )
     */

    // Handle in-topic CANPoint values
    let points_idx_intopic = get_topic_format_value_indexes(field);
    if points_idx_intopic.len() > 0 {
        token_pts_val_buffer.extend(quote! {
            let mut vec_points_in_topic: Vec<SimPoint> = Vec::new();
        });
        for idx in &points_idx_intopic {
            token_pts_val_buffer.extend(gen_sim_point(&points[*idx - 1]));
            token_pts_val_buffer.extend(quote! { vec_points_in_topic.push(__new_point); });
        }
    }

    // Handle normal CANPoint values
    token_pts_val_buffer.extend(quote! {
        let mut vec_points: Vec<SimPoint> = Vec::new();
    });
    for idx in &field.values {
        token_pts_val_buffer.extend(gen_sim_point(&points[*idx - 1]));
        token_pts_val_buffer.extend(quote! { vec_points.push(__new_point); });
    }

    // println!("in topic values are {:?}", intopic_values)

    let _debug_field_name = &field.name;

    let token_id = &msg.id;
    let token_simfreq = &msg.sim_freq;
    let token_topic = &field.name; // TODO: transformation needed because of in topic values
    let token_desc = &msg.desc;
    let token_name = &field.name;
    let token_unit = &field.unit;
    let token_key = msg
        .key
        .as_ref()
        .map(|e| quote! { Some(#e.to_string()) })
        .unwrap_or_else(|| quote! { None });
    let token_isext = msg
        .is_ext
        .as_ref()
        .map(|e| quote! { Some(#e) })
        .unwrap_or_else(|| quote! { None });

    quote! {
        let _____START_DEBUG_FIELD_NAME = #_debug_field_name;
        #token_pts_val_buffer

        let __new_component = SimComponent {
            id: #token_id.to_string(),
            points: vec_points,
            points_intopic: None,
            topic: #token_topic.to_string(),
            unit: #token_unit.to_string(),
            name: #token_name.to_string(),
            last_update: Instant::now(),
            desc: #token_desc.to_string(),
            key: #token_key,
            is_ext: #token_isext,
            sim_freq: #token_simfreq,
        };
        __all_sim_components.push(__new_component);

        let _____END___DEBUG_FIELD_NAME = #_debug_field_name;
    }
}

/**
 *  Function to generate variable name for a SimComponentAttr  
 */
fn gen_sim_component_attr_name(field: &NetField) -> ProcMacro2TokenStream {
    // format_ident!("{}_attr", field.name.clone().to_lowercase().replace(['/', ' ', '-'], "_"));
    quote! {}
}

/**
 * Function to generate SimPoint (and the SimValue inside it) for a CANPoint
 */
fn gen_sim_point(point: &CANPoint) -> ProcMacro2TokenStream {
    let mut ret_stream = ProcMacro2TokenStream::new();
    if let Some(_s) = &point.sim {
        match _s {
            Sim::SimRange {
                min,
                max,
                inc_min,
                inc_max,
                round,
            } => {
                let _round = match round {
                    Some(true) => true,
                    _ => false,
                };
                ret_stream.extend(quote! {
                    let __new_value = SimValue::Range { min: #min, max: #max, inc_min: #inc_min, inc_max: #inc_max, round: #_round, current: 0.0 };
                });
            }
            Sim::SimDiscrete { options } => {
                let formatted_options = options
                    .iter()
                    .map(|[a, b]| quote! { (#a, #b) })
                    .collect::<Vec<_>>();

                ret_stream.extend(quote! {
                    let __new_value = SimValue::Discrete { options: vec![#(#formatted_options),*], current: 0.0 };
                });
            }
        }
    }
    let size_tokens = point.size;

    let endianness_tokens = point
        .endianness
        .as_ref()
        .map(|e| quote! { Some(#e.to_string()) })
        .unwrap_or_else(|| quote! { None });

    let format_tokens = point
        .format
        .as_ref()
        .map(|e| quote! { Some(#e.to_string()) })
        .unwrap_or_else(|| quote! { None });

    let parse_tokens = point
        .parse
        .as_ref()
        .map(|e| quote! { Some(#e) })
        .unwrap_or_else(|| quote! { None });

    let signed_tokens = point
        .signed
        .as_ref()
        .map(|e| quote! { Some(#e) })
        .unwrap_or_else(|| quote! { None });

    let default_tokens = point
        .default
        .as_ref()
        .map(|e| quote! { Some(#e) })
        .unwrap_or_else(|| quote! { None });

    let ieee_tokens = point
        .ieee754_f32
        .as_ref()
        .map(|e| quote! { Some(#e) })
        .unwrap_or_else(|| quote! { None });

    ret_stream.extend(quote! {
        let __new_point = SimPoint {
            size: #size_tokens,
            parse: #parse_tokens,
            signed: #signed_tokens,
            endianness: #endianness_tokens,
            format: #format_tokens,
            default: #default_tokens,
            ieee754_f32: #ieee_tokens,
            value: __new_value,
        };
    });

    quote! { #ret_stream }
}
