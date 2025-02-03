use std :: time :: Instant; use crate :: simulatable_message ::
{ SimComponent, SimValue, SimPoint }; pub fn get_hello_world() -> & 'static
str { "Hello, world!" } pub fn create_simulated_components() -> Vec <
SimComponent >
{
    let mut __all_sim_components : Vec < SimComponent > = Vec :: new(); let
    _____START_DEBUG_FIELD_NAME = "DTI/RPM/ERPM"; let mut vec_points : Vec <
    SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : -1000000f32, max : 1000000f32, inc_min : 1f32, inc_max : 1f32,
        round : false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 32usize, parse : None, signed : Some(true), endianness : None,
        format : None, default : None, ieee754_f32 : None, value :
        __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x416".to_string(), points : vec_points, points_intopic : None,
        topic : "DTI/RPM/ERPM".to_string(), unit : "ERPM".to_string(), name :
        "DTI/RPM/ERPM".to_string(), last_update : Instant :: now(), desc :
        "ERPM_Duty_Input_Voltage_Status".to_string(), key : None, is_ext :
        None, sim_freq : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "DTI/RPM/ERPM"; let
    _____START_DEBUG_FIELD_NAME = "DTI/Power/Duty_Cycle"; let mut vec_points :
    Vec < SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : -4000f32, max : 4000f32, inc_min : 1f32, inc_max : 1f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : Some(true), endianness : None,
        format : Some("divide10".to_string()), default : None, ieee754_f32 :
        None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x416".to_string(), points : vec_points, points_intopic : None,
        topic : "DTI/Power/Duty_Cycle".to_string(), unit : "%".to_string(),
        name : "DTI/Power/Duty_Cycle".to_string(), last_update : Instant ::
        now(), desc : "ERPM_Duty_Input_Voltage_Status".to_string(), key :
        None, is_ext : None, sim_freq : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "DTI/Power/Duty_Cycle"; let
    _____START_DEBUG_FIELD_NAME = "DTI/Power/Input_Voltage"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Range
    {
        min : 0f32, max : 505f32, inc_min : 0.01f32, inc_max : 10f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : Some(true), endianness : None,
        format : None, default : None, ieee754_f32 : None, value :
        __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x416".to_string(), points : vec_points, points_intopic : None,
        topic : "DTI/Power/Input_Voltage".to_string(), unit : "V".to_string(),
        name : "DTI/Power/Input_Voltage".to_string(), last_update : Instant ::
        now(), desc : "ERPM_Duty_Input_Voltage_Status".to_string(), key :
        None, is_ext : None, sim_freq : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "DTI/Power/Input_Voltage"; let
    _____START_DEBUG_FIELD_NAME = "DTI/FOC/Component_Id"; let mut vec_points :
    Vec < SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : -1000000f32, max : 1000000f32, inc_min : 1f32, inc_max : 1f32,
        round : false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 32usize, parse : None, signed : Some(true), endianness : None,
        format : Some("divide100".to_string()), default : None, ieee754_f32 :
        None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x476".to_string(), points : vec_points, points_intopic : None,
        topic : "DTI/FOC/Component_Id".to_string(), unit : "A".to_string(),
        name : "DTI/FOC/Component_Id".to_string(), last_update : Instant ::
        now(), desc : "MC_FOC_Algorithm".to_string(), key : None, is_ext :
        None, sim_freq : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "DTI/FOC/Component_Id"; let
    _____START_DEBUG_FIELD_NAME = "DTI/FOC/Component_Iq"; let mut vec_points :
    Vec < SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : -1000000f32, max : 1000000f32, inc_min : 1f32, inc_max : 1f32,
        round : false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 32usize, parse : None, signed : Some(true), endianness : None,
        format : Some("divide100".to_string()), default : None, ieee754_f32 :
        None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x476".to_string(), points : vec_points, points_intopic : None,
        topic : "DTI/FOC/Component_Iq".to_string(), unit : "A".to_string(),
        name : "DTI/FOC/Component_Iq".to_string(), last_update : Instant ::
        now(), desc : "MC_FOC_Algorithm".to_string(), key : None, is_ext :
        None, sim_freq : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "DTI/FOC/Component_Iq"; let
    _____START_DEBUG_FIELD_NAME = "MPU/Commands/AC_Current_Target"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Range
    {
        min : 0f32, max : 380f32, inc_min : 0.01f32, inc_max : 25f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : Some(true), endianness : None,
        format : Some("divide10".to_string()), default : None, ieee754_f32 :
        None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x036".to_string(), points : vec_points, points_intopic : None,
        topic : "MPU/Commands/AC_Current_Target".to_string(), unit :
        "A".to_string(), name : "MPU/Commands/AC_Current_Target".to_string(),
        last_update : Instant :: now(), desc :
        "AC_Current_Command".to_string(), key : None, is_ext : None, sim_freq
        : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MPU/Commands/AC_Current_Target"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Commands/Max_DC_Current_Target"; let
    mut vec_points : Vec < SimPoint > = Vec :: new(); let __new_value =
    SimValue :: Range
    {
        min : 0f32, max : 505f32, inc_min : 0.05f32, inc_max : 5f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : Some(true), endianness : None,
        format : Some("divide10".to_string()), default : None, ieee754_f32 :
        None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x156".to_string(), points : vec_points, points_intopic : None,
        topic : "BMS/Commands/Max_DC_Current_Target".to_string(), unit :
        "A".to_string(), name :
        "BMS/Commands/Max_DC_Current_Target".to_string(), last_update :
        Instant :: now(), desc : "Max_DC_Current_Command".to_string(), key :
        None, is_ext : None, sim_freq : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Commands/Max_DC_Current_Target"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Commands/Max_DC_Brake_Current_Target";
    let mut vec_points : Vec < SimPoint > = Vec :: new(); let __new_value =
    SimValue :: Range
    {
        min : -250f32, max : 0f32, inc_min : 0.01f32, inc_max : 10f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : Some(true), endianness : None,
        format : Some("divide10".to_string()), default : None, ieee754_f32 :
        None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x176".to_string(), points : vec_points, points_intopic : None,
        topic : "BMS/Commands/Max_DC_Brake_Current_Target".to_string(), unit :
        "A".to_string(), name :
        "BMS/Commands/Max_DC_Brake_Current_Target".to_string(), last_update :
        Instant :: now(), desc : "Max_DC_Brake_Current_Command".to_string(),
        key : None, is_ext : None, sim_freq : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Commands/Max_DC_Brake_Current_Target";
    let _____START_DEBUG_FIELD_NAME = "MPU/Commands/Drive_Enable_Target"; let
    mut vec_points : Vec < SimPoint > = Vec :: new(); let __new_value =
    SimValue :: Discrete
    { options : vec! [(0f32, 0.8f32), (1f32, 0.2f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x196".to_string(), points : vec_points, points_intopic : None,
        topic : "MPU/Commands/Drive_Enable_Target".to_string(), unit :
        "".to_string(), name : "MPU/Commands/Drive_Enable_Target".to_string(),
        last_update : Instant :: now(), desc :
        "Drive_Enable_Command".to_string(), key : None, is_ext : None,
        sim_freq : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MPU/Commands/Drive_Enable_Target"; let
    _____START_DEBUG_FIELD_NAME = "WHEEL/Buttons/1"; let mut vec_points : Vec
    < SimPoint > = Vec :: new(); let __new_value = SimValue :: Discrete
    {
        options : vec!
        [(0f32, 0.25f32), (1f32, 0.15f32), (2f32, 0.15f32), (3f32, 0.15f32),
        (4f32, 0.15f32), (5f32, 0.15f32)], current : 0.0
    }; let __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x680".to_string(), points : vec_points, points_intopic : None,
        topic : "WHEEL/Buttons/1".to_string(), unit : "".to_string(), name :
        "WHEEL/Buttons/1".to_string(), last_update : Instant :: now(), desc :
        "Wheel State".to_string(), key : None, is_ext : None, sim_freq : 8f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "WHEEL/Buttons/1"; let
    _____START_DEBUG_FIELD_NAME = "MSB/FL/Temp"; let mut vec_points : Vec <
    SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 0f32, max : 80f32, inc_min : 0.01f32, inc_max : 25f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x602".to_string(), points : vec_points, points_intopic : None,
        topic : "MSB/FL/Temp".to_string(), unit : "C".to_string(), name :
        "MSB/FL/Temp".to_string(), last_update : Instant :: now(), desc :
        "Front Left MSB Env".to_string(), key : None, is_ext : None, sim_freq
        : 500f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MSB/FL/Temp"; let
    _____START_DEBUG_FIELD_NAME = "MSB/FL/Humidity"; let mut vec_points : Vec
    < SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 20f32, max : 80f32, inc_min : 1f32, inc_max : 2.5f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x602".to_string(), points : vec_points, points_intopic : None,
        topic : "MSB/FL/Humidity".to_string(), unit : "".to_string(), name :
        "MSB/FL/Humidity".to_string(), last_update : Instant :: now(), desc :
        "Front Left MSB Env".to_string(), key : None, is_ext : None, sim_freq
        : 500f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MSB/FL/Humidity"; let
    _____START_DEBUG_FIELD_NAME = "MSB/FL/Accel"; let mut vec_points : Vec <
    SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : -2000f32, max : 2000f32, inc_min : 0.1f32, inc_max : 2.5f32,
        round : false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : Some(true), endianness : None,
        format : None, default : None, ieee754_f32 : None, value :
        __new_value,
    }; vec_points.push(__new_point); let __new_value = SimValue :: Range
    {
        min : -2000f32, max : 2000f32, inc_min : 0.1f32, inc_max : 2.5f32,
        round : false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : Some(true), endianness : None,
        format : None, default : None, ieee754_f32 : None, value :
        __new_value,
    }; vec_points.push(__new_point); let __new_value = SimValue :: Range
    {
        min : -2000f32, max : 2000f32, inc_min : 0.1f32, inc_max : 2.5f32,
        round : false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : Some(true), endianness : None,
        format : None, default : None, ieee754_f32 : None, value :
        __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x603".to_string(), points : vec_points, points_intopic : None,
        topic : "MSB/FL/Accel".to_string(), unit : "mg".to_string(), name :
        "MSB/FL/Accel".to_string(), last_update : Instant :: now(), desc :
        "Front Left MSB Accel".to_string(), key : None, is_ext : None,
        sim_freq : 500f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MSB/FL/Accel"; let
    _____START_DEBUG_FIELD_NAME = "MSB/FL/Gyro"; let mut vec_points : Vec <
    SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : -500000f32, max : 500000f32, inc_min : 10f32, inc_max : 1000f32,
        round : false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : Some(true), endianness : None,
        format : None, default : None, ieee754_f32 : None, value :
        __new_value,
    }; vec_points.push(__new_point); let __new_value = SimValue :: Range
    {
        min : -500000f32, max : 500000f32, inc_min : 10f32, inc_max : 1000f32,
        round : false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : Some(true), endianness : None,
        format : None, default : None, ieee754_f32 : None, value :
        __new_value,
    }; vec_points.push(__new_point); let __new_value = SimValue :: Range
    {
        min : -500000f32, max : 500000f32, inc_min : 10f32, inc_max : 1000f32,
        round : false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : Some(true), endianness : None,
        format : None, default : None, ieee754_f32 : None, value :
        __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x604".to_string(), points : vec_points, points_intopic : None,
        topic : "MSB/FL/Gyro".to_string(), unit : "mdps".to_string(), name :
        "MSB/FL/Gyro".to_string(), last_update : Instant :: now(), desc :
        "Front Left MSB Gyro".to_string(), key : None, is_ext : None, sim_freq
        : 500f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MSB/FL/Gyro"; let
    _____START_DEBUG_FIELD_NAME = "MSB/FL/Strain"; let mut vec_points : Vec <
    SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 0f32, max : 10f32, inc_min : 0.001f32, inc_max : 0.02f32, round
        : false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 32usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_value = SimValue :: Range
    {
        min : 0f32, max : 10f32, inc_min : 0.001f32, inc_max : 0.02f32, round
        : false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 32usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x605".to_string(), points : vec_points, points_intopic : None,
        topic : "MSB/FL/Strain".to_string(), unit : "".to_string(), name :
        "MSB/FL/Strain".to_string(), last_update : Instant :: now(), desc :
        "Front Left MSB Strain".to_string(), key : None, is_ext : None,
        sim_freq : 500f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MSB/FL/Strain"; let
    _____START_DEBUG_FIELD_NAME = "MSB/FL/Shock"; let mut vec_points : Vec <
    SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 0f32, max : 10f32, inc_min : 0.001f32, inc_max : 0.02f32, round
        : false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 32usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x606".to_string(), points : vec_points, points_intopic : None,
        topic : "MSB/FL/Shock".to_string(), unit : "".to_string(), name :
        "MSB/FL/Shock".to_string(), last_update : Instant :: now(), desc :
        "Front Left Shockpot".to_string(), key : None, is_ext : None, sim_freq
        : 500f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MSB/FL/Shock"; let
    _____START_DEBUG_FIELD_NAME = "MSB/FL/RideHeight"; let mut vec_points :
    Vec < SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 0f32, max : 10f32, inc_min : 0.001f32, inc_max : 0.02f32, round
        : false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : Some(true), endianness : None,
        format : None, default : None, ieee754_f32 : None, value :
        __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x607".to_string(), points : vec_points, points_intopic : None,
        topic : "MSB/FL/RideHeight".to_string(), unit : "".to_string(), name :
        "MSB/FL/RideHeight".to_string(), last_update : Instant :: now(), desc
        : "Front Left Ride Height".to_string(), key : None, is_ext : None,
        sim_freq : 500f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MSB/FL/RideHeight"; let
    _____START_DEBUG_FIELD_NAME = "MSB/FL/WheelTemp"; let mut vec_points : Vec
    < SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 0f32, max : 110f32, inc_min : 0.5f32, inc_max : 1.5f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x608".to_string(), points : vec_points, points_intopic : None,
        topic : "MSB/FL/WheelTemp".to_string(), unit : "C".to_string(), name :
        "MSB/FL/WheelTemp".to_string(), last_update : Instant :: now(), desc :
        "Front Left Wheel Temp".to_string(), key : None, is_ext : None,
        sim_freq : 500f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MSB/FL/WheelTemp"; let
    _____START_DEBUG_FIELD_NAME = "MSB/FR/Temp"; let mut vec_points : Vec <
    SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 0f32, max : 80f32, inc_min : 0.01f32, inc_max : 25f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x622".to_string(), points : vec_points, points_intopic : None,
        topic : "MSB/FR/Temp".to_string(), unit : "C".to_string(), name :
        "MSB/FR/Temp".to_string(), last_update : Instant :: now(), desc :
        "Front Right MSB Env".to_string(), key : None, is_ext : None, sim_freq
        : 500f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MSB/FR/Temp"; let
    _____START_DEBUG_FIELD_NAME = "MSB/FR/Humidity"; let mut vec_points : Vec
    < SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 0f32, max : 80f32, inc_min : 0.01f32, inc_max : 25f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x622".to_string(), points : vec_points, points_intopic : None,
        topic : "MSB/FR/Humidity".to_string(), unit : "".to_string(), name :
        "MSB/FR/Humidity".to_string(), last_update : Instant :: now(), desc :
        "Front Right MSB Env".to_string(), key : None, is_ext : None, sim_freq
        : 500f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MSB/FR/Humidity"; let
    _____START_DEBUG_FIELD_NAME = "MSB/FR/Accel"; let mut vec_points : Vec <
    SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : -2000f32, max : 2000f32, inc_min : 0.1f32, inc_max : 2.5f32,
        round : false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : Some(true), endianness : None,
        format : None, default : None, ieee754_f32 : None, value :
        __new_value,
    }; vec_points.push(__new_point); let __new_value = SimValue :: Range
    {
        min : -2000f32, max : 2000f32, inc_min : 0.1f32, inc_max : 2.5f32,
        round : false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : Some(true), endianness : None,
        format : None, default : None, ieee754_f32 : None, value :
        __new_value,
    }; vec_points.push(__new_point); let __new_value = SimValue :: Range
    {
        min : -2000f32, max : 2000f32, inc_min : 0.1f32, inc_max : 2.5f32,
        round : false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : Some(true), endianness : None,
        format : None, default : None, ieee754_f32 : None, value :
        __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x623".to_string(), points : vec_points, points_intopic : None,
        topic : "MSB/FR/Accel".to_string(), unit : "mg".to_string(), name :
        "MSB/FR/Accel".to_string(), last_update : Instant :: now(), desc :
        "Front Right MSB Accel".to_string(), key : None, is_ext : None,
        sim_freq : 500f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MSB/FR/Accel"; let
    _____START_DEBUG_FIELD_NAME = "MSB/FR/Gyro"; let mut vec_points : Vec <
    SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : -500000f32, max : 500000f32, inc_min : 10f32, inc_max : 1000f32,
        round : false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : Some(true), endianness : None,
        format : None, default : None, ieee754_f32 : None, value :
        __new_value,
    }; vec_points.push(__new_point); let __new_value = SimValue :: Range
    {
        min : -500000f32, max : 500000f32, inc_min : 10f32, inc_max : 1000f32,
        round : false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : Some(true), endianness : None,
        format : None, default : None, ieee754_f32 : None, value :
        __new_value,
    }; vec_points.push(__new_point); let __new_value = SimValue :: Range
    {
        min : -500000f32, max : 500000f32, inc_min : 10f32, inc_max : 1000f32,
        round : false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : Some(true), endianness : None,
        format : None, default : None, ieee754_f32 : None, value :
        __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x624".to_string(), points : vec_points, points_intopic : None,
        topic : "MSB/FR/Gyro".to_string(), unit : "mdps".to_string(), name :
        "MSB/FR/Gyro".to_string(), last_update : Instant :: now(), desc :
        "Front Right MSB Gyro".to_string(), key : None, is_ext : None,
        sim_freq : 500f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MSB/FR/Gyro"; let
    _____START_DEBUG_FIELD_NAME = "MSB/FR/Strain"; let mut vec_points : Vec <
    SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 0f32, max : 10f32, inc_min : 0.001f32, inc_max : 0.02f32, round
        : false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 32usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_value = SimValue :: Range
    {
        min : 0f32, max : 10f32, inc_min : 0.001f32, inc_max : 0.02f32, round
        : false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 32usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x625".to_string(), points : vec_points, points_intopic : None,
        topic : "MSB/FR/Strain".to_string(), unit : "".to_string(), name :
        "MSB/FR/Strain".to_string(), last_update : Instant :: now(), desc :
        "Front Right MSB Strain".to_string(), key : None, is_ext : None,
        sim_freq : 500f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MSB/FR/Strain"; let
    _____START_DEBUG_FIELD_NAME = "MSB/FR/Shock"; let mut vec_points : Vec <
    SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 0f32, max : 10f32, inc_min : 0.001f32, inc_max : 0.02f32, round
        : false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 32usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x626".to_string(), points : vec_points, points_intopic : None,
        topic : "MSB/FR/Shock".to_string(), unit : "".to_string(), name :
        "MSB/FR/Shock".to_string(), last_update : Instant :: now(), desc :
        "Front Right Shockpot".to_string(), key : None, is_ext : None,
        sim_freq : 500f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MSB/FR/Shock"; let
    _____START_DEBUG_FIELD_NAME = "MSB/FR/RideHeight"; let mut vec_points :
    Vec < SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 0f32, max : 10f32, inc_min : 0.001f32, inc_max : 0.02f32, round
        : false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : Some(true), endianness : None,
        format : None, default : None, ieee754_f32 : None, value :
        __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x627".to_string(), points : vec_points, points_intopic : None,
        topic : "MSB/FR/RideHeight".to_string(), unit : "".to_string(), name :
        "MSB/FR/RideHeight".to_string(), last_update : Instant :: now(), desc
        : "Front Right Ride Height".to_string(), key : None, is_ext : None,
        sim_freq : 500f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MSB/FR/RideHeight"; let
    _____START_DEBUG_FIELD_NAME = "MSB/FR/WheelTemp"; let mut vec_points : Vec
    < SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 0f32, max : 110f32, inc_min : 0.5f32, inc_max : 1.5f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x628".to_string(), points : vec_points, points_intopic : None,
        topic : "MSB/FR/WheelTemp".to_string(), unit : "C".to_string(), name :
        "MSB/FR/WheelTemp".to_string(), last_update : Instant :: now(), desc :
        "Front Right Wheel Temp".to_string(), key : None, is_ext : None,
        sim_freq : 500f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MSB/FR/WheelTemp"; let
    _____START_DEBUG_FIELD_NAME = "MSB/BL/Temp"; let mut vec_points : Vec <
    SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 0f32, max : 80f32, inc_min : 0.01f32, inc_max : 25f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x642".to_string(), points : vec_points, points_intopic : None,
        topic : "MSB/BL/Temp".to_string(), unit : "C".to_string(), name :
        "MSB/BL/Temp".to_string(), last_update : Instant :: now(), desc :
        "Back Left MSB Env".to_string(), key : None, is_ext : None, sim_freq :
        500f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MSB/BL/Temp"; let
    _____START_DEBUG_FIELD_NAME = "MSB/BL/Humidity"; let mut vec_points : Vec
    < SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 0f32, max : 80f32, inc_min : 0.01f32, inc_max : 25f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x642".to_string(), points : vec_points, points_intopic : None,
        topic : "MSB/BL/Humidity".to_string(), unit : "".to_string(), name :
        "MSB/BL/Humidity".to_string(), last_update : Instant :: now(), desc :
        "Back Left MSB Env".to_string(), key : None, is_ext : None, sim_freq :
        500f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MSB/BL/Humidity"; let
    _____START_DEBUG_FIELD_NAME = "MSB/BL/Accel"; let mut vec_points : Vec <
    SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : -2000f32, max : 2000f32, inc_min : 0.1f32, inc_max : 2.5f32,
        round : false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : Some(true), endianness : None,
        format : None, default : None, ieee754_f32 : None, value :
        __new_value,
    }; vec_points.push(__new_point); let __new_value = SimValue :: Range
    {
        min : -2000f32, max : 2000f32, inc_min : 0.1f32, inc_max : 2.5f32,
        round : false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : Some(true), endianness : None,
        format : None, default : None, ieee754_f32 : None, value :
        __new_value,
    }; vec_points.push(__new_point); let __new_value = SimValue :: Range
    {
        min : -2000f32, max : 2000f32, inc_min : 0.1f32, inc_max : 2.5f32,
        round : false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : Some(true), endianness : None,
        format : None, default : None, ieee754_f32 : None, value :
        __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x643".to_string(), points : vec_points, points_intopic : None,
        topic : "MSB/BL/Accel".to_string(), unit : "mg".to_string(), name :
        "MSB/BL/Accel".to_string(), last_update : Instant :: now(), desc :
        "Back Left MSB Accel".to_string(), key : None, is_ext : None, sim_freq
        : 500f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MSB/BL/Accel"; let
    _____START_DEBUG_FIELD_NAME = "MSB/BL/Gyro"; let mut vec_points : Vec <
    SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : -500000f32, max : 500000f32, inc_min : 10f32, inc_max : 1000f32,
        round : false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : Some(true), endianness : None,
        format : None, default : None, ieee754_f32 : None, value :
        __new_value,
    }; vec_points.push(__new_point); let __new_value = SimValue :: Range
    {
        min : -500000f32, max : 500000f32, inc_min : 10f32, inc_max : 1000f32,
        round : false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : Some(true), endianness : None,
        format : None, default : None, ieee754_f32 : None, value :
        __new_value,
    }; vec_points.push(__new_point); let __new_value = SimValue :: Range
    {
        min : -500000f32, max : 500000f32, inc_min : 10f32, inc_max : 1000f32,
        round : false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : Some(true), endianness : None,
        format : None, default : None, ieee754_f32 : None, value :
        __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x644".to_string(), points : vec_points, points_intopic : None,
        topic : "MSB/BL/Gyro".to_string(), unit : "mdps".to_string(), name :
        "MSB/BL/Gyro".to_string(), last_update : Instant :: now(), desc :
        "Back Left MSB Gyro".to_string(), key : None, is_ext : None, sim_freq
        : 500f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MSB/BL/Gyro"; let
    _____START_DEBUG_FIELD_NAME = "MSB/BL/Strain"; let mut vec_points : Vec <
    SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 0f32, max : 10f32, inc_min : 0.001f32, inc_max : 0.02f32, round
        : false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 32usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_value = SimValue :: Range
    {
        min : 0f32, max : 10f32, inc_min : 0.001f32, inc_max : 0.02f32, round
        : false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 32usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x645".to_string(), points : vec_points, points_intopic : None,
        topic : "MSB/BL/Strain".to_string(), unit : "".to_string(), name :
        "MSB/BL/Strain".to_string(), last_update : Instant :: now(), desc :
        "Back Left MSB Strain".to_string(), key : None, is_ext : None,
        sim_freq : 500f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MSB/BL/Strain"; let
    _____START_DEBUG_FIELD_NAME = "MSB/BL/Shock"; let mut vec_points : Vec <
    SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 0f32, max : 10f32, inc_min : 0.001f32, inc_max : 0.02f32, round
        : false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 32usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x646".to_string(), points : vec_points, points_intopic : None,
        topic : "MSB/BL/Shock".to_string(), unit : "".to_string(), name :
        "MSB/BL/Shock".to_string(), last_update : Instant :: now(), desc :
        "Back Left Shockpot".to_string(), key : None, is_ext : None, sim_freq
        : 500f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MSB/BL/Shock"; let
    _____START_DEBUG_FIELD_NAME = "MSB/BL/RideHeight"; let mut vec_points :
    Vec < SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 0f32, max : 10f32, inc_min : 0.001f32, inc_max : 0.02f32, round
        : false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : Some(true), endianness : None,
        format : None, default : None, ieee754_f32 : None, value :
        __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x647".to_string(), points : vec_points, points_intopic : None,
        topic : "MSB/BL/RideHeight".to_string(), unit : "".to_string(), name :
        "MSB/BL/RideHeight".to_string(), last_update : Instant :: now(), desc
        : "Front Back Ride Height".to_string(), key : None, is_ext : None,
        sim_freq : 500f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MSB/BL/RideHeight"; let
    _____START_DEBUG_FIELD_NAME = "MSB/BL/WheelTemp"; let mut vec_points : Vec
    < SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 0f32, max : 110f32, inc_min : 0.5f32, inc_max : 1.5f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x648".to_string(), points : vec_points, points_intopic : None,
        topic : "MSB/BL/WheelTemp".to_string(), unit : "C".to_string(), name :
        "MSB/BL/WheelTemp".to_string(), last_update : Instant :: now(), desc :
        "Back Left Wheel Temp".to_string(), key : None, is_ext : None,
        sim_freq : 500f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MSB/BL/WheelTemp"; let
    _____START_DEBUG_FIELD_NAME = "MSB/BR/Temp"; let mut vec_points : Vec <
    SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 0f32, max : 80f32, inc_min : 0.01f32, inc_max : 25f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x662".to_string(), points : vec_points, points_intopic : None,
        topic : "MSB/BR/Temp".to_string(), unit : "C".to_string(), name :
        "MSB/BR/Temp".to_string(), last_update : Instant :: now(), desc :
        "Back Right MSB Env".to_string(), key : None, is_ext : None, sim_freq
        : 500f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MSB/BR/Temp"; let
    _____START_DEBUG_FIELD_NAME = "MSB/BR/Humidity"; let mut vec_points : Vec
    < SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 0f32, max : 80f32, inc_min : 0.01f32, inc_max : 25f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x662".to_string(), points : vec_points, points_intopic : None,
        topic : "MSB/BR/Humidity".to_string(), unit : "".to_string(), name :
        "MSB/BR/Humidity".to_string(), last_update : Instant :: now(), desc :
        "Back Right MSB Env".to_string(), key : None, is_ext : None, sim_freq
        : 500f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MSB/BR/Humidity"; let
    _____START_DEBUG_FIELD_NAME = "MSB/BR/Accel"; let mut vec_points : Vec <
    SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : -2000f32, max : 2000f32, inc_min : 0.1f32, inc_max : 2.5f32,
        round : false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : Some(true), endianness : None,
        format : None, default : None, ieee754_f32 : None, value :
        __new_value,
    }; vec_points.push(__new_point); let __new_value = SimValue :: Range
    {
        min : -2000f32, max : 2000f32, inc_min : 0.1f32, inc_max : 2.5f32,
        round : false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : Some(true), endianness : None,
        format : None, default : None, ieee754_f32 : None, value :
        __new_value,
    }; vec_points.push(__new_point); let __new_value = SimValue :: Range
    {
        min : -2000f32, max : 2000f32, inc_min : 0.1f32, inc_max : 2.5f32,
        round : false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : Some(true), endianness : None,
        format : None, default : None, ieee754_f32 : None, value :
        __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x663".to_string(), points : vec_points, points_intopic : None,
        topic : "MSB/BR/Accel".to_string(), unit : "mg".to_string(), name :
        "MSB/BR/Accel".to_string(), last_update : Instant :: now(), desc :
        "Back Right MSB Accel".to_string(), key : None, is_ext : None,
        sim_freq : 500f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MSB/BR/Accel"; let
    _____START_DEBUG_FIELD_NAME = "MSB/BR/Gyro"; let mut vec_points : Vec <
    SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : -500000f32, max : 500000f32, inc_min : 10f32, inc_max : 1000f32,
        round : false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : Some(true), endianness : None,
        format : None, default : None, ieee754_f32 : None, value :
        __new_value,
    }; vec_points.push(__new_point); let __new_value = SimValue :: Range
    {
        min : -500000f32, max : 500000f32, inc_min : 10f32, inc_max : 1000f32,
        round : false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : Some(true), endianness : None,
        format : None, default : None, ieee754_f32 : None, value :
        __new_value,
    }; vec_points.push(__new_point); let __new_value = SimValue :: Range
    {
        min : -500000f32, max : 500000f32, inc_min : 10f32, inc_max : 1000f32,
        round : false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : Some(true), endianness : None,
        format : None, default : None, ieee754_f32 : None, value :
        __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x664".to_string(), points : vec_points, points_intopic : None,
        topic : "MSB/BR/Gyro".to_string(), unit : "mdps".to_string(), name :
        "MSB/BR/Gyro".to_string(), last_update : Instant :: now(), desc :
        "Back Right MSB Gyro".to_string(), key : None, is_ext : None, sim_freq
        : 500f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MSB/BR/Gyro"; let
    _____START_DEBUG_FIELD_NAME = "MSB/BR/Strain"; let mut vec_points : Vec <
    SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 0f32, max : 10f32, inc_min : 0.001f32, inc_max : 0.02f32, round
        : false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 32usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_value = SimValue :: Range
    {
        min : 0f32, max : 10f32, inc_min : 0.001f32, inc_max : 0.02f32, round
        : false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 32usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x665".to_string(), points : vec_points, points_intopic : None,
        topic : "MSB/BR/Strain".to_string(), unit : "".to_string(), name :
        "MSB/BR/Strain".to_string(), last_update : Instant :: now(), desc :
        "Back Right MSB Strain".to_string(), key : None, is_ext : None,
        sim_freq : 500f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MSB/BR/Strain"; let
    _____START_DEBUG_FIELD_NAME = "MSB/BR/Shock"; let mut vec_points : Vec <
    SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 0f32, max : 10f32, inc_min : 0.001f32, inc_max : 0.02f32, round
        : false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 32usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x666".to_string(), points : vec_points, points_intopic : None,
        topic : "MSB/BR/Shock".to_string(), unit : "".to_string(), name :
        "MSB/BR/Shock".to_string(), last_update : Instant :: now(), desc :
        "Back Right Shockpot".to_string(), key : None, is_ext : None, sim_freq
        : 500f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MSB/BR/Shock"; let
    _____START_DEBUG_FIELD_NAME = "MSB/BR/RideHeight"; let mut vec_points :
    Vec < SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 0f32, max : 10f32, inc_min : 0.001f32, inc_max : 0.02f32, round
        : false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : Some(true), endianness : None,
        format : None, default : None, ieee754_f32 : None, value :
        __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x667".to_string(), points : vec_points, points_intopic : None,
        topic : "MSB/BR/RideHeight".to_string(), unit : "".to_string(), name :
        "MSB/BR/RideHeight".to_string(), last_update : Instant :: now(), desc
        : "Back Right Ride Height".to_string(), key : None, is_ext : None,
        sim_freq : 500f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MSB/BR/RideHeight"; let
    _____START_DEBUG_FIELD_NAME = "MSB/BR/WheelTemp"; let mut vec_points : Vec
    < SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 0f32, max : 110f32, inc_min : 0.5f32, inc_max : 1.5f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x668".to_string(), points : vec_points, points_intopic : None,
        topic : "MSB/BR/WheelTemp".to_string(), unit : "C".to_string(), name :
        "MSB/BR/WheelTemp".to_string(), last_update : Instant :: now(), desc :
        "Back Right Wheel Temp".to_string(), key : None, is_ext : None,
        sim_freq : 500f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MSB/BR/WheelTemp"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Pack/PerCell/Cell-{6}/Attr/{3}"; let
    mut vec_points_in_topic : Vec < SimPoint > = Vec :: new(); let __new_value
    = SimValue :: Range
    {
        min : 1f32, max : 15f32, inc_min : 1f32, inc_max : 5f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points_in_topic.push(__new_point); let __new_value = SimValue ::
    Range
    {
        min : 1f32, max : 15f32, inc_min : 1f32, inc_max : 5f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points_in_topic.push(__new_point); let mut vec_points : Vec <
    SimPoint > = Vec :: new(); let __new_value = SimValue :: Discrete
    { options : vec! [(0f32, 0.5f32), (1f32, 0.5f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_value = SimValue :: Discrete
    { options : vec! [(0f32, 0.5f32), (1f32, 0.5f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_value = SimValue :: Discrete
    { options : vec! [(0f32, 0.5f32), (1f32, 0.5f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_value = SimValue :: Discrete
    { options : vec! [(0f32, 0.5f32), (1f32, 0.5f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x1337".to_string(), points : vec_points, points_intopic : None,
        topic : "BMS/Pack/PerCell/Cell-{6}/Attr/{3}".to_string(), unit :
        "V".to_string(), name :
        "BMS/Pack/PerCell/Cell-{6}/Attr/{3}".to_string(), last_update :
        Instant :: now(), desc :
        "Experimental Segment Temperatures".to_string(), key : None, is_ext :
        None, sim_freq : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Pack/PerCell/Cell-{6}/Attr/{3}"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Pack/PerCell/Cell-{6}/Attr/{3}"; let
    mut vec_points_in_topic : Vec < SimPoint > = Vec :: new(); let __new_value
    = SimValue :: Range
    {
        min : 1f32, max : 15f32, inc_min : 1f32, inc_max : 5f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points_in_topic.push(__new_point); let __new_value = SimValue ::
    Range
    {
        min : 1f32, max : 15f32, inc_min : 1f32, inc_max : 5f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points_in_topic.push(__new_point); let mut vec_points : Vec <
    SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : -100f32, max : 300f32, inc_min : 1f32, inc_max : 5f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : Some(true), endianness :
        Some("little".to_string()), format : Some("divide100".to_string()),
        default : Some(-20f32), ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x1337".to_string(), points : vec_points, points_intopic : None,
        topic : "BMS/Pack/PerCell/Cell-{6}/Attr/{3}".to_string(), unit :
        "Ah".to_string(), name :
        "BMS/Pack/PerCell/Cell-{6}/Attr/{3}".to_string(), last_update :
        Instant :: now(), desc :
        "Experimental Segment Temperatures".to_string(), key : None, is_ext :
        None, sim_freq : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Pack/PerCell/Cell-{6}/Attr/{3}"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Pack/PerCell/Cell-{5}/Attr/{4}"; let
    mut vec_points_in_topic : Vec < SimPoint > = Vec :: new(); let __new_value
    = SimValue :: Range
    {
        min : 1f32, max : 15f32, inc_min : 1f32, inc_max : 5f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points_in_topic.push(__new_point); let __new_value = SimValue ::
    Range
    {
        min : 1f32, max : 15f32, inc_min : 1f32, inc_max : 5f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points_in_topic.push(__new_point); let mut vec_points : Vec <
    SimPoint > = Vec :: new(); let __new_value = SimValue :: Discrete
    { options : vec! [(0f32, 0.5f32), (1f32, 0.5f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_value = SimValue :: Discrete
    { options : vec! [(0f32, 0.5f32), (1f32, 0.5f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_value = SimValue :: Discrete
    { options : vec! [(0f32, 0.5f32), (1f32, 0.5f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_value = SimValue :: Discrete
    { options : vec! [(0f32, 0.5f32), (1f32, 0.5f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x1337".to_string(), points : vec_points, points_intopic : None,
        topic : "BMS/Pack/PerCell/Cell-{5}/Attr/{4}".to_string(), unit :
        "V".to_string(), name :
        "BMS/Pack/PerCell/Cell-{5}/Attr/{4}".to_string(), last_update :
        Instant :: now(), desc :
        "Experimental Segment Temperatures".to_string(), key : None, is_ext :
        None, sim_freq : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Pack/PerCell/Cell-{5}/Attr/{4}"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Pack/PerCell/Cell-{5}/Attr/{4}"; let
    mut vec_points_in_topic : Vec < SimPoint > = Vec :: new(); let __new_value
    = SimValue :: Range
    {
        min : 1f32, max : 15f32, inc_min : 1f32, inc_max : 5f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points_in_topic.push(__new_point); let __new_value = SimValue ::
    Range
    {
        min : 1f32, max : 15f32, inc_min : 1f32, inc_max : 5f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points_in_topic.push(__new_point); let mut vec_points : Vec <
    SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : -50f32, max : 50f32, inc_min : 0.5f32, inc_max : 1.5f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : Some(true), endianness : None,
        format : None, default : None, ieee754_f32 : None, value :
        __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x1337".to_string(), points : vec_points, points_intopic : None,
        topic : "BMS/Pack/PerCell/Cell-{5}/Attr/{4}".to_string(), unit :
        "Ah".to_string(), name :
        "BMS/Pack/PerCell/Cell-{5}/Attr/{4}".to_string(), last_update :
        Instant :: now(), desc :
        "Experimental Segment Temperatures".to_string(), key : None, is_ext :
        None, sim_freq : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Pack/PerCell/Cell-{5}/Attr/{4}"; let
    _____START_DEBUG_FIELD_NAME = "MPU/State/Mode"; let mut vec_points : Vec <
    SimPoint > = Vec :: new(); let __new_value = SimValue :: Discrete
    {
        options : vec!
        [(0f32, 0.01f32), (1f32, 0.78f32), (2f32, 0.1f32), (3f32, 0.1f32),
        (4f32, 0.01f32)], current : 0.0
    }; let __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x501".to_string(), points : vec_points, points_intopic : None,
        topic : "MPU/State/Mode".to_string(), unit : "".to_string(), name :
        "MPU/State/Mode".to_string(), last_update : Instant :: now(), desc :
        "MPU Status".to_string(), key : None, is_ext : None, sim_freq :
        250f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MPU/State/Mode"; let
    _____START_DEBUG_FIELD_NAME = "MPU/State/ModeIndex"; let mut vec_points :
    Vec < SimPoint > = Vec :: new(); let __new_value = SimValue :: Discrete
    {
        options : vec!
        [(0f32, 0.2f32), (1f32, 0.2f32), (2f32, 0.2f32), (3f32, 0.1f32),
        (4f32, 0.1f32), (5f32, 0.1f32), (6f32, 0.1f32)], current : 0.0
    }; let __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x501".to_string(), points : vec_points, points_intopic : None,
        topic : "MPU/State/ModeIndex".to_string(), unit : "".to_string(), name
        : "MPU/State/ModeIndex".to_string(), last_update : Instant :: now(),
        desc : "MPU Status".to_string(), key : None, is_ext : None, sim_freq :
        250f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MPU/State/ModeIndex"; let
    _____START_DEBUG_FIELD_NAME = "MPU/State/Speed"; let mut vec_points : Vec
    < SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 0f32, max : 88f32, inc_min : 0f32, inc_max : 2f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : Some(true), endianness : None,
        format : None, default : None, ieee754_f32 : None, value :
        __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x501".to_string(), points : vec_points, points_intopic : None,
        topic : "MPU/State/Speed".to_string(), unit : "mph".to_string(), name
        : "MPU/State/Speed".to_string(), last_update : Instant :: now(), desc
        : "MPU Status".to_string(), key : None, is_ext : None, sim_freq :
        250f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MPU/State/Speed"; let
    _____START_DEBUG_FIELD_NAME = "MPU/State/TSMS"; let mut vec_points : Vec <
    SimPoint > = Vec :: new(); let __new_value = SimValue :: Discrete
    { options : vec! [(0f32, 0.8f32), (1f32, 0.2f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x501".to_string(), points : vec_points, points_intopic : None,
        topic : "MPU/State/TSMS".to_string(), unit : "".to_string(), name :
        "MPU/State/TSMS".to_string(), last_update : Instant :: now(), desc :
        "MPU Status".to_string(), key : None, is_ext : None, sim_freq :
        250f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MPU/State/TSMS"; let
    _____START_DEBUG_FIELD_NAME = "MPU/State/TorqueLimit"; let mut vec_points
    : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 0f32, max : 1f32, inc_min : 0.1f32, inc_max : 0.1f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : None, endianness : None, format
        : Some("divide100".to_string()), default : None, ieee754_f32 : None,
        value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x501".to_string(), points : vec_points, points_intopic : None,
        topic : "MPU/State/TorqueLimit".to_string(), unit :
        "percentage".to_string(), name : "MPU/State/TorqueLimit".to_string(),
        last_update : Instant :: now(), desc : "MPU Status".to_string(), key :
        None, is_ext : None, sim_freq : 250f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MPU/State/TorqueLimit"; let
    _____START_DEBUG_FIELD_NAME = "MPU/Sense/Accel"; let mut vec_points : Vec
    < SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 0f32, max : 2f32, inc_min : 0.01f32, inc_max : 0.25f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_value = SimValue :: Range
    {
        min : 0f32, max : 2f32, inc_min : 0.01f32, inc_max : 0.25f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_value = SimValue :: Range
    {
        min : 0f32, max : 2f32, inc_min : 0.01f32, inc_max : 0.25f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x500".to_string(), points : vec_points, points_intopic : None,
        topic : "MPU/Sense/Accel".to_string(), unit : "g".to_string(), name :
        "MPU/Sense/Accel".to_string(), last_update : Instant :: now(), desc :
        "MPU Sense Acceleromter".to_string(), key : None, is_ext : None,
        sim_freq : 250f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MPU/Sense/Accel"; let
    _____START_DEBUG_FIELD_NAME = "MPU/Sense/Gyro"; let mut vec_points : Vec <
    SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 0f32, max : 7f32, inc_min : 0.01f32, inc_max : 0.5f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_value = SimValue :: Range
    {
        min : 0f32, max : 7f32, inc_min : 0.01f32, inc_max : 0.5f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_value = SimValue :: Range
    {
        min : 0f32, max : 7f32, inc_min : 0.01f32, inc_max : 0.5f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x506".to_string(), points : vec_points, points_intopic : None,
        topic : "MPU/Sense/Gyro".to_string(), unit : "".to_string(), name :
        "MPU/Sense/Gyro".to_string(), last_update : Instant :: now(), desc :
        "MPU Sense Gyro".to_string(), key : None, is_ext : None, sim_freq :
        250f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MPU/Sense/Gyro"; let
    _____START_DEBUG_FIELD_NAME = "MPU/Sense/Temp_IMU"; let mut vec_points :
    Vec < SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 0f32, max : 70f32, inc_min : 1f32, inc_max : 2f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : Some(true), endianness : None,
        format : None, default : None, ieee754_f32 : None, value :
        __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x507".to_string(), points : vec_points, points_intopic : None,
        topic : "MPU/Sense/Temp_IMU".to_string(), unit : "C".to_string(), name
        : "MPU/Sense/Temp_IMU".to_string(), last_update : Instant :: now(),
        desc : "MPU Sense Temp".to_string(), key : None, is_ext : None,
        sim_freq : 1000f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MPU/Sense/Temp_IMU"; let
    _____START_DEBUG_FIELD_NAME = "MPU/Fault/Code"; let mut vec_points : Vec <
    SimPoint > = Vec :: new(); let __new_value = SimValue :: Discrete
    {
        options : vec!
        [(0f32, 0.84f32), (1f32, 0.01f32), (2f32, 0.01f32), (4f32, 0.01f32),
        (8f32, 0.01f32), (16f32, 0.01f32), (32f32, 0.01f32), (64f32, 0.01f32),
        (128f32, 0.01f32), (256f32, 0.01f32), (512f32, 0.01f32),
        (1024f32, 0.01f32), (2048f32, 0.01f32), (3840f32, 0.01f32),
        (4096f32, 0.01f32), (8192f32, 0.01f32), (16384f32, 0.01f32)], current
        : 0.0
    }; let __new_point = SimPoint
    {
        size : 32usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x502".to_string(), points : vec_points, points_intopic : None,
        topic : "MPU/Fault/Code".to_string(), unit : "".to_string(), name :
        "MPU/Fault/Code".to_string(), last_update : Instant :: now(), desc :
        "MPU Fault".to_string(), key : None, is_ext : None, sim_freq : 250f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MPU/Fault/Code"; let
    _____START_DEBUG_FIELD_NAME = "MPU/Fault/Severity"; let mut vec_points :
    Vec < SimPoint > = Vec :: new(); let __new_value = SimValue :: Discrete
    {
        options : vec!
        [(0f32, 0.75f32), (1f32, 0.05f32), (2f32, 0.05f32), (3f32, 0.05f32),
        (4f32, 0.05f32), (5f32, 0.05f32)], current : 0.0
    }; let __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x502".to_string(), points : vec_points, points_intopic : None,
        topic : "MPU/Fault/Severity".to_string(), unit : "".to_string(), name
        : "MPU/Fault/Severity".to_string(), last_update : Instant :: now(),
        desc : "MPU Fault".to_string(), key : None, is_ext : None, sim_freq :
        250f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MPU/Fault/Severity"; let
    _____START_DEBUG_FIELD_NAME = "MPU/Sense/Voltage"; let mut vec_points :
    Vec < SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 22.5f32, max : 29f32, inc_min : 0.02f32, inc_max : 0.15f32,
        round : false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 32usize, parse : None, signed : None, endianness :
        Some("little".to_string()), format : Some("divide10000".to_string()),
        default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x503".to_string(), points : vec_points, points_intopic : None,
        topic : "MPU/Sense/Voltage".to_string(), unit : "V".to_string(), name
        : "MPU/Sense/Voltage".to_string(), last_update : Instant :: now(),
        desc : "MPU Sense Voltage".to_string(), key : None, is_ext : None,
        sim_freq : 1000f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MPU/Sense/Voltage"; let
    _____START_DEBUG_FIELD_NAME = "MPU/Sense/SOC"; let mut vec_points : Vec <
    SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 5f32, max : 99f32, inc_min : 0.1f32, inc_max : 0.5f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x503".to_string(), points : vec_points, points_intopic : None,
        topic : "MPU/Sense/SOC".to_string(), unit : "%".to_string(), name :
        "MPU/Sense/SOC".to_string(), last_update : Instant :: now(), desc :
        "MPU Sense Voltage".to_string(), key : None, is_ext : None, sim_freq :
        1000f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MPU/Sense/SOC"; let
    _____START_DEBUG_FIELD_NAME = "MPU/Pedals/Accelerator_1"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Range
    {
        min : 800f32, max : 1900f32, inc_min : 1f32, inc_max : 24f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 32usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x504".to_string(), points : vec_points, points_intopic : None,
        topic : "MPU/Pedals/Accelerator_1".to_string(), unit : "".to_string(),
        name : "MPU/Pedals/Accelerator_1".to_string(), last_update : Instant
        :: now(), desc : "MPU Accel Pedals".to_string(), key : None, is_ext :
        None, sim_freq : 100f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MPU/Pedals/Accelerator_1"; let
    _____START_DEBUG_FIELD_NAME = "MPU/Pedals/Accelerator_2"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Range
    {
        min : 1200f32, max : 2300f32, inc_min : 1f32, inc_max : 24f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 32usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x504".to_string(), points : vec_points, points_intopic : None,
        topic : "MPU/Pedals/Accelerator_2".to_string(), unit : "".to_string(),
        name : "MPU/Pedals/Accelerator_2".to_string(), last_update : Instant
        :: now(), desc : "MPU Accel Pedals".to_string(), key : None, is_ext :
        None, sim_freq : 100f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MPU/Pedals/Accelerator_2"; let
    _____START_DEBUG_FIELD_NAME = "MPU/Pedals/Brake_1"; let mut vec_points :
    Vec < SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 580f32, max : 990f32, inc_min : 1f32, inc_max : 10f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 32usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x505".to_string(), points : vec_points, points_intopic : None,
        topic : "MPU/Pedals/Brake_1".to_string(), unit : "".to_string(), name
        : "MPU/Pedals/Brake_1".to_string(), last_update : Instant :: now(),
        desc : "MPU Brake Pedals".to_string(), key : None, is_ext : None,
        sim_freq : 100f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MPU/Pedals/Brake_1"; let
    _____START_DEBUG_FIELD_NAME = "MPU/Pedals/Brake_2"; let mut vec_points :
    Vec < SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 580f32, max : 990f32, inc_min : 1f32, inc_max : 10f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 32usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x505".to_string(), points : vec_points, points_intopic : None,
        topic : "MPU/Pedals/Brake_2".to_string(), unit : "".to_string(), name
        : "MPU/Pedals/Brake_2".to_string(), last_update : Instant :: now(),
        desc : "MPU Brake Pedals".to_string(), key : None, is_ext : None,
        sim_freq : 100f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MPU/Pedals/Brake_2"; let
    _____START_DEBUG_FIELD_NAME = "MPU/Debug/Spare0"; let mut vec_points : Vec
    < SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 580f32, max : 990f32, inc_min : 1f32, inc_max : 10f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x701".to_string(), points : vec_points, points_intopic : None,
        topic : "MPU/Debug/Spare0".to_string(), unit : "".to_string(), name :
        "MPU/Debug/Spare0".to_string(), last_update : Instant :: now(), desc :
        "MPU Debug".to_string(), key : None, is_ext : None, sim_freq :
        1000f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MPU/Debug/Spare0"; let
    _____START_DEBUG_FIELD_NAME = "MPU/Debug/Spare1"; let mut vec_points : Vec
    < SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 580f32, max : 990f32, inc_min : 1f32, inc_max : 10f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : Some(true), endianness : None,
        format : None, default : None, ieee754_f32 : None, value :
        __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x701".to_string(), points : vec_points, points_intopic : None,
        topic : "MPU/Debug/Spare1".to_string(), unit : "".to_string(), name :
        "MPU/Debug/Spare1".to_string(), last_update : Instant :: now(), desc :
        "MPU Debug".to_string(), key : None, is_ext : None, sim_freq :
        1000f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MPU/Debug/Spare1"; let
    _____START_DEBUG_FIELD_NAME = "MPU/Debug/Spare2"; let mut vec_points : Vec
    < SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 580f32, max : 990f32, inc_min : 1f32, inc_max : 10f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : None, endianness :
        Some("little".to_string()), format : None, default : None, ieee754_f32
        : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x701".to_string(), points : vec_points, points_intopic : None,
        topic : "MPU/Debug/Spare2".to_string(), unit : "".to_string(), name :
        "MPU/Debug/Spare2".to_string(), last_update : Instant :: now(), desc :
        "MPU Debug".to_string(), key : None, is_ext : None, sim_freq :
        1000f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MPU/Debug/Spare2"; let
    _____START_DEBUG_FIELD_NAME = "MPU/Debug/Spare3"; let mut vec_points : Vec
    < SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 580f32, max : 990f32, inc_min : 1f32, inc_max : 10f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 32usize, parse : None, signed : Some(true), endianness :
        Some("little".to_string()), format : None, default : None, ieee754_f32
        : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x701".to_string(), points : vec_points, points_intopic : None,
        topic : "MPU/Debug/Spare3".to_string(), unit : "".to_string(), name :
        "MPU/Debug/Spare3".to_string(), last_update : Instant :: now(), desc :
        "MPU Debug".to_string(), key : None, is_ext : None, sim_freq :
        1000f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MPU/Debug/Spare3"; let
    _____START_DEBUG_FIELD_NAME = "Calypso/Bidir/State/FirstOff/A"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Range
    {
        min : 1f32, max : 100f32, inc_min : 1f32, inc_max : 2f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 32usize, parse : None, signed : None, endianness : None, format
        : Some("divide10000".to_string()), default : Some(18.443f32),
        ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x450".to_string(), points : vec_points, points_intopic : None,
        topic : "Calypso/Bidir/State/FirstOff/A".to_string(), unit :
        "Z".to_string(), name : "Calypso/Bidir/State/FirstOff/A".to_string(),
        last_update : Instant :: now(), desc : "Example Enc Msg".to_string(),
        key : Some("FirstOff".to_string()), is_ext : None, sim_freq : 750f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "Calypso/Bidir/State/FirstOff/A"; let
    _____START_DEBUG_FIELD_NAME = "Calypso/Bidir/State/FirstOff/B"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Range
    {
        min : 1f32, max : 100f32, inc_min : 1f32, inc_max : 2f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : Some(true), endianness : None,
        format : None, default : Some(35.4f32), ieee754_f32 : None, value :
        __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x450".to_string(), points : vec_points, points_intopic : None,
        topic : "Calypso/Bidir/State/FirstOff/B".to_string(), unit :
        "G".to_string(), name : "Calypso/Bidir/State/FirstOff/B".to_string(),
        last_update : Instant :: now(), desc : "Example Enc Msg".to_string(),
        key : Some("FirstOff".to_string()), is_ext : None, sim_freq : 750f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "Calypso/Bidir/State/FirstOff/B"; let
    _____START_DEBUG_FIELD_NAME = "Calypso/Bidir/State/FirstOff/C"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Range
    {
        min : 1f32, max : 100f32, inc_min : 1f32, inc_max : 2f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : None, endianness : None, format
        : None, default : Some(19f32), ieee754_f32 : None, value :
        __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x450".to_string(), points : vec_points, points_intopic : None,
        topic : "Calypso/Bidir/State/FirstOff/C".to_string(), unit :
        "G".to_string(), name : "Calypso/Bidir/State/FirstOff/C".to_string(),
        last_update : Instant :: now(), desc : "Example Enc Msg".to_string(),
        key : Some("FirstOff".to_string()), is_ext : None, sim_freq : 750f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "Calypso/Bidir/State/FirstOff/C"; let
    _____START_DEBUG_FIELD_NAME = "Calypso/Bidir/State/FirstOff/D"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Range
    {
        min : 1f32, max : 100f32, inc_min : 1f32, inc_max : 2f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : Some(true), endianness :
        Some("little".to_string()), format : Some("divide100".to_string()),
        default : Some(-21.8f32), ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x450".to_string(), points : vec_points, points_intopic : None,
        topic : "Calypso/Bidir/State/FirstOff/D".to_string(), unit :
        "G".to_string(), name : "Calypso/Bidir/State/FirstOff/D".to_string(),
        last_update : Instant :: now(), desc : "Example Enc Msg".to_string(),
        key : Some("FirstOff".to_string()), is_ext : None, sim_freq : 750f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "Calypso/Bidir/State/FirstOff/D"; let
    _____START_DEBUG_FIELD_NAME = "Calypso/Bidir/State/SecondOff/A"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Range
    {
        min : 1f32, max : 100f32, inc_min : 1f32, inc_max : 2f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 32usize, parse : None, signed : None, endianness :
        Some("little".to_string()), format : None, default : None, ieee754_f32
        : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0xFFFFF".to_string(), points : vec_points, points_intopic :
        None, topic : "Calypso/Bidir/State/SecondOff/A".to_string(), unit :
        "GG".to_string(), name :
        "Calypso/Bidir/State/SecondOff/A".to_string(), last_update : Instant
        :: now(), desc : "Example Enc Msg Ext".to_string(), key :
        Some("SecondOff".to_string()), is_ext : Some(true), sim_freq : 750f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "Calypso/Bidir/State/SecondOff/A"; let
    _____START_DEBUG_FIELD_NAME = "Calypso/Bidir/Unknown"; let mut vec_points
    : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 1f32, max : 100f32, inc_min : 1f32, inc_max : 2f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x7FF".to_string(), points : vec_points, points_intopic : None,
        topic : "Calypso/Bidir/Unknown".to_string(), unit : "pts".to_string(),
        name : "Calypso/Bidir/Unknown".to_string(), last_update : Instant ::
        now(), desc : "Calypso bidir unknown key".to_string(), key : None,
        is_ext : None, sim_freq : 750f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "Calypso/Bidir/Unknown"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Pack/Voltage"; let mut vec_points : Vec
    < SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 400f32, max : 505f32, inc_min : 0.01f32, inc_max : 0.3f32, round
        : false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : None, endianness : None, format
        : Some("divide10".to_string()), default : None, ieee754_f32 : None,
        value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x80".to_string(), points : vec_points, points_intopic : None,
        topic : "BMS/Pack/Voltage".to_string(), unit : "V".to_string(), name :
        "BMS/Pack/Voltage".to_string(), last_update : Instant :: now(), desc :
        "accumulator status".to_string(), key : None, is_ext : None, sim_freq
        : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Pack/Voltage"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Pack/Current"; let mut vec_points : Vec
    < SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 0f32, max : 50f32, inc_min : 0f32, inc_max : 3f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : Some(false), signed : None, endianness : None,
        format : None, default : None, ieee754_f32 : None, value :
        __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x80".to_string(), points : vec_points, points_intopic : None,
        topic : "BMS/Pack/Current".to_string(), unit : "A".to_string(), name :
        "BMS/Pack/Current".to_string(), last_update : Instant :: now(), desc :
        "accumulator status".to_string(), key : None, is_ext : None, sim_freq
        : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Pack/Current"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Pack/Amp-hours"; let mut vec_points :
    Vec < SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 0f32, max : 100f32, inc_min : 0f32, inc_max : 0.1f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x80".to_string(), points : vec_points, points_intopic : None,
        topic : "BMS/Pack/Amp-hours".to_string(), unit : "Ah".to_string(),
        name : "BMS/Pack/Amp-hours".to_string(), last_update : Instant ::
        now(), desc : "accumulator status".to_string(), key : None, is_ext :
        None, sim_freq : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Pack/Amp-hours"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Pack/SOC"; let mut vec_points : Vec <
    SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 0f32, max : 100f32, inc_min : 0.01f32, inc_max : 0.1f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x80".to_string(), points : vec_points, points_intopic : None,
        topic : "BMS/Pack/SOC".to_string(), unit : "%".to_string(), name :
        "BMS/Pack/SOC".to_string(), last_update : Instant :: now(), desc :
        "accumulator status".to_string(), key : None, is_ext : None, sim_freq
        : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Pack/SOC"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Pack/Health"; let mut vec_points : Vec
    < SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 0f32, max : 100f32, inc_min : 0f32, inc_max : 0.1f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x80".to_string(), points : vec_points, points_intopic : None,
        topic : "BMS/Pack/Health".to_string(), unit : "%".to_string(), name :
        "BMS/Pack/Health".to_string(), last_update : Instant :: now(), desc :
        "accumulator status".to_string(), key : None, is_ext : None, sim_freq
        : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Pack/Health"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Cells/Volts_High_Value"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Range
    {
        min : 2.5f32, max : 4.25f32, inc_min : 0.001f32, inc_max : 0.1f32,
        round : false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : None, endianness : None, format
        : Some("divide10000".to_string()), default : None, ieee754_f32 : None,
        value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x83".to_string(), points : vec_points, points_intopic : None,
        topic : "BMS/Cells/Volts_High_Value".to_string(), unit :
        "V".to_string(), name : "BMS/Cells/Volts_High_Value".to_string(),
        last_update : Instant :: now(), desc : "Cell Data".to_string(), key :
        None, is_ext : None, sim_freq : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Cells/Volts_High_Value"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Cells/Volts_High_Chip"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Range
    {
        min : 1f32, max : 12f32, inc_min : 1f32, inc_max : 1f32, round : true,
        current : 0.0
    }; let __new_point = SimPoint
    {
        size : 4usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x83".to_string(), points : vec_points, points_intopic : None,
        topic : "BMS/Cells/Volts_High_Chip".to_string(), unit :
        "".to_string(), name : "BMS/Cells/Volts_High_Chip".to_string(),
        last_update : Instant :: now(), desc : "Cell Data".to_string(), key :
        None, is_ext : None, sim_freq : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Cells/Volts_High_Chip"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Cells/Volts_High_Cell"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Range
    {
        min : 1f32, max : 120f32, inc_min : 1f32, inc_max : 1f32, round :
        true, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 4usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x83".to_string(), points : vec_points, points_intopic : None,
        topic : "BMS/Cells/Volts_High_Cell".to_string(), unit :
        "".to_string(), name : "BMS/Cells/Volts_High_Cell".to_string(),
        last_update : Instant :: now(), desc : "Cell Data".to_string(), key :
        None, is_ext : None, sim_freq : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Cells/Volts_High_Cell"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Cells/Volts_Low_Value"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Range
    {
        min : 2.5f32, max : 4.25f32, inc_min : 0.001f32, inc_max : 0.1f32,
        round : false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : None, endianness : None, format
        : Some("divide10000".to_string()), default : None, ieee754_f32 : None,
        value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x83".to_string(), points : vec_points, points_intopic : None,
        topic : "BMS/Cells/Volts_Low_Value".to_string(), unit :
        "V".to_string(), name : "BMS/Cells/Volts_Low_Value".to_string(),
        last_update : Instant :: now(), desc : "Cell Data".to_string(), key :
        None, is_ext : None, sim_freq : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Cells/Volts_Low_Value"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Cells/Volts_Low_Chip"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Range
    {
        min : 1f32, max : 12f32, inc_min : 1f32, inc_max : 1f32, round : true,
        current : 0.0
    }; let __new_point = SimPoint
    {
        size : 4usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x83".to_string(), points : vec_points, points_intopic : None,
        topic : "BMS/Cells/Volts_Low_Chip".to_string(), unit : "".to_string(),
        name : "BMS/Cells/Volts_Low_Chip".to_string(), last_update : Instant
        :: now(), desc : "Cell Data".to_string(), key : None, is_ext : None,
        sim_freq : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Cells/Volts_Low_Chip"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Cells/Volts_Low_Cell"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Range
    {
        min : 1f32, max : 120f32, inc_min : 1f32, inc_max : 1f32, round :
        true, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 4usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x83".to_string(), points : vec_points, points_intopic : None,
        topic : "BMS/Cells/Volts_Low_Cell".to_string(), unit : "".to_string(),
        name : "BMS/Cells/Volts_Low_Cell".to_string(), last_update : Instant
        :: now(), desc : "Cell Data".to_string(), key : None, is_ext : None,
        sim_freq : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Cells/Volts_Low_Cell"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Cells/Volts_Avg_Value"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Range
    {
        min : 2.5f32, max : 4.25f32, inc_min : 0.001f32, inc_max : 0.1f32,
        round : false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : None, endianness : None, format
        : Some("divide10000".to_string()), default : None, ieee754_f32 : None,
        value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x83".to_string(), points : vec_points, points_intopic : None,
        topic : "BMS/Cells/Volts_Avg_Value".to_string(), unit :
        "V".to_string(), name : "BMS/Cells/Volts_Avg_Value".to_string(),
        last_update : Instant :: now(), desc : "Cell Data".to_string(), key :
        None, is_ext : None, sim_freq : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Cells/Volts_Avg_Value"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Cells/Temp_High_Value"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Range
    {
        min : 10f32, max : 50f32, inc_min : 0.01f32, inc_max : 0.3f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x84".to_string(), points : vec_points, points_intopic : None,
        topic : "BMS/Cells/Temp_High_Value".to_string(), unit :
        "C".to_string(), name : "BMS/Cells/Temp_High_Value".to_string(),
        last_update : Instant :: now(), desc :
        "Cell Temperatures".to_string(), key : None, is_ext : None, sim_freq :
        700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Cells/Temp_High_Value"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Cells/Temp_High_Cell"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Range
    {
        min : 1f32, max : 120f32, inc_min : 1f32, inc_max : 1f32, round :
        true, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 4usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x84".to_string(), points : vec_points, points_intopic : None,
        topic : "BMS/Cells/Temp_High_Cell".to_string(), unit : "".to_string(),
        name : "BMS/Cells/Temp_High_Cell".to_string(), last_update : Instant
        :: now(), desc : "Cell Temperatures".to_string(), key : None, is_ext :
        None, sim_freq : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Cells/Temp_High_Cell"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Cells/Temp_High_Chip"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Range
    {
        min : 1f32, max : 12f32, inc_min : 1f32, inc_max : 1f32, round : true,
        current : 0.0
    }; let __new_point = SimPoint
    {
        size : 4usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x84".to_string(), points : vec_points, points_intopic : None,
        topic : "BMS/Cells/Temp_High_Chip".to_string(), unit : "".to_string(),
        name : "BMS/Cells/Temp_High_Chip".to_string(), last_update : Instant
        :: now(), desc : "Cell Temperatures".to_string(), key : None, is_ext :
        None, sim_freq : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Cells/Temp_High_Chip"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Cells/Temp_Low_Value"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Range
    {
        min : 10f32, max : 50f32, inc_min : 0.01f32, inc_max : 0.3f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x84".to_string(), points : vec_points, points_intopic : None,
        topic : "BMS/Cells/Temp_Low_Value".to_string(), unit :
        "C".to_string(), name : "BMS/Cells/Temp_Low_Value".to_string(),
        last_update : Instant :: now(), desc :
        "Cell Temperatures".to_string(), key : None, is_ext : None, sim_freq :
        700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Cells/Temp_Low_Value"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Cells/Temp_Low_Cell"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Range
    {
        min : 1f32, max : 120f32, inc_min : 1f32, inc_max : 1f32, round :
        true, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 4usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x84".to_string(), points : vec_points, points_intopic : None,
        topic : "BMS/Cells/Temp_Low_Cell".to_string(), unit : "".to_string(),
        name : "BMS/Cells/Temp_Low_Cell".to_string(), last_update : Instant ::
        now(), desc : "Cell Temperatures".to_string(), key : None, is_ext :
        None, sim_freq : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Cells/Temp_Low_Cell"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Cells/Temp_Low_Chip"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Range
    {
        min : 1f32, max : 12f32, inc_min : 1f32, inc_max : 1f32, round : true,
        current : 0.0
    }; let __new_point = SimPoint
    {
        size : 4usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x84".to_string(), points : vec_points, points_intopic : None,
        topic : "BMS/Cells/Temp_Low_Chip".to_string(), unit : "".to_string(),
        name : "BMS/Cells/Temp_Low_Chip".to_string(), last_update : Instant ::
        now(), desc : "Cell Temperatures".to_string(), key : None, is_ext :
        None, sim_freq : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Cells/Temp_Low_Chip"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Cells/Temp_Avg_Value"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Range
    {
        min : 10f32, max : 50f32, inc_min : 0.01f32, inc_max : 0.3f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x84".to_string(), points : vec_points, points_intopic : None,
        topic : "BMS/Cells/Temp_Avg_Value".to_string(), unit :
        "C".to_string(), name : "BMS/Cells/Temp_Avg_Value".to_string(),
        last_update : Instant :: now(), desc :
        "Cell Temperatures".to_string(), key : None, is_ext : None, sim_freq :
        700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Cells/Temp_Avg_Value"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Segment_Temp/1"; let mut vec_points :
    Vec < SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 10f32, max : 50f32, inc_min : 0.01f32, inc_max : 0.3f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : Some(true), endianness : None,
        format : None, default : None, ieee754_f32 : None, value :
        __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x85".to_string(), points : vec_points, points_intopic : None,
        topic : "BMS/Segment_Temp/1".to_string(), unit : "C".to_string(), name
        : "BMS/Segment_Temp/1".to_string(), last_update : Instant :: now(),
        desc : "Segment Temperatures".to_string(), key : None, is_ext : None,
        sim_freq : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Segment_Temp/1"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Segment_Temp/2"; let mut vec_points :
    Vec < SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 10f32, max : 50f32, inc_min : 0.01f32, inc_max : 0.3f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : Some(true), endianness : None,
        format : None, default : None, ieee754_f32 : None, value :
        __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x85".to_string(), points : vec_points, points_intopic : None,
        topic : "BMS/Segment_Temp/2".to_string(), unit : "C".to_string(), name
        : "BMS/Segment_Temp/2".to_string(), last_update : Instant :: now(),
        desc : "Segment Temperatures".to_string(), key : None, is_ext : None,
        sim_freq : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Segment_Temp/2"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Segment_Temp/3"; let mut vec_points :
    Vec < SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 10f32, max : 50f32, inc_min : 0.01f32, inc_max : 0.3f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : Some(true), endianness : None,
        format : None, default : None, ieee754_f32 : None, value :
        __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x85".to_string(), points : vec_points, points_intopic : None,
        topic : "BMS/Segment_Temp/3".to_string(), unit : "C".to_string(), name
        : "BMS/Segment_Temp/3".to_string(), last_update : Instant :: now(),
        desc : "Segment Temperatures".to_string(), key : None, is_ext : None,
        sim_freq : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Segment_Temp/3"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Segment_Temp/4"; let mut vec_points :
    Vec < SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 10f32, max : 50f32, inc_min : 0.01f32, inc_max : 0.3f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : Some(true), endianness : None,
        format : None, default : None, ieee754_f32 : None, value :
        __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x85".to_string(), points : vec_points, points_intopic : None,
        topic : "BMS/Segment_Temp/4".to_string(), unit : "C".to_string(), name
        : "BMS/Segment_Temp/4".to_string(), last_update : Instant :: now(),
        desc : "Segment Temperatures".to_string(), key : None, is_ext : None,
        sim_freq : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Segment_Temp/4"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Segment_Temp/5"; let mut vec_points :
    Vec < SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 10f32, max : 50f32, inc_min : 0.01f32, inc_max : 0.3f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : Some(true), endianness : None,
        format : None, default : None, ieee754_f32 : None, value :
        __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x85".to_string(), points : vec_points, points_intopic : None,
        topic : "BMS/Segment_Temp/5".to_string(), unit : "C".to_string(), name
        : "BMS/Segment_Temp/5".to_string(), last_update : Instant :: now(),
        desc : "Segment Temperatures".to_string(), key : None, is_ext : None,
        sim_freq : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Segment_Temp/5"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Segment_Temp/6"; let mut vec_points :
    Vec < SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 10f32, max : 50f32, inc_min : 0.01f32, inc_max : 0.3f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : Some(true), endianness : None,
        format : None, default : None, ieee754_f32 : None, value :
        __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x85".to_string(), points : vec_points, points_intopic : None,
        topic : "BMS/Segment_Temp/6".to_string(), unit : "C".to_string(), name
        : "BMS/Segment_Temp/6".to_string(), last_update : Instant :: now(),
        desc : "Segment Temperatures".to_string(), key : None, is_ext : None,
        sim_freq : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Segment_Temp/6"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Pack/DCL"; let mut vec_points : Vec <
    SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 20f32, max : 520f32, inc_min : 0.1f32, inc_max : 4f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x86".to_string(), points : vec_points, points_intopic : None,
        topic : "BMS/Pack/DCL".to_string(), unit : "".to_string(), name :
        "BMS/Pack/DCL".to_string(), last_update : Instant :: now(), desc :
        "Current Limits".to_string(), key : None, is_ext : None, sim_freq :
        700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Pack/DCL"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Pack/CCL"; let mut vec_points : Vec <
    SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 0f32, max : 25f32, inc_min : 0f32, inc_max : 2f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : Some(true), endianness : None,
        format : None, default : None, ieee754_f32 : None, value :
        __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x86".to_string(), points : vec_points, points_intopic : None,
        topic : "BMS/Pack/CCL".to_string(), unit : "".to_string(), name :
        "BMS/Pack/CCL".to_string(), last_update : Instant :: now(), desc :
        "Current Limits".to_string(), key : None, is_ext : None, sim_freq :
        700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Pack/CCL"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Pack/Current"; let mut vec_points : Vec
    < SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 0f32, max : 100f32, inc_min : 0f32, inc_max : 5f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : Some(true), endianness : None,
        format : None, default : None, ieee754_f32 : None, value :
        __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x86".to_string(), points : vec_points, points_intopic : None,
        topic : "BMS/Pack/Current".to_string(), unit : "".to_string(), name :
        "BMS/Pack/Current".to_string(), last_update : Instant :: now(), desc :
        "Current Limits".to_string(), key : None, is_ext : None, sim_freq :
        700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Pack/Current"; __all_sim_components
}