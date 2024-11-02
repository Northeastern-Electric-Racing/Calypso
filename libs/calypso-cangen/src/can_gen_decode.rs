use crate::can_types::*;
use proc_macro2::Literal;
use proc_macro2::TokenStream as ProcMacro2TokenStream;
use quote::{format_ident, quote};

/**
 *  Trait to generate individual decode function for a CANMsg
 *  For NetField and CANPoint, generates parts of the function 
 */
pub trait CANGenDecode {
    fn gen_decoder_fn(&mut self) -> ProcMacro2TokenStream;
}

/**
 *  Function to generate decoder function for a CANMsg
 */
impl CANGenDecode for CANMsg {
    fn gen_decoder_fn(&mut self) -> ProcMacro2TokenStream {
        let field_decoders = self
            .fields
            .iter_mut()
            .map(|field| field.gen_decoder_fn())
            .collect::<Vec<ProcMacro2TokenStream>>()
            .into_iter()
            .fold(ProcMacro2TokenStream::new(), |mut acc, ts| {
                acc.extend(ts);
                acc
            });
        let min_size: usize = self
            .fields
            .iter()
            .map(|field| field.points.iter().map(|point| point.size).sum::<usize>())
            .sum::<usize>()
            / 8;
        let fn_name = format_ident!(
            "decode_{}",
            self.desc.clone().to_lowercase().replace(' ', "_")
        );

        quote! {
            pub fn #fn_name(data: &[u8]) -> Vec<DecodeData> {
                if data.len() < #min_size { return vec![]; }
                let mut reader = BitReader::endian(Cursor::new(&data), BigEndian);
                let mut result: Vec<DecodeData> = Vec::new();
                let mut decoded_points: Vec<f32> = Vec::new();
                let mut topic_suffixes: Vec<f32> = Vec::new();
                #field_decoders
                result
            }
        }
    }
}

/**
 *  Function to generate result.push() line for decoding a NetField
 */
impl CANGenDecode for NetField {
    fn gen_decoder_fn(&mut self) -> ProcMacro2TokenStream {
        match self.send {
            // If send exists and is false, then skip this field (i.e. skip all its points)
            Some(false) => {
                let mut point_skips = ProcMacro2TokenStream::new();
                for point in &self.points {
                    let size_literal = Literal::usize_unsuffixed(point.size);
                    let skip_line = quote! { 
                        reader.skip(#size_literal).unwrap(); 
                    };
                    point_skips.extend(skip_line);
                } 
                quote! {
                    #point_skips
                }
            }
            // Otherwise, send it (default)
            _ => {
                let unit = self.unit.clone();

                // If topic_append, we need to set up the suffix
                match self.topic_append {
                    Some(true) => {
                        let mut topic_suffix_point = self.points.remove(0);
                        let topic_suffix_read = topic_suffix_point.gen_decoder_fn();
                        let topic = format!("{}/", self.name);
                        let point_decoders = self
                            .points
                            .iter_mut()
                            .map(|point| {
                                let decoder = point.gen_decoder_fn();
                                quote! { decoded_points.push(#decoder); }
                            })
                            .collect::<Vec<ProcMacro2TokenStream>>()
                            .into_iter()
                            .fold(ProcMacro2TokenStream::new(), |mut acc, ts| {
                                acc.extend(ts);
                                acc
                            });
                        quote! {
                            topic_suffixes.push(#topic_suffix_read);
                            #point_decoders
                            result.push(
                                DecodeData::new(decoded_points.clone(),
                                    #topic + String::from(topic_suffixes.pop()),
                                    #unit)
                            );
                            decoded_points.clear();
                        }
                    }
                    _ => {
                        let point_decoders = self
                            .points
                            .iter_mut()
                            .map(|point| {
                                let decoder = point.gen_decoder_fn();
                                quote! { decoded_points.push(#decoder); }
                            })
                            .collect::<Vec<ProcMacro2TokenStream>>()
                            .into_iter()
                            .fold(ProcMacro2TokenStream::new(), |mut acc, ts| {
                                acc.extend(ts);
                                acc
                            });
                        let topic = self.name.clone();
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
                }
            }
        }
    }
}

/**
 *  Function to generate formatted reader line for decoding a CANPoint
 */
impl CANGenDecode for CANPoint {
    fn gen_decoder_fn(&mut self) -> ProcMacro2TokenStream {
        // read_func and read_type to map signedness (read_func for big endian, read_type for little endian)
        let size_literal = Literal::usize_unsuffixed(self.size);
        let read_func = match self.signed {
            Some(true) => quote! { reader.read_signed_in::<#size_literal, i32>().unwrap() },
            _ => quote! { reader.read_in::<#size_literal, u32>().unwrap() },
        };
        let read_type = match self.signed {
            Some(true) => quote! { i32 },
            _ => quote! { u32 },
        };

        // prefix to call potential format function
        let format_prefix = match &self.format {
            Some(format) => {
                let id = format_ident!("{}_d", format);
                quote! { FormatData::#id }
            }
            _ => quote! {},
        };

        // Endianness affects which read to use
        match self.endianness {
            Some(ref s) if s == "little" => {
                quote! {
                    #format_prefix (reader.read_as_to::<LittleEndian, #read_type>().unwrap() as f32)
                }
            }
            _ => {
                quote! {
                    #format_prefix (#read_func as f32)
                }
            }
        }
    }
}
