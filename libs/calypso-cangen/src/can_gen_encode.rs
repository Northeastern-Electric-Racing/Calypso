use crate::can_types::*;
use proc_macro2::Literal;
use proc_macro2::TokenStream as ProcMacro2TokenStream;
use quote::{format_ident, quote};

/**
 *  Function to generate encoder function for CANMsg
 *  Generates nothing if the CANMsg is not Encodable (i.e. no key)
 */
pub fn gen_encoder_fn(msg: &mut CANMsg) -> ProcMacro2TokenStream {
    match &msg.key {
        Some(_key) => {
            let point_encoders = msg
                .points
                .iter_mut()
                .map(|point| gen_encoder_point(point))
                .collect::<Vec<ProcMacro2TokenStream>>()
                .into_iter()
                .fold(ProcMacro2TokenStream::new(), |mut acc, ts| {
                    acc.extend(ts);
                    acc
                });
            let fn_name = format_ident!(
                "encode_{}",
                msg.desc.clone().to_lowercase().replace(' ', "_")
            );
            let id_int =
                u32::from_str_radix(msg.id.clone().trim_start_matches("0x"), 16).unwrap();
            let ext_ident = msg.is_ext.unwrap_or(false);
            quote! {
                pub fn #fn_name(data: Vec<f32>) -> EncodeData {
                    let mut writer = BitWriter::endian(Vec::new(), BigEndian);
                    let mut pt_index: usize = 0;
                    #point_encoders
                    EncodeData {
                        value: writer.into_writer(),
                        id: #id_int,
                        is_ext: #ext_ident,
                    }
                }
            }
        }
        None => {
            quote! {}
        }
    }
}

/**
 *  Function to generate encoder line for CANPoint
 */
fn gen_encoder_point(point: &mut CANPoint) -> ProcMacro2TokenStream {
    let size_literal = Literal::usize_unsuffixed(point.size);
    let write_type = match point.signed {
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
    };
    let format_prefix = match &point.format {
        Some(format) => {
            let id = format_ident!("{}_e", format);
            quote! {  FormatData::#id }
        }
        _ => quote! {},
    };
    let default_value: f32 = match point.default {
        Some(default) => default,
        _ => 0f32,
    };
    let float_final = quote! {
        #format_prefix ( *data.get(pt_index).unwrap_or(&(#default_value)) )
    };

    let write_line = match point.endianness {
        // Little endian
        Some(ref s) if s == "little" => {
            quote! {
                writer.write_as_from::<LittleEndian, #write_type>(#float_final as #write_type).unwrap();
            }
        }
        // Big endian (default)
        _ => {
            match point.signed {
                // Signed
                Some(true) => {
                    quote! {
                        writer.write_signed_out::<#size_literal, #write_type>(#float_final as #write_type).unwrap();
                    }
                }
                // Unsigned (default)
                _ => {
                    quote! {
                        writer.write_out::<#size_literal, #write_type>(#float_final as #write_type).unwrap();
                    }
                }
            }
        }
    };
    quote! {
        #write_line
        pt_index += 1;
    }
}
