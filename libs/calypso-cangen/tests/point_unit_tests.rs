/**
 *  Unit tests for CANPoint codegen
 */

#[cfg(test)]
mod tests {
    use calypso_cangen::can_types::CANPoint;
    use calypso_cangen::can_gen_decode::gen_decoder_point;
    use calypso_cangen::can_gen_encode::gen_encoder_point;
    use quote::quote;

    #[test]
    fn test_gen_decoder_point_default() {
        let index: usize = 0;
        let mut point = CANPoint {
            size: 1,
            parse: None,
            signed: None,
            endianness: None,
            format: None,
            default: None,
            ieee754_f32: None,
            sim: None,
        };

        assert_eq!(
            gen_decoder_point(index, &mut point).to_string(),
            quote! { 
                let point_1 = (reader.read_in::<1 ,u32>().unwrap() as f32); 
            }.to_string()
        );
    } 

    #[test]
    fn test_gen_decoder_point_parse_true() {
        let index: usize = 0;
        let mut point = CANPoint {
            size: 1,
            parse: Some(true),
            signed: None,
            endianness: None,
            format: None,
            default: None,
            ieee754_f32: None,
            sim: None,
        };

        assert_eq!(
            gen_decoder_point(index, &mut point).to_string(),
            quote! { 
                let point_1 = (reader.read_in::<1 ,u32>().unwrap() as f32); 
            }.to_string()
        );
    }

    #[test]
    fn test_gen_decoder_point_skip() {
        let index: usize = 0;
        let mut point = CANPoint {
            size: 1,
            parse: Some(false),
            signed: None,
            endianness: None,
            format: None,
            default: None,
            ieee754_f32: None,
            sim: None,
        };

        assert_eq!(
            gen_decoder_point(index, &mut point).to_string(),
            quote! { 
                reader.skip(1).unwrap(); 
            }.to_string()
        );
    }

    #[test]
    fn test_gen_decoder_point_signed() {
        let index: usize = 0;
        let mut point = CANPoint {
            size: 1,
            parse: None,
            signed: Some(true),
            endianness: None,
            format: None,
            default: None,
            ieee754_f32: None,
            sim: None,
        };

        assert_eq!(
            gen_decoder_point(index, &mut point).to_string(),
            quote! {
                let point_1 = (reader.read_signed_in::<1, i32>().unwrap() as f32);
            }.to_string()
        );
    }

    #[test]
    fn test_gen_decoder_point_unsigned_little_8b() {
        let index: usize = 0;
        let mut point = CANPoint {
            size: 8,
            parse: None,
            signed: Some(false),
            endianness: Some("little".to_string()),
            format: None,
            default: None,
            ieee754_f32: None,
            sim: None,
        };

        assert_eq!(
            gen_decoder_point(index, &mut point).to_string(),
            quote! { 
                let point_1 = (reader.read_as_to::<LittleEndian, u8>().unwrap() as f32);
            }.to_string()
        );
    }

    #[test]
    fn test_gen_decoder_point_signed_little_8b() {
        let index: usize = 0;
        let mut point = CANPoint {
            size: 8,
            parse: None,
            signed: Some(true),
            endianness: Some("little".to_string()),
            format: None,
            default: None,
            ieee754_f32: None,
            sim: None,
        };

        assert_eq!(
            gen_decoder_point(index, &mut point).to_string(),
            quote! { 
                let point_1 = (reader.read_as_to::<LittleEndian, i8>().unwrap() as f32);
            }.to_string()
        );
    }

    #[test]
    fn test_gen_decoder_point_unsigned_little_16b() {
        let index: usize = 0;
        let mut point = CANPoint {
            size: 16,
            parse: None,
            signed: Some(false),
            endianness: Some("little".to_string()),
            format: None,
            default: None,
            ieee754_f32: None,
            sim: None,
        };

        assert_eq!(
            gen_decoder_point(index, &mut point).to_string(),
            quote! { 
                let point_1 = (reader.read_as_to::<LittleEndian, u16>().unwrap() as f32);
            }.to_string()
        );
    }

    #[test]
    fn test_gen_decoder_point_signed_little_16b() {
        let index: usize = 0;
        let mut point = CANPoint {
            size: 16,
            parse: None,
            signed: Some(true),
            endianness: Some("little".to_string()),
            format: None,
            default: None,
            ieee754_f32: None,
            sim: None,
        };

        assert_eq!(
            gen_decoder_point(index, &mut point).to_string(),
            quote! { 
                let point_1 = (reader.read_as_to::<LittleEndian, i16>().unwrap() as f32);
            }.to_string()
        );
    }

    #[test]
    fn test_gen_decoder_point_unsigned_little_32b() {
        let index: usize = 0;
        let mut point = CANPoint {
            size: 32,
            parse: None,
            signed: Some(false),
            endianness: Some("little".to_string()),
            format: None,
            default: None,
            ieee754_f32: None,
            sim: None,
        };

        assert_eq!(
            gen_decoder_point(index, &mut point).to_string(),
            quote! { 
                let point_1 = (reader.read_as_to::<LittleEndian, u32>().unwrap() as f32);
            }.to_string()
        );
    }

    #[test]
    fn test_gen_decoder_point_signed_little_32b() {
        let index: usize = 0;
        let mut point = CANPoint {
            size: 32,
            parse: None,
            signed: Some(true),
            endianness: Some("little".to_string()),
            format: None,
            default: None,
            ieee754_f32: None,
            sim: None,
        };

        assert_eq!(
            gen_decoder_point(index, &mut point).to_string(),
            quote! { 
                let point_1 = (reader.read_as_to::<LittleEndian, i32>().unwrap() as f32);
            }.to_string()
        );
    }

    #[test]
    fn test_gen_decoder_point_little_f32() {
        let index: usize = 0;
        let mut point = CANPoint {
            size: 32,
            parse: None,
            signed: None, 
            endianness: Some("little".to_string()),
            format: None,
            default: None,
            ieee754_f32: Some(true),
            sim: None,
        };

        assert_eq!(
            gen_decoder_point(index, &mut point).to_string(), 
            quote! {
                let point_1 = (f32::from_bits(reader.read_as_to::<LittleEndian, u32>().unwrap()));
            }.to_string()
        );
    }

    #[test]
    fn test_gen_decoder_point_big_f32() {
        let index: usize = 0;
        let mut point = CANPoint {
            size: 32,
            parse: None,
            signed: None,
            endianness: None,
            format: None,
            default: None,
            ieee754_f32: Some(true),
            sim: None,
        };

        assert_eq!(
            gen_decoder_point(index, &mut point).to_string(),
            quote! {
                let point_1 = (f32::from_bits(reader.read_in::<32, u32>().unwrap()));
            }.to_string()
        );
    }

    #[test]
    fn test_gen_decoder_point_format() {
        let index: usize = 0;
        let mut point = CANPoint {
            size: 1,
            parse: None,
            signed: None,
            endianness: None,
            format: Some("divide10".to_string()),
            default: None,
            ieee754_f32: None,
            sim: None,
        };

        assert_eq!(
            gen_decoder_point(index, &mut point).to_string(),
            quote! {
                let point_1 = FormatData::divide10_d(reader.read_in::<1 ,u32>().unwrap() as f32);
            }.to_string()
        );
    }
}
