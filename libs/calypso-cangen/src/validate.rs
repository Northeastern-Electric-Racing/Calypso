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
    #[error("Message {0} description ({1}) contains illegal characters. Message descriptions may only contain letters and whitespace (_ included).")]
    MessageDescIllegalChars(String, String),

    #[error("Message {0} ({1}) totals to {2} bits. Message totals should be byte-aligned (bit size should be a power of 2).")]
    MessageTotalByteMisalignment(String, String, usize),

    #[error("NetField {0} references a value ({1}) that is out of bounds of the corresponding points list (max: {2})")]
    FieldValueOutOfBounds(String, usize, usize),

    // TODO implement
    #[error("NetField topic name {0} references a a value ({1}) that is out of bounds of the corresponding points list (max: {2})")]
    FieldInTopicValueOutOfBounds(String, usize, usize),

    #[error("Sim frequencies for Point {0} of Message {1} add to {2}. Sim enum frequencies must add up to 1.")]
    PointSimEnumFrequencySum(usize, String, f32),

    #[error("Point {0} of Message {1} is {2} bits. The maximum size for a point is 32 bits.")]
    PointSizeOverMax(usize, String, usize),

    #[error("Signed point {0} of Message {1} is {2} bits. Signed points must be 8, 16, or 32 bits.")]
    PointSignedBitCount(usize, String, usize),

    #[error("Little-endian point {0} of Message {1} is {2} bits. Little-endian points must be 8, 16, or 32 bits.")]
    PointLittleEndianBitCount(usize, String, usize),

    #[error("Point {0} of Message {1} specifies endianness and is {2} bits. Points with <=8 bits should not specify endianness.")]
    PointSmallSizeEndianness(usize, String, usize),

    #[error("IEEE754 float point {0} of Message {1} is {2} bits, instead of 32 bits.")]
    PointFloatBitCount(usize, String, usize),

    #[error(transparent)] // Pass-through for IO error
    IOError(#[from] std::io::Error),
}

/**
 *  Validate all CAN spec files in CANGEN_SPEC_PATH
 */
pub fn validate_all_spec() -> Result<(), Vec<CANSpecError>> {
    let mut __all_errors = Vec::new();
    match fs::read_dir(CANGEN_SPEC_PATH) {
        Ok(__entries) => {
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
fn validate_spec_file(_path: PathBuf) -> Result<(), Vec<CANSpecError>> {
    let mut _errors = Vec::new();
    match fs::File::open(_path) {
        Ok(mut _file) => {
            let mut _contents = String::new();
            let _ = _file.read_to_string(&mut _contents);
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
        Err(_err) => {
            _errors.push(_err.into());
            Err(_errors)
        }
    }
}

/**
 *  Validate a CANMsg
 */
fn validate_msg(_msg: CANMsg) -> Result<(), Vec<CANSpecError>> {
    let mut _errors = Vec::new();

    // Sum bit count of points for checks
    let mut _bit_count: usize = 0;

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

    for (_i, _point) in _msg.points.iter().enumerate(){
        _bit_count += _point.size;
        let _parse = !matches!(_point.parse, Some(false));

        // Check Sim enum frequencies add to 1 (roughly, f32s are approximate)
        if let Some(Sim::SimEnum { options }) = &_point.sim {
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

        // Check signed point bit count
        if let Some(true) = _point.signed {
            if _point.size != 8 && _point.size != 16 && _point.size != 32 {
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
            else if s == "little"
                && _point.size != 8
                && _point.size != 16
                && _point.size != 32
            {
                _errors.push(CANSpecError::PointLittleEndianBitCount(
                    _i,
                    _msg.id.clone(),
                    _point.size,
                ));
            }
        }

        // Check IEEE754 f32 point bit count
        if let Some(true) = _point.ieee754_f32 {
            if _point.size != 32 {
                _errors.push(CANSpecError::PointFloatBitCount(
                    _i,
                    _msg.id.clone(),
                    _point.size,
                ));
            }
        }
    }

    for _field in _msg.fields {
        // Check that field doesn't reference any OoB points
        for _value in _field.values {
            if _value <= 0 || _value > _msg.points.len() {
                _errors.push(CANSpecError::FieldValueOutOfBounds(
                    _field.name.clone(),
                    _value,
                    _msg.points.len(),
                ));
            }
        }

        // TODO: Check that field name doesn't reference any OoB points
        if false {
            _errors.push(CANSpecError::FieldInTopicValueOutOfBounds(
                _field.name.clone(),
                0,
                _msg.points.len(),
            ));
        }
    }

    // Check message total alignment
    if _bit_count % 8 != 0 {
        _errors.push(CANSpecError::MessageTotalByteMisalignment(
            _msg.id.clone(),
            _msg.desc.clone(),
            _bit_count,
        ));
    }

    // Propagate
    if _errors.is_empty() {
        Ok(())
    } else {
        Err(_errors)
    }
}
