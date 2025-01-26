use crate::can_types::*;
use proc_macro2::Literal;
use proc_macro2::TokenStream as ProcMacro2TokenStream;
use quote::{format_ident, quote};

/**
 *  Function to generate decoder function for a CANMsg
 */
pub fn gen_decoder_fn(msg: &mut CANMsg) -> ProcMacro2TokenStream {
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
            let mut decoded_points: Vec<f32> = Vec::new();
            #field_decoders
            result
        }
    }
}

/**
 *  Function to generate result.push() line for decoding a NetField
 */
// TODO: Implement point/field changes!!
fn gen_decoder_field(field: &mut NetField, points: &mut Vec<CANPoint>) -> ProcMacro2TokenStream {
    let unit = field.unit.clone();

    // In-topic name handling
    let name_before_point = match field.name.find('{') {
        Some(pos) => pos,
        None => 0
    };
    let name_after_point = match field.name.find('}') {
        Some(pos) => pos,
        None => 0
    };
    let topic_append: bool = name_after_point > name_before_point;

    let point_decoders = field
        .values
        .iter()
        .map(|value| {
            let value_point = &mut points[value-1];
            let decoder = gen_decoder_point(value_point);
            if let Some(false) = value_point.parse {
                quote! { #decoder; }
            } else {
                quote! { decoded_points.push(#decoder); }
            }
        })
        .collect::<Vec<ProcMacro2TokenStream>>()
        .into_iter()
        .fold(ProcMacro2TokenStream::new(), |mut acc, ts| {
            acc.extend(ts);
            acc
        });

    let topic = match topic_append {
        true => {
            let point_idx = match field.name[name_before_point+1..name_after_point].parse::<usize>() {
                Ok(num) => num,
                _ => 0
            };
            let point_decoder = match point_idx {
                0 => quote! { "0" },
                idx @ 1.. => {
                    let decoder = gen_decoder_point(&mut points[idx-1]);
                    quote! { #decoder }
                },
            };
            let topic_prefix = field.name[..name_before_point].to_string();
            let topic_suffix = field.name[name_after_point+1..].to_string();
            quote! { #topic_prefix + String::from(#point_decoder) + #topic_suffix }
        },
        false => {
            let name = &field.name;
            quote! { #name }
        }
    };

    quote! {
        #point_decoders
        result.push(
            DecodeData::new(decoded_points.clone(),
                #topic,
                #unit)
        );
        decoded_points.clear();
    }
}

/**
 *  Function to generate formatted reader line for decoding a CANPoint
 */
fn gen_decoder_point(point: &mut CANPoint) -> ProcMacro2TokenStream {
    let size_literal = Literal::usize_unsuffixed(point.size);

    // If parse exists and is false, then skip this point
    if let Some(false) = point.parse {
        let skip_line = quote! {
            reader.skip(#size_literal).unwrap()
        };
        return skip_line; 
    }
    // Otherwise, parse this point as normal (default behavior)

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
        quote! { #format_prefix (f32::from_bits(#read_func)) }
    } else {
        quote! { #format_prefix (#read_func as f32) }
    }
}
