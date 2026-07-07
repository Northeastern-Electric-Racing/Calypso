use crate::CANGEN_SPEC_PATH;
use definition_rs::{BidirMode, OdysseyMsg, Sim};
use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use thiserror::Error;

/**
 *  JSON spec error enum
 */
#[derive(Error, Debug)]
pub enum CANSpecError {
    #[error(
        "Message {0} description ({1}) contains illegal characters. Message descriptions may only contain letters and whitespace (_ included)."
    )]
    MessageDescIllegalChars(String, String),

    #[error(
        "Message {0} ({1}) totals to {2} bits. Message totals should be byte-aligned (bit size should be a power of 2)."
    )]
    MessageTotalByteMisalignment(String, String, usize),

    #[error(
        "NetField {0} references a value ({1}) that is out of bounds of the corresponding points list (max: {2})"
    )]
    FieldValueOutOfBounds(String, usize, usize),

    #[error(
        "NetField topic name {0} references a a value ({1}) that is out of bounds of the corresponding points list (max: {2})"
    )]
    FieldInTopicValueOutOfBounds(String, usize, usize),

    #[error(
        "Sim frequencies for Point {0} of Message {1} add to {2}. Sim enum frequencies must add up to 1."
    )]
    PointSimEnumFrequencySum(usize, String, f32),

    #[error("Point {0} of Message {1} is {2} bits. The maximum size for a point is 32 bits.")]
    PointSizeOverMax(usize, String, usize),

    #[error(
        "Signed point {0} of Message {1} is {2} bits. Signed points must be 8, 16, or 32 bits."
    )]
    PointSignedBitCount(usize, String, usize),

    #[error(
        "Little-endian point {0} of Message {1} is {2} bits. Little-endian points must be 8, 16, or 32 bits."
    )]
    PointLittleEndianBitCount(usize, String, usize),

    #[error(
        "Message {0} contains both little endian points and points that are not byte aligned.  This is disallowed."
    )]
    MessageLittleEndianAndMisaligned(String),

    #[error(
        "Point {0} of Message {1} specifies endianness and is {2} bits. Points with <=8 bits should not specify endianness."
    )]
    PointSmallSizeEndianness(usize, String, usize),

    #[error("IEEE754 float point {0} of Message {1} is {2} bits, instead of 32 bits.")]
    PointFloatBitCount(usize, String, usize),

    #[error(
        "Point {0} of Message {1} specifies a default value, but the message is not a bidirectional broadcast message."
    )]
    BidirDefaultValueButOneshot(usize, String),

    #[error("Duplicate topic names {0}, last seen at message {1}")]
    DuplicateTopicNames(String, String),

    #[error("Invalid topic name {0}")]
    InvalidTopicName(String),

    #[error(transparent)] // Pass-through for IO error
    IOError(#[from] std::io::Error),
}

/**
 *  Validate all CAN spec files in `CANGEN_SPEC_PATH`
 */
pub fn validate_all_spec() -> Result<(), Vec<CANSpecError>> {
    let mut __all_errors = Vec::new();

    let mut _topics: HashSet<String> = HashSet::new();

    match fs::read_dir(CANGEN_SPEC_PATH) {
        Ok(__entries) => {
            for __entry in __entries {
                match __entry {
                    Ok(__entry) => {
                        let __path = __entry.path();
                        if __path.is_file() && __path.extension().is_some_and(|ext| ext == "json") {
                            match validate_spec_file(__path.clone(), &mut _topics) {
                                Ok(()) => {}
                                Err(__file_errors) => __all_errors.extend(__file_errors),
                            }
                        }
                    }
                    Err(__err) => __all_errors.push(__err.into()),
                }
            }

            if __all_errors.is_empty() {
                Ok(())
            } else {
                Err(__all_errors)
            }
        }
        Err(__err) => {
            __all_errors.push(__err.into());
            Err(__all_errors)
        }
    }
}

/**
 *  Validate a CAN spec file
 */
#[allow(clippy::expect_fun_call)]
fn validate_spec_file(
    _path: PathBuf,
    mut _topics: &mut HashSet<String>,
) -> Result<(), Vec<CANSpecError>> {
    let mut _errors = Vec::new();
    match fs::File::open(_path.clone()) {
        Ok(mut _file) => {
            let mut _contents = String::new();
            _file
                .read_to_string(&mut _contents)
                .expect(&format!("Could not read file {:?}", _path));

            // since untagged enums give shitty error messages
            let jd = &mut serde_json::Deserializer::from_str(&_contents);

            let result: Result<Vec<OdysseyMsg>, _> = serde_path_to_error::deserialize(jd);
            match result {
                Ok(_msgs) => {
                    for _msg in _msgs {
                        match validate_msg(_msg, _topics) {
                            Ok(()) => {}
                            Err(_msg_errors) => _errors.extend(_msg_errors),
                        }
                    }
                }
                Err(err) => {
                    let _err_path = err.path().to_string();
                    panic!(
                        "Could not deserialize message number {} at {:?}",
                        _err_path, _path
                    )
                }
            }

            if _errors.is_empty() {
                Ok(())
            } else {
                Err(_errors)
            }
        }
        Err(_err) => {
            _errors.push(_err.into());
            Err(_errors)
        }
    }
}

/**
 *  Validate a `CANMsg`
 */
fn validate_msg(
    _msg: OdysseyMsg,
    mut _topics: &mut HashSet<String>,
) -> Result<(), Vec<CANSpecError>> {
    let mut _errors = Vec::new();

    match _msg {
        OdysseyMsg::Can(_msg) => {
            // Sum bit count of points for checks
            let mut _bit_count: usize = 0;

            // Regex pattern for in-topic naming
            let _topic_regex_pattern = Regex::new(r"\{(\d+)\}").unwrap(); // Basically, digits enclosed in braces

            // Check description contains legal chars
            let _desc = _msg.desc.clone();
            if !_desc
                .chars()
                .all(|c| c.is_alphabetic() || c.is_whitespace() || c == '_')
            {
                _errors.push(CANSpecError::MessageDescIllegalChars(
                    _msg.id.clone(),
                    _desc,
                ));
            }

            let mut _is_byte_aligned = true; // if the whole message has only byte aligned points
            let mut _contains_little_endians = false; // if the message contains any little endian points

            for (_i, _point) in _msg.points.iter().enumerate() {
                _bit_count += _point.size;
                let _parse = !matches!(_point.parse, Some(false));

                // Check Sim enum frequencies add to 1 (roughly, f32s are approximate)
                if let Some(Sim::SimDiscrete { options }) = &_point.sim {
                    let mut _sim_total: f32 = 0.0;
                    options.iter().for_each(|opt| {
                        _sim_total += opt[1];
                    });
                    if (_sim_total - 1.0).abs() > 0.00001 {
                        _errors.push(CANSpecError::PointSimEnumFrequencySum(
                            _i,
                            _msg.id.clone(),
                            _sim_total,
                        ));
                    }
                }
                // if we arent byte aligned
                if _point.size != 8 && _point.size != 16 && _point.size != 32 {
                    _is_byte_aligned = false;
                    if let Some(true) = _point.signed {
                        _errors.push(CANSpecError::PointSignedBitCount(
                            _i,
                            _msg.id.clone(),
                            _point.size,
                        ));
                    }
                }

                // Check that point size is at most 32 bits
                if _point.size > 32 && _parse {
                    _errors.push(CANSpecError::PointSizeOverMax(
                        _i,
                        _msg.id.clone(),
                        _point.size,
                    ));
                    continue;
                }

                if let Some(ref s) = _point.endianness {
                    // Check that small points don't specify endianness
                    if _point.size <= 8 {
                        _errors.push(CANSpecError::PointSmallSizeEndianness(
                            _i,
                            _msg.id.clone(),
                            _point.size,
                        ));
                    }
                    // Check little endian point bit count
                    else if s == "little" {
                        _contains_little_endians = true;
                        if _point.size != 8 && _point.size != 16 && _point.size != 32 {
                            _errors.push(CANSpecError::PointLittleEndianBitCount(
                                _i,
                                _msg.id.clone(),
                                _point.size,
                            ));
                        }
                    }
                }

                // Check IEEE754 f32 point bit count
                if let Some(true) = _point.ieee754_f32
                    && _point.size != 32
                {
                    _errors.push(CANSpecError::PointFloatBitCount(
                        _i,
                        _msg.id.clone(),
                        _point.size,
                    ));
                }

                // Check that a user isnt using the default value it is useless
                if _point.default.is_some() {
                    match _msg.bidir_mode {
                        BidirMode::Broadcast => (),
                        BidirMode::Oneshot => _errors.push(
                            CANSpecError::BidirDefaultValueButOneshot(_i, _msg.id.clone()),
                        ),
                        BidirMode::Configuration => todo!(),
                    }
                }
            }

            for _field in _msg.fields {
                // Check that field doesn't reference any OoB points
                for _value in _field.values {
                    if _value == 0 || _value > _msg.points.len() {
                        _errors.push(CANSpecError::FieldValueOutOfBounds(
                            _field.name.clone(),
                            _value,
                            _msg.points.len(),
                        ));
                    }
                }

                // Check that field name doesn't reference any OoB points
                let _topic_format_value_indexes: Vec<usize> = _topic_regex_pattern
                    .captures_iter(&_field.name.clone())
                    .map(|cap| cap[1].parse::<usize>().unwrap())
                    .collect();
                for _value in _topic_format_value_indexes {
                    if _value == 0 || _value > _msg.points.len() {
                        _errors.push(CANSpecError::FieldInTopicValueOutOfBounds(
                            _field.name.clone(),
                            _value,
                            _msg.points.len(),
                        ));
                    }
                }

                if _topics.contains(&_field.name) {
                    _errors.push(CANSpecError::DuplicateTopicNames(
                        _field.name.clone(),
                        _msg.desc.clone(),
                    ));
                }

                // check topic contains valid chars
                if _field.name.chars().any(|c| "?#*.".contains(c)) {
                    _errors.push(CANSpecError::InvalidTopicName(_field.name.clone()));
                }

                _topics.insert(_field.name.clone());
            }

            // Check message total alfloatfloatignment
            if !_bit_count.is_multiple_of(8) || _bit_count == 0 {
                _errors.push(CANSpecError::MessageTotalByteMisalignment(
                    _msg.id.clone(),
                    _msg.desc.clone(),
                    _bit_count,
                ));
            }

            // check the little endian cannot be mixed with non-byte-alignment
            if !_is_byte_aligned && _contains_little_endians {
                // _errors.push(CANSpecError::MessageLittleEndianAndMisaligned(
                //     _msg.desc.clone(),
                // ))
                // TODO re-enable this check with the extra condition that this message has C GEN intended for it
            }
        }
        OdysseyMsg::Meta(_msg) => {
            for _field in _msg.fields {
                if _topics.contains(&_field.name) {
                    _errors.push(CANSpecError::DuplicateTopicNames(
                        _field.name.clone(),
                        _msg.desc.clone(),
                    ));
                }
                _topics.insert(_field.name.clone());
            }
        }
    }

    // Propagate
    if _errors.is_empty() {
        Ok(())
    } else {
        Err(_errors)
    }
}
