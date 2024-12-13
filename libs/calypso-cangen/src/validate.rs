use crate::can_types::*;
use crate::CANGEN_SPEC_PATH;
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use thiserror::Error;

/**
 *  JSON spec error enum   
 */
#[derive(Error, Debug)]
pub enum CANSpecError {
    #[error("Message {0} description ({1}) contains illegal characters. Message descriptions may only contain letters and spaces.")]
    MessageDescIllegalChars(String, String),

    #[error("{0} totals to {1} bits. NetField totals should be byte-aligned (bit size should be a power of 2).")]
    FieldTotalByteMisalignment(String, usize),

    #[error("Sim frequencies ({1}, {2}) for NetField {0} do not add to 1. Sim enum frequencies must add up to 1.")]
    FieldSimEnumFrequencySum(String, f32, f32),

    #[error(
        "Signed point {0} of NetField {1} is {2} bits. Signed messages must be 8, 16, or 32 bits."
    )]
    PointSignedBitCount(usize, String, usize),

    #[error("Little-endian point {0} of NetField {1} is {2} bits. Little-endian messages must be 8, 16, or 32 bits.")]
    PointLittleEndianBitCount(usize, String, usize),

    #[error("IEEE754 float point {0} of NetField {1} is {2} bits, instead of 32 bits.")]
    PointFloatBitCount(usize, String, usize),
}

/**
 *  Validate all CAN spec files in CANGEN_SPEC_PATH
 */
pub fn validate_all_spec() -> Result<(), Vec<CANSpecError>> {
    match fs::read_dir(CANGEN_SPEC_PATH) {
        Ok(__entries) => {
            let mut __all_errors = Vec::new();
            for __entry in __entries {
                match __entry {
                    Ok(__entry) => {
                        let __path = __entry.path();
                        if __path.is_file() && __path.extension().map_or(false, |ext| ext == "json")
                        {
                            match validate_spec_file(__path.clone()) {
                                Ok(()) => {}
                                Err(__file_errors) => __all_errors.extend(__file_errors),
                            };
                        }
                    }
                    Err(_) => eprintln!("Error opening file"),
                }
            }

            if __all_errors.is_empty() {
                Ok(())
            } else {
                Err(__all_errors)
            }
        }
        Err(_) => {
            eprintln!("Could not read from directory");
            Ok(())
        }
    }
}

/**
 *  Validate a CAN spec file
 */
fn validate_spec_file(_path: PathBuf) -> Result<(), Vec<CANSpecError>> {
    match fs::File::open(_path) {
        Ok(mut _file) => {
            let mut _contents = String::new();
            let _ = _file.read_to_string(&mut _contents);
            let mut _errors = Vec::new();
            let _msgs: Vec<CANMsg> = serde_json::from_str(&_contents).unwrap();
            for _msg in _msgs {
                match validate_msg(_msg) {
                    Ok(()) => {}
                    Err(_msg_errors) => _errors.extend(_msg_errors),
                };
            }

            if _errors.is_empty() {
                Ok(())
            } else {
                Err(_errors)
            }
        }
        Err(_) => {
            eprintln!("Error opening file");
            Ok(())
        }
    }
}

/**
 *  Validate a CANMsg
 */
fn validate_msg(_msg: CANMsg) -> Result<(), Vec<CANSpecError>> {
    let mut _errors = Vec::new();

    // Check description contains legal chars
    let _desc = _msg.desc.clone();
    if !_desc
        .chars()
        .all(|c| c.is_alphabetic() || c.is_whitespace())
    {
        _errors.push(CANSpecError::MessageDescIllegalChars(
            _msg.id.clone(),
            _desc,
        ));
    }

    for _field in _msg.fields {
        let _name = _field.name.clone();

        // Check Sim enum frequencies add to 1
        if let Some(Sim::SimEnum { options }) = _field.sim {
            options.iter().for_each(|opt| {
                if opt[0] + opt[1] != 1.0 {
                    _errors.push(CANSpecError::FieldSimEnumFrequencySum(
                        _name.clone(),
                        opt[0],
                        opt[1],
                    ));
                }
            })
        }

        // Sum bit count of points for checks
        let mut _bit_count: usize = 0;

        for (_i, _point) in _field.points.iter().enumerate() {
            _bit_count += _point.size;

            // Check signed point bit count
            if let Some(true) = _point.signed {
                if _point.size != 8 && _point.size != 16 && _point.size != 32 {
                    _errors.push(CANSpecError::PointSignedBitCount(
                        _i,
                        _name.clone(),
                        _point.size,
                    ));
                }
            }

            // Check little endian point bit count
            // TODO fix
            if let Some(ref s) = _point.endianness {
                if s == "little" && _point.size != 8 && _point.size != 16 && _point.size != 32 {
                    _errors.push(CANSpecError::PointLittleEndianBitCount(
                        _i,
                        _name.clone(),
                        _point.size,
                    ));
                }
            }

            // Check IEEE754 f32 point bit count
            if let Some(true) = _point.ieee754_f32 {
                if _point.size != 32 {
                    _errors.push(CANSpecError::PointFloatBitCount(
                        _i,
                        _name.clone(),
                        _point.size,
                    ));
                }
            }
        }

        // Check field total alignment
        if _bit_count % 2 != 0 {
            _errors.push(CANSpecError::FieldTotalByteMisalignment(
                _name.clone(),
                _bit_count,
            ));
        }
    }

    if _errors.is_empty() {
        Ok(())
    } else {
        Err(_errors)
    }
}
