extern crate calypso_cangen;
extern crate proc_macro;
extern crate serde_json;
use calypso_cangen::CANGEN_SPEC_PATH;
use calypso_cangen::can_gen_decode::gen_decoder_fn;
use calypso_cangen::can_gen_encode::gen_encoder_fn;
use calypso_cangen::can_gen_simulate::gen_simulate_canmsg;
use definition_rs::{BidirMode, OdysseyMsg};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as ProcMacro2TokenStream;
use quote::{format_ident, quote};
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

/**
 *  Macro to generate all the code for `decode_data.rs`
 *  - Generates prelude, phf map, and all decode functions
 */
#[proc_macro]
pub fn gen_decode_data(_item: TokenStream) -> TokenStream {
    let __decode_prelude = quote! {
        use std::io::Cursor;
        use bitstream_io::{BigEndian, LittleEndian, BitReader, BitRead};
        use phf::phf_map;
        use definition_rs::*;
        use crate::data::{DecodeData, FormatData};
    };
    let mut __decode_functions = quote! {};
    let mut __decode_map_entries = ProcMacro2TokenStream::new();

    // get the parsed JSON of each valid spec file
    let __parsed = match fs::read_dir(CANGEN_SPEC_PATH) {
        Ok(__parsed) => __parsed,
        Err(__error) => {
            eprintln!("Could not read from directory: {CANGEN_SPEC_PATH} with error: {__error}");
            return TokenStream::new();
        }
    };

    let __json: Vec<OdysseyMsg> = __parsed
        .filter_map(Result::ok)
        .map(|__entry| __entry.path())
        .filter(|__path| __path.is_file() && __path.extension().is_some_and(|ext| ext == "json"))
        .filter_map(|__path| {
            let __data = match fs::read_to_string(__path) {
                Ok(__data) => __data,
                Err(__error) => {
                    eprintln!("Could not read file: {__error}");
                    return None;
                }
            };

            // treat deserialization failures as critical
            Some(
                serde_json::from_str::<Vec<OdysseyMsg>>(&__data)
                    .expect("Error deserializing {__path}"),
            )
        })
        .flatten()
        .collect();

    let mut __decode_functions = gen_decode_fns(__json.clone());
    let mut __decode_map_entries = gen_decode_mappings(__json);

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
 *  Helper function to generate decode phf map entries for a given
 *  Odyssey Messages
 */
fn gen_decode_mappings(mut _msgs: Vec<OdysseyMsg>) -> ProcMacro2TokenStream {
    let _entries: ProcMacro2TokenStream = _msgs
        .iter_mut()
        .filter_map(|_m| match _m {
            OdysseyMsg::Can(can) => Some(can),
            OdysseyMsg::Meta(_) => None,
        })
        .map(|_m| {
            let _id_int = u32::from_str_radix(_m.id.clone().trim_start_matches("0x"), 16).unwrap();
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
 *  Helper function to generate decode functions for given Odyssey Messages
 */
fn gen_decode_fns(mut _msgs: Vec<OdysseyMsg>) -> ProcMacro2TokenStream {
    let _fns = _msgs
        .iter_mut()
        .filter_map(|_m| match _m {
            OdysseyMsg::Can(can) => Some(can),
            OdysseyMsg::Meta(_) => None,
        })
        .map(gen_decoder_fn)
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
 *  Macro to generate all the code for `encode_data.rs`
 *  - Generates prelude, phf map, and all encode functions
 */
#[proc_macro]
pub fn gen_encode_data(_item: TokenStream) -> TokenStream {
    let __encode_prelude = quote! {
        use bitstream_io::{BigEndian, LittleEndian, BitWriter, BitWrite};
        use phf::phf_map;
        use definition_rs::*;
        use crate::data::{EncodeData, FormatData};
    };
    let mut __encode_functions = quote! {};
    let mut __encode_map_entries = ProcMacro2TokenStream::new();
    let mut __encode_key_list_entries = ProcMacro2TokenStream::new();
    let mut __encode_key_list_size: usize = 0;

    // get the parsed JSON of each valid spec file
    let __parsed = match fs::read_dir(CANGEN_SPEC_PATH) {
        Ok(__parsed) => __parsed,
        Err(__error) => {
            eprintln!("Could not read from directory: {CANGEN_SPEC_PATH} with error: {__error}");
            return TokenStream::new();
        }
    };

    let __json: Vec<OdysseyMsg> = __parsed
        .filter_map(Result::ok)
        .map(|__entry| __entry.path())
        .filter(|__path| __path.is_file() && __path.extension().is_some_and(|ext| ext == "json"))
        .filter_map(|__path| {
            let __data = match fs::read_to_string(__path) {
                Ok(__data) => __data,
                Err(__error) => {
                    eprintln!("Could not read file: {__error}");
                    return None;
                }
            };

            // treat deserialization failures as critical
            Some(
                serde_json::from_str::<Vec<OdysseyMsg>>(&__data)
                    .expect("Error deserializing {__path}"),
            )
        })
        .flatten()
        .collect();

    // Iterate through CAN spec directory and generate encode functions/mappings
    // for each valid entry

    let __encode_functions = gen_encode_fns(__json.clone());
    let __encode_map_entries = gen_encode_mappings(__json.clone());
    let __encode_key_list_entries = gen_encode_keys(__json, &mut __encode_key_list_size);

    let __encode_expanded = quote! {
        #__encode_prelude

        #__encode_functions

        pub static ENCODE_FUNCTION_MAP: phf::Map<&'static str, (fn(data: Vec<f32>) -> EncodeData, BidirMode)> = phf_map! {
            #__encode_map_entries
        };

        pub const ENCODABLE_KEY_LIST: [&str; #__encode_key_list_size] = [
            #__encode_key_list_entries
        ];
    };
    TokenStream::from(__encode_expanded)
}

/**
 *  Helper function to generate encode functions for given Odyssey Messages
 */
fn gen_encode_fns(mut _msgs: Vec<OdysseyMsg>) -> ProcMacro2TokenStream {
    let _fns = _msgs
        .iter_mut()
        .filter_map(|e| match e {
            OdysseyMsg::Can(group) => Some(group),
            _ => None,
        })
        .map(gen_encoder_fn)
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
 *  Helper function to generate encode phf map entries for
 *  given Odyssey Messages
 */
fn gen_encode_mappings(mut _msgs: Vec<OdysseyMsg>) -> ProcMacro2TokenStream {
    let _entries = _msgs
        .iter_mut()
        .filter_map(|_m| match _m {
            OdysseyMsg::Can(send_canmsg) => Some(send_canmsg),
            OdysseyMsg::Meta(_) => None,
        })
        .map(|_m| {
            if let Some(key) = &_m.key {
                let fn_name = format_ident!(
                    "encode_{}",
                    _m.desc.clone().to_lowercase().replace(' ', "_")
                );
                let bidir_mode = _m.bidir_mode;
                quote! { #key => (#fn_name,  #bidir_mode),}
            } else {
                quote! {}
            }
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
 *  Helper function to generate encode key list entries for
 *  given Odyssey Messages
 */
fn gen_encode_keys(
    mut _msgs: Vec<OdysseyMsg>,
    _key_list_size: &mut usize,
) -> ProcMacro2TokenStream {
    let _entries = _msgs
        .iter_mut()
        .filter_map(|_m| match _m {
            OdysseyMsg::Can(send_canmsg) => Some(send_canmsg),
            OdysseyMsg::Meta(_) => None,
        })
        .map(|_m| {
            if let Some(key) = &_m.key {
                // dont add to list if oneshot
                if _m.bidir_mode == BidirMode::Oneshot {
                    quote! {}
                } else {
                    *_key_list_size += 1;
                    quote! { #key, }
                }
            } else {
                quote! {}
            }
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
 *  Macro to generate all the code for `simulate_data.rs`
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
            .filter(|_path| _path.is_file() && _path.extension().is_some_and(|ext| ext == "json"))
            .for_each(|path| {
                _simulate_obj_entries.extend(gen_simulate_file_to_objects(path.clone()));
            });
    } else {
        eprintln!("Could not read from directory: {CANGEN_SPEC_PATH}");
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
            eprintln!("Error opening file {_path:?}: {e}");
            return quote! {};
        }
    };

    let mut _msgs: Vec<OdysseyMsg> = serde_json::from_str(&_contents).unwrap();
    let _objects: ProcMacro2TokenStream = _msgs
        .iter_mut()
        .filter_map(|_m| match _m {
            OdysseyMsg::Can(canmsg) => Some(canmsg),
            OdysseyMsg::Meta(_) => None, // meta messages cannot be simulated yet
        })
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
