// #![allow(clippy::all)]
extern crate calypso_cangen;
extern crate proc_macro;
extern crate serde_json;
use calypso_cangen::can_gen_decode::*;
use calypso_cangen::can_gen_encode::*;
use calypso_cangen::can_gen_simulate::*;
use calypso_cangen::can_types::*;
use calypso_cangen::CANGEN_SPEC_PATH;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as ProcMacro2TokenStream;
use quote::{format_ident, quote};
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;

/**
 *  Macro to generate all the code for decode_data.rs
 *  - Generates prelude, phf map, and all decode functions
 */
#[proc_macro]
pub fn gen_decode_data(_item: TokenStream) -> TokenStream {
    let __decode_prelude = quote! {
        use std::io::Cursor;
        use bitstream_io::{BigEndian, LittleEndian, BitReader, BitRead};
        use phf::phf_map;
        use calypso_cangen::can_types::*;
        use crate::data::{DecodeData, FormatData};
    };
    let mut __decode_functions = quote! {
        pub fn decode_mock(_data: &[u8]) -> Vec::<DecodeData> {
            let result = vec![
                DecodeData::new(vec![0.0], "Calypso/Unknown", "")
            ];
            result
        }
    };
    let mut __decode_map_entries = ProcMacro2TokenStream::new();

    // Iterate through CAN spec directory and generate decode functions/mappings
    // for each valid entry
    if let Ok(__entries) = fs::read_dir(CANGEN_SPEC_PATH) {
        __entries 
            .filter_map(Result::ok)
            .map(|__entry| __entry.path())
            .filter(|__path| {
                __path.is_file() && _path.extension().is_some_and(|ext| ext == "json")
            })
            .for_each(|__path| {
                __decode_functions.extend(gen_decode_fns(__path.clone()));
                __decode_map_entries.extend(gen_decode_mappings(__path.clone()));
            });
    } else {
        eprintln!("Could not read from directory: {}", CANGEN_SPEC_PATH);
    }

    let __decode_expanded = quote! {
        #__decode_prelude

        #__decode_functions

        pub static DECODE_FUNCTION_MAP: phf::Map<u32, fn(data: &[u8]) -> Vec<DecodeData>> = phf_map! {
            #__decode_map_entries
        };
    };
    TokenStream::from(__decode_expanded)
}

/**
 *  Helper function to generate decode phf map entries for a given JSON spec file
 */
fn gen_decode_mappings(_path: PathBuf) -> ProcMacro2TokenStream {
    let _contents = match fs::read_to_string(&_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error opening file {:?}: {}", _path, e);
            return quote! {};
        }
    };

    // Iterate through CANMsg list and generate decode mapping for each
    let mut _msgs: Vec<CANMsg> = serde_json::from_str(&_contents).unwrap();
    let _entries: ProcMacro2TokenStream = _msgs 
        .iter_mut()
        .map(|_m| {
            let _id_int = 
                u32::from_str_radix(_m.id.clone().trim_start_matches("0x"), 16).unwrap();
            let _fn_name = format_ident!(
                "decode_{}",
                _m.desc.clone().to_lowercase().replace(' ', "_")
            );
            quote! { #_id_int => #_fn_name, }
        })
        .fold(ProcMacro2TokenStream::new(), |mut acc, ts| {
            acc.extend(ts);
            acc
        });

    quote! {
        #_entries
    }
}

/**
 *  Helper function to generate decode functions for a given JSON spec file
 */
fn gen_decode_fns(_path: PathBuf) -> ProcMacro2TokenStream {
    let _contents = match fs::read_to_string(&_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error opening file {:?}: {}", _path, e);
            return quote! {};
        }
    };

    // Iterate through CANMsg list and generate decode function for each
    let mut _msgs: Vec<CANMsg> = serde_json::from_str(&_contents).unwrap();
    let _fns = _msgs
        .iter_mut()
        .map(|_m| gen_decoder_fn(_m))
        .collect::<Vec<ProcMacro2TokenStream>>()
        .into_iter()
        .fold(ProcMacro2TokenStream::new(), |mut acc, ts| {
            acc.extend(ts);
            acc.extend(ProcMacro2TokenStream::from_str("\n"));
            acc
        });

    quote! {
        #_fns
    }
}

/**
 *  Macro to generate all the code for encode_data.rs
 *  - Generates prelude, phf map, and all encode functions
 */
#[proc_macro]
pub fn gen_encode_data(_item: TokenStream) -> TokenStream {
    let __encode_prelude = quote! {
        use bitstream_io::{BigEndian, LittleEndian, BitWriter, BitWrite};
        use phf::phf_map;
        use calypso_cangen::can_types::*;
        use crate::data::{EncodeData, FormatData};
    };
    let mut __encode_functions = quote! {
        pub fn encode_mock(data: Vec<f32>) -> EncodeData {
            let mut writer = BitWriter::endian(Vec::new(), BigEndian);
            writer.write_from::<u8>(data.len() as u8).unwrap();
            EncodeData {
                value: writer.into_writer(),
                id: 2047,
                is_ext: false,
            }
        }
    };
    let mut __encode_map_entries = ProcMacro2TokenStream::new();
    let mut __encode_key_list_entries = ProcMacro2TokenStream::new();
    let mut __encode_key_list_size: usize = 0;

    // Iterate through CAN spec directory and generate encode functions/mappings
    // for each valid entry
    if let Ok(__entries) = fs::read_dir(CANGEN_SPEC_PATH) {
        __entries 
            .filter_map(Result::ok)
            .map(|__entry| __entry.path())
            .filter(|__path| {
                __path.is_file() && _path.extension().is_some_and(|ext| ext == "json")
            })
            .for_each(|__path| {
                __encode_functions.extend(gen_encode_fns(__path.clone()));
                __encode_map_entries.extend(gen_encode_mappings(__path.clone()));
                __encode_key_list_entries.extend(gen_encode_keys(
                    __path.clone(),
                    &mut __encode_key_list_size,
                ));
            });
    } else {
        eprintln!("Could not read from directory: {}", CANGEN_SPEC_PATH);
    }

    let __encode_expanded = quote! {
        #__encode_prelude

        #__encode_functions

        pub static ENCODE_FUNCTION_MAP: phf::Map<&'static str, fn(data: Vec<f32>) -> EncodeData> = phf_map! {
            #__encode_map_entries
        };

        pub const ENCODABLE_KEY_LIST: [&str; #__encode_key_list_size] = [
            #__encode_key_list_entries
        ];
    };
    TokenStream::from(__encode_expanded)
}

/**
 *  Helper function to generate encode functions for a given JSON spec file
 */
fn gen_encode_fns(_path: PathBuf) -> ProcMacro2TokenStream {
    let _contents = match fs::read_to_string(&_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error opening file {:?}: {}", _path, e);
            return quote! {};
        }
    };

    // Iterate through CANMsg list and generate encode function for each
    let mut _msgs: Vec<CANMsg> = serde_json::from_str(&_contents).unwrap();
    let _fns = _msgs
        .iter_mut()
        .map(|_m| gen_encoder_fn(_m))
        .collect::<Vec<ProcMacro2TokenStream>>()
        .into_iter()
        .fold(ProcMacro2TokenStream::new(), |mut acc, ts| {
            acc.extend(ts);
            acc
        });

    quote! {
        #_fns
    }
}

/**
 *  Helper function to generate encode phf map entries for a given JSON spec file
 */
fn gen_encode_mappings(_path: PathBuf) -> ProcMacro2TokenStream {
    let _contents = match fs::read_to_string(&_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error opening file {:?}: {}", _path, e);
            return quote! {};
        }
    };

    let mut _msgs: Vec<CANMsg> = serde_json::from_str(&_contents).unwrap();
    let mut _entries = ProcMacro2TokenStream::new();

    // Only create encode mappings for CANMsgs with key field
    for mut _msg in _msgs {
        let _entry = match &_msg.key {
            Some(key) => {
                let fn_name = format_ident!(
                    "encode_{}",
                    _msg.desc.clone().to_lowercase().replace(' ', "_")
                );
                quote! {
                    #key => #fn_name,
                }
            }
            None => {
                quote! {}
            }
        };
        _entries.extend(_entry);
    }

    quote! {
        #_entries
    }
}

/**
 *  Helper function to generate encode key list entries for a given JSON spec file
 */
fn gen_encode_keys(_path: PathBuf, _key_list_size: &mut usize) -> ProcMacro2TokenStream {
    let _contents = match fs::read_to_string(&_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error opening file {:?}: {}", _path, e);
            return quote! {};
        }
    };

    let mut _msgs: Vec<CANMsg> = serde_json::from_str(&_contents).unwrap();
    let mut _entries = ProcMacro2TokenStream::new();
    for mut _msg in _msgs {
        let _entry = match &_msg.key {
            Some(key) => {
                *_key_list_size += 1;
                quote! {
                    #key,
                }
            }
            None => {
                quote! {}
            }
        };
        _entries.extend(_entry);
    }

    quote! {
        #_entries
    }
}


/**
 *  Macro to generate all the code for simulate_data.rs 
 *  - Generates prelude, main function, and all components
 */
#[proc_macro]
pub fn gen_simulate_data(_item: TokenStream) -> TokenStream {
    let _simulate_prelude = quote! {
        use std::time::Instant;
        use crate::simulatable_message::{SimComponent, SimValue, SimPoint};
    };

    let mut _simulate_obj_entries = ProcMacro2TokenStream::new();

    if let Ok(entries) = fs::read_dir(CANGEN_SPEC_PATH) {
        entries
            .filter_map(Result::ok)
            .map(|_entry| _entry.path())
            .filter(|_path| {
                _path.is_file()
                    && _path.extension().map(|ext| ext == "json").unwrap_or(false)
            })
            .for_each(|path| {
                _simulate_obj_entries.extend(gen_simulate_file_to_objects(path.clone()));
            });
    } else {
        eprintln!("Could not read from directory: {}", CANGEN_SPEC_PATH);
    }

    let _simulate_mainfunc = quote! {
        pub fn create_simulated_components() -> Vec<SimComponent> {
            let mut __all_sim_components: Vec<SimComponent> = Vec::new();
            #_simulate_obj_entries // Loop of (new entry, push entry)...
            __all_sim_components.iter_mut().for_each(|c| c.initialize());
            __all_sim_components
        }
    };

    let combined = quote! {
        #_simulate_prelude
        #_simulate_mainfunc
    };

    TokenStream::from(combined)
}

/**
 *  Helper function to generate Sim objects for a given JSON spec file
 */
fn gen_simulate_file_to_objects(_path: PathBuf) -> ProcMacro2TokenStream {
    let _contents = match fs::read_to_string(&_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error opening file {:?}: {}", _path, e);
            return quote! {};
        }
    };

    let mut _msgs: Vec<CANMsg> = serde_json::from_str(&_contents).unwrap();
    let _objects: ProcMacro2TokenStream = _msgs
        .iter_mut()
        .map(|_m| gen_simulate_canmsg(_m))
        .fold(ProcMacro2TokenStream::new(), |mut acc, ts| {
            acc.extend(ts);
            acc.extend(ProcMacro2TokenStream::from_str("\n"));
            acc
        });

    quote! {
        #_objects
    }
}
