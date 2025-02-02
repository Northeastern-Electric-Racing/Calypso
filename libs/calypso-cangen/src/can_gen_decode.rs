use crate::can_types::*;
use proc_macro2::Literal;
use proc_macro2::TokenStream as ProcMacro2TokenStream;
use quote::{format_ident, quote};
use regex::Regex;

/**
 *  Function to generate decoder function for a CANMsg
 */
pub fn gen_decoder_fn(msg: &mut CANMsg) -> ProcMacro2TokenStream {
    // Generate local variables for each CANPoint in the Message
    let point_decoders = msg
        .points
        .iter_mut()
        .enumerate()
        .map(|(i, point)| gen_decoder_point(i, point))
        .collect::<Vec<ProcMacro2TokenStream>>()
        .into_iter()
        .fold(ProcMacro2TokenStream::new(), |mut acc, ts| {
            acc.extend(ts);
            acc
        });
    // Push DecodeData structs to result for each NetField in the message
    let field_decoders = msg 
        .fields
        .iter_mut()
        .map(|field| gen_decoder_field(field, &mut msg.points))
        .collect::<Vec<ProcMacro2TokenStream>>()
        .into_iter()
        .fold(ProcMacro2TokenStream::new(), |mut acc, ts| {
            acc.extend(ts);
            acc
        });
    let min_size: usize = msg
        .fields
        .iter()
        .map(|field| field.values.iter().map(|value| msg.points[value-1].size).sum::<usize>())
        .sum::<usize>()
        / 8;
    let fn_name = format_ident!(
        "decode_{}",
        msg.desc.clone().to_lowercase().replace(' ', "_")
    );

    quote! {
        pub fn #fn_name(data: &[u8]) -> Vec<DecodeData> {
            if data.len() < #min_size { return vec![]; }
            let mut reader = BitReader::endian(Cursor::new(&data), BigEndian);
            let mut result: Vec<DecodeData> = Vec::new();
            #point_decoders
            #field_decoders
            result
        }
    }
}

/**
 *  Function to generate DecodeData struct for decoding a NetField
 */
fn gen_decoder_field(field: &mut NetField, points: &mut Vec<CANPoint>) -> ProcMacro2TokenStream {
    // First, check that all of the correlated points are to be parsed
    // If not, return a blank TokenStream
    if let false = field
        .values
        .iter()
        .map(|value| points[value-1].parse.unwrap_or(true))
        .fold(true, |mut acc, p| { acc &= p; acc })
    {
       return quote! { };
    }
    // Otherwise, construct TokenStream as normal

    let unit = field.unit.clone();

    // Construct comma-separated list of relevant points
    let values = field
        .values
        .iter()
        .map(|value| {
            if points[value-1].parse.unwrap_or(true) {
                let val_point = format_ident!("point_{}", value);
                quote! { #val_point, }
            } else {
                quote! { }
            }
        })
        .collect::<Vec<ProcMacro2TokenStream>>()
        .into_iter()
        .fold(ProcMacro2TokenStream::new(), |mut acc, ts| {
            acc.extend(ts);
            acc
        });


    // In-topic name handling
    // Use regex to parse point indices from field name
    let topic_regex_pattern = Regex::new(r"\{(\d+)\}").unwrap();  // Basically, digits enclosed in braces 
    let topic_format_value_indexes: Vec<usize> = topic_regex_pattern
        .captures_iter(&field.name.clone())
        .map(|cap| cap[1].parse::<usize>().unwrap())
        .collect();
    // Generate comma-separated list of points for runtime format!() call
    let topic_format_values = topic_format_value_indexes
        .iter()
        .map(|value| {
            if points[value-1].parse.unwrap_or(true) {
                let val_point = format_ident!("point_{}", value);
                quote! { #val_point, }
            } else {
                quote! { }
            }
        })
        .collect::<Vec<ProcMacro2TokenStream>>()
        .into_iter()
        .fold(ProcMacro2TokenStream::new(), |mut acc, ts| {
            acc.extend(ts);
            acc
        });
    // This converts the topic name from JSON spec into a suitable input for format!()
    // i.e. "Hello/{8}/World/{9}/" -> "Hello/{0}/World{1}/"
    let mut topic_format_string = field.name.clone();
    for (i, val) in topic_format_value_indexes.iter().enumerate() {
        let pattern = format!("{{{}}}", val);
        let replacement = format!("{{{}}}", i);
        topic_format_string = topic_format_string.replace(&pattern, &replacement);
    }
    let topic = quote! { 
        &format!(#topic_format_string, #topic_format_values)
    };

    quote! {
        result.push(
            DecodeData::new(vec![#values],
                #topic,
                #unit)
        );
    }
}

/**
 *  Function to generate formatted reader line for decoding a CANPoint
 *  i.e. `let point_X = ...;`, where X is index
 */
fn gen_decoder_point(index: usize, point: &mut CANPoint) -> ProcMacro2TokenStream {
    let size_literal = Literal::usize_unsuffixed(point.size);

    // If parse exists and is false, then skip this point
    if let Some(false) = point.parse {
        return quote! {
            reader.skip(#size_literal).unwrap();
        };
    }
    // Otherwise, parse this point as normal (default behavior)
    let point_name = format_ident!("point_{}", index+1); 

    // If this point is an IEEE754 f32, always read it as a u32, and transmute to f32 later
    let read_type = match point.ieee754_f32 {
        Some(true) => quote! { u32 },
        _ => match point.signed {
            Some(true) => match point.size {
                0..=8 => quote! { i8 },
                9..=16 => quote! { i16 },
                _ => quote! { i32 },
            },
            _ => match point.size {
                0..=8 => quote! { u8 },
                9..=16 => quote! { u16 },
                _ => quote! { u32 },
            },
        },
    };

    // Prefix to call potential format function
    let format_prefix = match &point.format {
        Some(format) => {
            let id = format_ident!("{}_d", format);
            quote! { FormatData::#id }
        }
        _ => quote! {},
    };

    // Endianness and signedness affect which read to use
    let read_func = match point.endianness {
        Some(ref s) if s == "little" => {
            quote! {
                reader.read_as_to::<LittleEndian, #read_type>().unwrap()
            }
        }
        _ => match point.signed {
            Some(true) if point.ieee754_f32.is_none() => {
                quote! { reader.read_signed_in::<#size_literal, i32>().unwrap() }
            }
            _ => quote! { reader.read_in::<#size_literal, u32>().unwrap() },
        },
    };

    // Transmute if point is IEEE754 f32, else convert
    if let Some(true) = point.ieee754_f32 {
        quote! { let #point_name = #format_prefix (f32::from_bits(#read_func)); }
    } else {
        quote! { let #point_name = #format_prefix (#read_func as f32); }
    }
}
