extern crate calypso_cangen;
extern crate proc_macro;
extern crate serde_json;
use calypso_cangen::can_gen_decode::*;
use calypso_cangen::can_gen_encode::*;
use calypso_cangen::can_types::*;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as ProcMacro2TokenStream;
use quote::{quote, format_ident};
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;

/**
 *  Path to CAN spec JSON files
 *  Used by all daedalus macros 
 *  Filepath is relative to project root (i.e. /Calypso)
 */
const DAEDALUS_CANGEN_SPEC_PATH: &'static str = "./Embedded-Base/cangen/can-messages";

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

    match fs::read_dir(DAEDALUS_CANGEN_SPEC_PATH) {
        Ok(__entries) => {
            for __entry in __entries {
                match __entry {
                    Ok(__entry) => {
                        let __path = __entry.path();
                        if __path.is_file() && __path.extension().map_or(false, |ext| ext == "json")
                        {
                            __decode_functions.extend(gen_decode_fns(__path.clone()));
                            __decode_map_entries.extend(gen_decode_mappings(__path.clone()));
                        }
                    }
                    Err(_) => {
                        eprintln!("Could not generate decode functions and mappings");
                    }
                }
            }
        }
        Err(_) => {
            eprintln!("Could not read from directory");
        }
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
    match fs::File::open(_path) {
        Ok(mut _file) => {
            let mut _contents = String::new();
            let _ = _file.read_to_string(&mut _contents);
            let mut _msgs: Vec<CANMsg> = serde_json::from_str(&_contents).unwrap();
            let mut _entries = ProcMacro2TokenStream::new();
            for mut _msg in _msgs {
                let id_int = u32::from_str_radix(_msg.id.clone().trim_start_matches("0x"), 16).unwrap();
                let fn_name = format_ident!(
                    "decode_{}",
                    _msg.desc.clone().to_lowercase().replace(' ', "_")
                );
                let _entry = quote! { #id_int => #fn_name, };
                _entries.extend(_entry);
            }

            quote! {
                #_entries
            }
        }
        Err(_) => {
            eprintln!("Error opening file");
            quote! { }
        }
    }
}

/**
 *  Helper function to generate decode functions for a given JSON spec file
 */
fn gen_decode_fns(_path: PathBuf) -> ProcMacro2TokenStream {
    match fs::File::open(_path) {
        Ok(mut _file) => {
            let mut _contents = String::new();
            let _ = _file.read_to_string(&mut _contents);
            let mut _msgs: Vec<CANMsg> = serde_json::from_str(&_contents).unwrap();
            let _fns = _msgs
                .iter_mut()
                .map(|_m| _m.gen_decoder_fn())
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
        Err(_) => {
            eprintln!("Error opening file");
            quote! { }
        }
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

    match fs::read_dir(DAEDALUS_CANGEN_SPEC_PATH) {
        Ok(__entries) => {
            for __entry in __entries {
                match __entry {
                    Ok(__entry) => {
                        let __path = __entry.path();
                        if __path.is_file() && __path.extension().map_or(false, |ext| ext == "json")
                        {
                            __encode_functions.extend(gen_encode_fns(__path.clone()));
                            __encode_map_entries.extend(gen_encode_mappings(__path.clone()));
                            __encode_key_list_entries.extend(gen_encode_keys(__path.clone(), &mut __encode_key_list_size));
                        }
                    }
                    Err(_) => {
                        eprintln!("Could not generate encode functions and mappings");
                    }
                }
            }
        }
        Err(_) => {
            eprintln!("Could not read from directory");
        }
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
    match fs::File::open(_path) {
        Ok(mut _file) => {
            let mut _contents = String::new();
            let _ = _file.read_to_string(&mut _contents);
            let mut _msgs: Vec<CANMsg> = serde_json::from_str(&_contents).unwrap();
            let _fns = _msgs
                .iter_mut()
                .map(|_m| _m.gen_encoder_fn())
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
        Err(_) => {
            eprintln!("Error opening file");
            quote! { }
        }
    }
}

/**
 *  Helper function to generate encode phf map entries for a given JSON spec file
 */
fn gen_encode_mappings(_path: PathBuf) -> ProcMacro2TokenStream {
    match fs::File::open(_path) {
        Ok(mut _file) => {
            let mut _contents = String::new();
            let _ = _file.read_to_string(&mut _contents);
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
                    },
                    None => {
                        quote! { }
                    }
                };
                _entries.extend(_entry);
            }
            
            quote! {
                #_entries
            }
        }
        Err(_) => {
            eprintln!("Error opening file");
            quote! { }
        }
    }
}

/**
 *  Helper function to generate encode key list entries for a given JSON spec file
 */
fn gen_encode_keys(_path: PathBuf, _key_list_size: &mut usize) -> ProcMacro2TokenStream {
    match fs::File::open(_path) {
        Ok(mut _file) => {
            let mut _contents = String::new();
            let _ = _file.read_to_string(&mut _contents);
            let mut _msgs: Vec<CANMsg> = serde_json::from_str(&_contents).unwrap();
            let mut _entries = ProcMacro2TokenStream::new();
            for mut _msg in _msgs {
                let _entry = match &_msg.key {
                    Some(key) => {
                        *_key_list_size += 1;
                        quote! {
                            #key,
                        }
                    },
                    None => {
                        quote! { }
                    }
                };
                _entries.extend(_entry);
            }

            quote! {
                #_entries
            }
        }
        Err(_) => {
            eprintln!("Error opening file");
            quote! { }
        }
    }
}



/**
 *  Macro to generate all the code for simulate_data.rs
 *  - Generates prelude, all SimulatedComponentAttrs, and all 
 *    SimulatedComponents 
 */
#[proc_macro]
pub fn gen_simulate_data(_item: TokenStream) -> TokenStream {
    let __simulate_prelude = quote! {
        use crate::simulatable_message::{SimulatedComponent, SimulatedComponentAttr};
    };
    let mut __simulate_function_body = quote! { };

    match fs::read_dir(DAEDALUS_CANGEN_SPEC_PATH) {
        Ok(__entries) => {
            for __entry in __entries {
                match __entry {
                    Ok(__entry) => {
                        let __path = __entry.path();
                        if __path.is_file() && __path.extension().map_or(false, |ext| ext == "json")
                        {
                            __simulate_function_body.extend(gen_simulate_function_body(__path.clone()));
                        }
                    }
                    Err(_) => {
                        eprintln!("Could not generate simulate function");
                    }
                }
            }
        }
        Err(_) => {
            eprintln!("Could not read from directory");
        }
    }

    let __simulate_expanded = quote! {
        #__simulate_prelude

        pub fn create_simulated_components() -> Vec<SimulatedComponent> {
            let mut simulatable_messages: Vec<SimulatedComponent> = Vec::new();
            
            #__simulate_function_body

            simulatable_messages
        }
    };
    TokenStream::from(__simulate_expanded)
}

/**
 *  Helper function to generate simulate function body for a given JSON spec file
 */
fn gen_simulate_function_body(_path: PathBuf) -> ProcMacro2TokenStream {
    match fs::File::open(_path) {
        Ok(mut _file) => {
            let mut _contents = String::new();
            let _ = _file.read_to_string(&mut _contents);
            let mut _msgs: Vec<CANMsg> = serde_json::from_str(&_contents).unwrap();
            let mut _body = ProcMacro2TokenStream::new();
            
            for mut _msg in _msgs {
                let mut _extend = ProcMacro2TokenStream::new(); 
                match _msg.sim_freq {
                    Some(_freq) => {
                        for mut _field in _msg.fields {
                            let _simulatable: bool = 
                                _field.sim_min.is_some() &&
                                _field.sim_max.is_some() &&
                                _field.sim_inc_min.is_some() &&
                                _field.sim_inc_max.is_some();
                            if _simulatable {
                                let _attr_name = format_ident!(
                                    "{}_attr",
                                    _field.name.clone().to_lowercase().replace(&['/', ' ', '-'], "_")
                                );
                                let _sim_min: f32 = match _field.sim_min {
                                    Some(min) => min,
                                    None => -1.0
                                };
                                let _sim_max: f32 = match _field.sim_max {
                                    Some(max) => max,
                                    None => -1.0
                                };
                                let _sim_inc_min: f32 = match _field.sim_inc_min {
                                    Some(inc) => inc,
                                    None => -1.0
                                };
                                let _sim_inc_max: f32 = match _field.sim_inc_max {
                                    Some(inc) => inc,
                                    None => -1.0
                                };
                                let _n_canpoints: u32 = _field.points.len().try_into().unwrap();
                                let _id = _msg.id.clone();
                                let _component_name = format_ident!(
                                    "{}",
                                    _field.name.clone().to_lowercase().replace(&['/', ' ', '-'], "_")
                                );
                                let _name = _field.name.clone();
                                let _unit = _field.unit.clone();
                                let _component = quote! {
                                    let #_attr_name: SimulatedComponentAttr = SimulatedComponentAttr {
                                        sim_min: #_sim_min,
                                        sim_max: #_sim_max,
                                        sim_inc_min: #_sim_inc_min,
                                        sim_inc_max: #_sim_inc_max,
                                        sim_freq: #_freq,
                                        n_canpoints: #_n_canpoints,
                                        id: #_id.to_string(),
                                    };

                                    let #_component_name = SimulatedComponent::new(
                                        #_name.to_string(),
                                        #_unit.to_string(),
                                        #_attr_name
                                    );

                                    simulatable_messages.push(#_component_name);
                                };
                                _extend.extend(_component);
                            }
                        }
                    },
                    None => { }
                };

                _body.extend(_extend);
            }

            quote! {
                #_body
            }
        }
        Err(_) => {
            eprintln!("Error opening file");
            quote! { }
        }
    }
}
