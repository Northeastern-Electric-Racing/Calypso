use std :: time :: Instant; use crate :: simulatable_message ::
{ SimComponent, SimValue, SimPoint }; pub fn create_simulated_components() ->
Vec < SimComponent >
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
        unit : "ERPM".to_string(), name : "DTI/RPM/ERPM".to_string(),
        last_update : Instant :: now(), desc :
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
        unit : "%".to_string(), name : "DTI/Power/Duty_Cycle".to_string(),
        last_update : Instant :: now(), desc :
        "ERPM_Duty_Input_Voltage_Status".to_string(), key : None, is_ext :
        None, sim_freq : 25f32,
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
        unit : "V".to_string(), name : "DTI/Power/Input_Voltage".to_string(),
        last_update : Instant :: now(), desc :
        "ERPM_Duty_Input_Voltage_Status".to_string(), key : None, is_ext :
        None, sim_freq : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "DTI/Power/Input_Voltage"; let
    _____START_DEBUG_FIELD_NAME = "DTI/Power/AC_Current"; let mut vec_points :
    Vec < SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : -125f32, max : 250f32, inc_min : 0.01f32, inc_max : 20f32, round
        : false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : Some(true), endianness : None,
        format : Some("divide10".to_string()), default : None, ieee754_f32 :
        None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x436".to_string(), points : vec_points, points_intopic : None,
        unit : "A".to_string(), name : "DTI/Power/AC_Current".to_string(),
        last_update : Instant :: now(), desc : "Currents_Status".to_string(),
        key : None, is_ext : None, sim_freq : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "DTI/Power/AC_Current"; let
    _____START_DEBUG_FIELD_NAME = "DTI/Power/DC_Current"; let mut vec_points :
    Vec < SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : -300f32, max : 300f32, inc_min : 2f32, inc_max : 5f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : Some(true), endianness : None,
        format : Some("divide10".to_string()), default : None, ieee754_f32 :
        None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x436".to_string(), points : vec_points, points_intopic : None,
        unit : "A".to_string(), name : "DTI/Power/DC_Current".to_string(),
        last_update : Instant :: now(), desc : "Currents_Status".to_string(),
        key : None, is_ext : None, sim_freq : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "DTI/Power/DC_Current"; let
    _____START_DEBUG_FIELD_NAME = "DTI/Temps/Controller_Temperature"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Range
    {
        min : 5f32, max : 80f32, inc_min : 0.05f32, inc_max : 0.5f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : Some(true), endianness : None,
        format : Some("divide10".to_string()), default : None, ieee754_f32 :
        None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x456".to_string(), points : vec_points, points_intopic : None,
        unit : "C".to_string(), name :
        "DTI/Temps/Controller_Temperature".to_string(), last_update : Instant
        :: now(), desc : "Temps_Fault_Code_Status".to_string(), key : None,
        is_ext : None, sim_freq : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "DTI/Temps/Controller_Temperature"; let
    _____START_DEBUG_FIELD_NAME = "DTI/Temps/Motor_Temperature"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Range
    {
        min : 5f32, max : 80f32, inc_min : 0.05f32, inc_max : 0.5f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : Some(true), endianness : None,
        format : Some("divide10".to_string()), default : None, ieee754_f32 :
        None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x456".to_string(), points : vec_points, points_intopic : None,
        unit : "C".to_string(), name :
        "DTI/Temps/Motor_Temperature".to_string(), last_update : Instant ::
        now(), desc : "Temps_Fault_Code_Status".to_string(), key : None,
        is_ext : None, sim_freq : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "DTI/Temps/Motor_Temperature"; let
    _____START_DEBUG_FIELD_NAME = "DTI/Fault/Fault_Code"; let mut vec_points :
    Vec < SimPoint > = Vec :: new(); let __new_value = SimValue :: Discrete
    {
        options : vec!
        [(0f32, 0.9f32), (1f32, 0.90999997f32), (2f32, 0.91999996f32),
        (3f32, 0.92999995f32), (4f32, 0.93999994f32), (5f32, 0.9499999f32),
        (6f32, 0.9599999f32), (7f32, 0.9699999f32), (8f32, 0.9799999f32),
        (9f32, 0.9899999f32), (10f32, 0.9999999f32)], current : 0.0
    }; let __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x456".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name : "DTI/Fault/Fault_Code".to_string(),
        last_update : Instant :: now(), desc :
        "Temps_Fault_Code_Status".to_string(), key : None, is_ext : None,
        sim_freq : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "DTI/Fault/Fault_Code"; let
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
        unit : "A".to_string(), name : "DTI/FOC/Component_Id".to_string(),
        last_update : Instant :: now(), desc : "MC_FOC_Algorithm".to_string(),
        key : None, is_ext : None, sim_freq : 25f32,
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
        unit : "A".to_string(), name : "DTI/FOC/Component_Iq".to_string(),
        last_update : Instant :: now(), desc : "MC_FOC_Algorithm".to_string(),
        key : None, is_ext : None, sim_freq : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "DTI/FOC/Component_Iq"; let
    _____START_DEBUG_FIELD_NAME = "DTI/General/Throttle_Signal"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Range
    {
        min : -100f32, max : 100f32, inc_min : 1f32, inc_max : 1f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : Some(true), endianness : None,
        format : None, default : None, ieee754_f32 : None, value :
        __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x496".to_string(), points : vec_points, points_intopic : None,
        unit : "%".to_string(), name :
        "DTI/General/Throttle_Signal".to_string(), last_update : Instant ::
        now(), desc : "MC_General_Data".to_string(), key : None, is_ext :
        None, sim_freq : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "DTI/General/Throttle_Signal"; let
    _____START_DEBUG_FIELD_NAME = "DTI/General/Brake_Signal"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Range
    {
        min : -100f32, max : 100f32, inc_min : 1f32, inc_max : 1f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : Some(true), endianness : None,
        format : None, default : None, ieee754_f32 : None, value :
        __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x496".to_string(), points : vec_points, points_intopic : None,
        unit : "%".to_string(), name : "DTI/General/Brake_Signal".to_string(),
        last_update : Instant :: now(), desc : "MC_General_Data".to_string(),
        key : None, is_ext : None, sim_freq : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "DTI/General/Brake_Signal"; let
    _____START_DEBUG_FIELD_NAME = "DTI/General/Digital_Out_4"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Discrete
    { options : vec! [(0f32, 0.95f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x496".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name : "DTI/General/Digital_Out_4".to_string(),
        last_update : Instant :: now(), desc : "MC_General_Data".to_string(),
        key : None, is_ext : None, sim_freq : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "DTI/General/Digital_Out_4"; let
    _____START_DEBUG_FIELD_NAME = "DTI/General/Digital_Out_3"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Discrete
    { options : vec! [(0f32, 0.95f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x496".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name : "DTI/General/Digital_Out_3".to_string(),
        last_update : Instant :: now(), desc : "MC_General_Data".to_string(),
        key : None, is_ext : None, sim_freq : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "DTI/General/Digital_Out_3"; let
    _____START_DEBUG_FIELD_NAME = "DTI/General/Digital_Out_2"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Discrete
    { options : vec! [(0f32, 0.95f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x496".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name : "DTI/General/Digital_Out_2".to_string(),
        last_update : Instant :: now(), desc : "MC_General_Data".to_string(),
        key : None, is_ext : None, sim_freq : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "DTI/General/Digital_Out_2"; let
    _____START_DEBUG_FIELD_NAME = "DTI/General/Digital_Out_1"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Discrete
    { options : vec! [(0f32, 0.95f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x496".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name : "DTI/General/Digital_Out_1".to_string(),
        last_update : Instant :: now(), desc : "MC_General_Data".to_string(),
        key : None, is_ext : None, sim_freq : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "DTI/General/Digital_Out_1"; let
    _____START_DEBUG_FIELD_NAME = "DTI/General/Digital_In_4"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Discrete
    { options : vec! [(0f32, 0.95f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x496".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name : "DTI/General/Digital_In_4".to_string(),
        last_update : Instant :: now(), desc : "MC_General_Data".to_string(),
        key : None, is_ext : None, sim_freq : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "DTI/General/Digital_In_4"; let
    _____START_DEBUG_FIELD_NAME = "DTI/General/Digital_In_3"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Discrete
    { options : vec! [(0f32, 0.95f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x496".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name : "DTI/General/Digital_In_3".to_string(),
        last_update : Instant :: now(), desc : "MC_General_Data".to_string(),
        key : None, is_ext : None, sim_freq : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "DTI/General/Digital_In_3"; let
    _____START_DEBUG_FIELD_NAME = "DTI/General/Digital_In_2"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Discrete
    { options : vec! [(0f32, 0.95f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x496".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name : "DTI/General/Digital_In_2".to_string(),
        last_update : Instant :: now(), desc : "MC_General_Data".to_string(),
        key : None, is_ext : None, sim_freq : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "DTI/General/Digital_In_2"; let
    _____START_DEBUG_FIELD_NAME = "DTI/General/Digital_In_1"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Discrete
    { options : vec! [(0f32, 0.95f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x496".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name : "DTI/General/Digital_In_1".to_string(),
        last_update : Instant :: now(), desc : "MC_General_Data".to_string(),
        key : None, is_ext : None, sim_freq : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "DTI/General/Digital_In_1"; let
    _____START_DEBUG_FIELD_NAME = "DTI/General/Drive_Enable"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Discrete
    { options : vec! [(0f32, 0.8f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x496".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name : "DTI/General/Drive_Enable".to_string(),
        last_update : Instant :: now(), desc : "MC_General_Data".to_string(),
        key : None, is_ext : None, sim_freq : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "DTI/General/Drive_Enable"; let
    _____START_DEBUG_FIELD_NAME = "DTI/Limit/Motor_Temp_Limit"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Discrete
    { options : vec! [(0f32, 0.95f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x496".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name :
        "DTI/Limit/Motor_Temp_Limit".to_string(), last_update : Instant ::
        now(), desc : "MC_General_Data".to_string(), key : None, is_ext :
        None, sim_freq : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "DTI/Limit/Motor_Temp_Limit"; let
    _____START_DEBUG_FIELD_NAME = "DTI/Limit/Motor_Acc_Temp_Limit"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Discrete
    { options : vec! [(0f32, 0.95f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x496".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name :
        "DTI/Limit/Motor_Acc_Temp_Limit".to_string(), last_update : Instant ::
        now(), desc : "MC_General_Data".to_string(), key : None, is_ext :
        None, sim_freq : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "DTI/Limit/Motor_Acc_Temp_Limit"; let
    _____START_DEBUG_FIELD_NAME = "DTI/Limit/Input_Voltage_Limit"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Discrete
    { options : vec! [(0f32, 0.95f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x496".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name :
        "DTI/Limit/Input_Voltage_Limit".to_string(), last_update : Instant ::
        now(), desc : "MC_General_Data".to_string(), key : None, is_ext :
        None, sim_freq : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "DTI/Limit/Input_Voltage_Limit"; let
    _____START_DEBUG_FIELD_NAME = "DTI/Limit/IGBT_Temp_Limit"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Discrete
    { options : vec! [(0f32, 0.95f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x496".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name : "DTI/Limit/IGBT_Temp_Limit".to_string(),
        last_update : Instant :: now(), desc : "MC_General_Data".to_string(),
        key : None, is_ext : None, sim_freq : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "DTI/Limit/IGBT_Temp_Limit"; let
    _____START_DEBUG_FIELD_NAME = "DTI/Limit/IGBT_Acc_Temp_Limit"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Discrete
    { options : vec! [(0f32, 0.95f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x496".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name :
        "DTI/Limit/IGBT_Acc_Temp_Limit".to_string(), last_update : Instant ::
        now(), desc : "MC_General_Data".to_string(), key : None, is_ext :
        None, sim_freq : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "DTI/Limit/IGBT_Acc_Temp_Limit"; let
    _____START_DEBUG_FIELD_NAME = "DTI/Limit/Drive_Enable_Limit"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Discrete
    { options : vec! [(0f32, 0.8f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x496".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name :
        "DTI/Limit/Drive_Enable_Limit".to_string(), last_update : Instant ::
        now(), desc : "MC_General_Data".to_string(), key : None, is_ext :
        None, sim_freq : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "DTI/Limit/Drive_Enable_Limit"; let
    _____START_DEBUG_FIELD_NAME = "DTI/Limit/DC_Current_Limit"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Discrete
    { options : vec! [(0f32, 0.95f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x496".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name :
        "DTI/Limit/DC_Current_Limit".to_string(), last_update : Instant ::
        now(), desc : "MC_General_Data".to_string(), key : None, is_ext :
        None, sim_freq : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "DTI/Limit/DC_Current_Limit"; let
    _____START_DEBUG_FIELD_NAME = "DTI/Limit/Cap_Temp_Limit"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Discrete
    { options : vec! [(0f32, 0.95f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x496".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name : "DTI/Limit/Cap_Temp_Limit".to_string(),
        last_update : Instant :: now(), desc : "MC_General_Data".to_string(),
        key : None, is_ext : None, sim_freq : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "DTI/Limit/Cap_Temp_Limit"; let
    _____START_DEBUG_FIELD_NAME = "DTI/Limit/Power_Limit"; let mut vec_points
    : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue :: Discrete
    { options : vec! [(0f32, 0.95f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x496".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name : "DTI/Limit/Power_Limit".to_string(),
        last_update : Instant :: now(), desc : "MC_General_Data".to_string(),
        key : None, is_ext : None, sim_freq : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "DTI/Limit/Power_Limit"; let
    _____START_DEBUG_FIELD_NAME = "DTI/Limit/RPM_Max_Limit"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Discrete
    { options : vec! [(0f32, 0.95f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x496".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name : "DTI/Limit/RPM_Max_Limit".to_string(),
        last_update : Instant :: now(), desc : "MC_General_Data".to_string(),
        key : None, is_ext : None, sim_freq : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "DTI/Limit/RPM_Max_Limit"; let
    _____START_DEBUG_FIELD_NAME = "DTI/Limit/RPM_Min_Limit"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Discrete
    { options : vec! [(0f32, 0.95f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x496".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name : "DTI/Limit/RPM_Min_Limit".to_string(),
        last_update : Instant :: now(), desc : "MC_General_Data".to_string(),
        key : None, is_ext : None, sim_freq : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "DTI/Limit/RPM_Min_Limit"; let
    _____START_DEBUG_FIELD_NAME = "DTI/General/CAN_Map_Version"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Discrete
    {
        options : vec!
        [(20f32, 0.01f32), (21f32, 0.02f32), (22f32, 0.03f32),
        (23f32, 0.04f32), (24f32, 1f32)], current : 0.0
    }; let __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x496".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name :
        "DTI/General/CAN_Map_Version".to_string(), last_update : Instant ::
        now(), desc : "MC_General_Data".to_string(), key : None, is_ext :
        None, sim_freq : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "DTI/General/CAN_Map_Version"; let
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
        unit : "A".to_string(), name :
        "MPU/Commands/AC_Current_Target".to_string(), last_update : Instant ::
        now(), desc : "AC_Current_Command".to_string(), key : None, is_ext :
        None, sim_freq : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MPU/Commands/AC_Current_Target"; let
    _____START_DEBUG_FIELD_NAME = "MPU/Commands/Brake_Current_Target"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Range
    {
        min : 0f32, max : 150f32, inc_min : 0.01f32, inc_max : 10f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : Some(true), endianness : None,
        format : Some("divide10".to_string()), default : None, ieee754_f32 :
        None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x056".to_string(), points : vec_points, points_intopic : None,
        unit : "A".to_string(), name :
        "MPU/Commands/Brake_Current_Target".to_string(), last_update : Instant
        :: now(), desc : "Brake_Current_Command".to_string(), key : None,
        is_ext : None, sim_freq : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MPU/Commands/Brake_Current_Target"; let
    _____START_DEBUG_FIELD_NAME = "DTI/Commands/ERPM_Target"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Range
    {
        min : -100000f32, max : 100000f32, inc_min : 100f32, inc_max : 400f32,
        round : false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 32usize, parse : None, signed : Some(true), endianness : None,
        format : None, default : None, ieee754_f32 : None, value :
        __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x076".to_string(), points : vec_points, points_intopic : None,
        unit : "A".to_string(), name : "DTI/Commands/ERPM_Target".to_string(),
        last_update : Instant :: now(), desc : "ERPM_Command".to_string(), key
        : None, is_ext : None, sim_freq : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "DTI/Commands/ERPM_Target"; let
    _____START_DEBUG_FIELD_NAME = "DTI/Commands/Position_Target"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Range
    {
        min : 0f32, max : 360f32, inc_min : 0.05f32, inc_max : 5f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : Some(true), endianness : None,
        format : Some("divide10".to_string()), default : None, ieee754_f32 :
        None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x096".to_string(), points : vec_points, points_intopic : None,
        unit : "degree".to_string(), name :
        "DTI/Commands/Position_Target".to_string(), last_update : Instant ::
        now(), desc : "Position_Command".to_string(), key : None, is_ext :
        None, sim_freq : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "DTI/Commands/Position_Target"; let
    _____START_DEBUG_FIELD_NAME = "DTI/Commands/Relative_Current_Target"; let
    mut vec_points : Vec < SimPoint > = Vec :: new(); let __new_value =
    SimValue :: Range
    {
        min : -100f32, max : 100f32, inc_min : 0.01f32, inc_max : 5f32, round
        : false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : Some(true), endianness : None,
        format : Some("divide10".to_string()), default : None, ieee754_f32 :
        None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x0B6".to_string(), points : vec_points, points_intopic : None,
        unit : "A".to_string(), name :
        "DTI/Commands/Relative_Current_Target".to_string(), last_update :
        Instant :: now(), desc : "Relative_Current_Command".to_string(), key :
        None, is_ext : None, sim_freq : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "DTI/Commands/Relative_Current_Target"; let
    _____START_DEBUG_FIELD_NAME =
    "DTI/Commands/Relative_Brake_Current_Target"; let mut vec_points : Vec <
    SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : -100f32, max : 100f32, inc_min : 0.01f32, inc_max : 5f32, round
        : false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : Some(true), endianness : None,
        format : Some("divide10".to_string()), default : None, ieee754_f32 :
        None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x0D6".to_string(), points : vec_points, points_intopic : None,
        unit : "A".to_string(), name :
        "DTI/Commands/Relative_Brake_Current_Target".to_string(), last_update
        : Instant :: now(), desc :
        "Relative_Brake_Current_Command".to_string(), key : None, is_ext :
        None, sim_freq : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME =
    "DTI/Commands/Relative_Brake_Current_Target"; let
    _____START_DEBUG_FIELD_NAME = "DTI/Commands/Digital_Output_1_Target"; let
    mut vec_points : Vec < SimPoint > = Vec :: new(); let __new_value =
    SimValue :: Discrete
    { options : vec! [(0f32, 0.95f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x0F6".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name :
        "DTI/Commands/Digital_Output_1_Target".to_string(), last_update :
        Instant :: now(), desc : "Digital_Output_Command".to_string(), key :
        None, is_ext : None, sim_freq : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "DTI/Commands/Digital_Output_1_Target"; let
    _____START_DEBUG_FIELD_NAME = "DTI/Commands/Digital_Output_2_Target"; let
    mut vec_points : Vec < SimPoint > = Vec :: new(); let __new_value =
    SimValue :: Discrete
    { options : vec! [(0f32, 0.95f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x0F6".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name :
        "DTI/Commands/Digital_Output_2_Target".to_string(), last_update :
        Instant :: now(), desc : "Digital_Output_Command".to_string(), key :
        None, is_ext : None, sim_freq : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "DTI/Commands/Digital_Output_2_Target"; let
    _____START_DEBUG_FIELD_NAME = "DTI/Commands/Digital_Output_3_Target"; let
    mut vec_points : Vec < SimPoint > = Vec :: new(); let __new_value =
    SimValue :: Discrete
    { options : vec! [(0f32, 0.95f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x0F6".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name :
        "DTI/Commands/Digital_Output_3_Target".to_string(), last_update :
        Instant :: now(), desc : "Digital_Output_Command".to_string(), key :
        None, is_ext : None, sim_freq : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "DTI/Commands/Digital_Output_3_Target"; let
    _____START_DEBUG_FIELD_NAME = "DTI/Commands/Digital_Output_4_Target"; let
    mut vec_points : Vec < SimPoint > = Vec :: new(); let __new_value =
    SimValue :: Discrete
    { options : vec! [(0f32, 0.95f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x0F6".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name :
        "DTI/Commands/Digital_Output_4_Target".to_string(), last_update :
        Instant :: now(), desc : "Digital_Output_Command".to_string(), key :
        None, is_ext : None, sim_freq : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "DTI/Commands/Digital_Output_4_Target"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Commands/Max_AC_Current_Target"; let
    mut vec_points : Vec < SimPoint > = Vec :: new(); let __new_value =
    SimValue :: Range
    {
        min : 0f32, max : 250f32, inc_min : 0.01f32, inc_max : 25f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : Some(true), endianness : None,
        format : Some("divide10".to_string()), default : None, ieee754_f32 :
        None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x116".to_string(), points : vec_points, points_intopic : None,
        unit : "A".to_string(), name :
        "BMS/Commands/Max_AC_Current_Target".to_string(), last_update :
        Instant :: now(), desc : "Max_AC_Current_Command".to_string(), key :
        None, is_ext : None, sim_freq : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Commands/Max_AC_Current_Target"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Commands/Max_AC_Brake_Current_Target";
    let mut vec_points : Vec < SimPoint > = Vec :: new(); let __new_value =
    SimValue :: Range
    {
        min : -250f32, max : 0f32, inc_min : 0.01f32, inc_max : 25f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : Some(true), endianness : None,
        format : Some("divide10".to_string()), default : None, ieee754_f32 :
        None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x136".to_string(), points : vec_points, points_intopic : None,
        unit : "A".to_string(), name :
        "BMS/Commands/Max_AC_Brake_Current_Target".to_string(), last_update :
        Instant :: now(), desc : "Max_AC_Brake_Current_Command".to_string(),
        key : None, is_ext : None, sim_freq : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Commands/Max_AC_Brake_Current_Target";
    let _____START_DEBUG_FIELD_NAME = "BMS/Commands/Max_DC_Current_Target";
    let mut vec_points : Vec < SimPoint > = Vec :: new(); let __new_value =
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
        unit : "A".to_string(), name :
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
        unit : "A".to_string(), name :
        "BMS/Commands/Max_DC_Brake_Current_Target".to_string(), last_update :
        Instant :: now(), desc : "Max_DC_Brake_Current_Command".to_string(),
        key : None, is_ext : None, sim_freq : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Commands/Max_DC_Brake_Current_Target";
    let _____START_DEBUG_FIELD_NAME = "MPU/Commands/Drive_Enable_Target"; let
    mut vec_points : Vec < SimPoint > = Vec :: new(); let __new_value =
    SimValue :: Discrete
    { options : vec! [(0f32, 0.8f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x196".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name :
        "MPU/Commands/Drive_Enable_Target".to_string(), last_update : Instant
        :: now(), desc : "Drive_Enable_Command".to_string(), key : None,
        is_ext : None, sim_freq : 25f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MPU/Commands/Drive_Enable_Target"; let
    _____START_DEBUG_FIELD_NAME = "WHEEL/Buttons/1"; let mut vec_points : Vec
    < SimPoint > = Vec :: new(); let __new_value = SimValue :: Discrete
    {
        options : vec!
        [(0f32, 0.25f32), (1f32, 0.4f32), (2f32, 0.55f32),
        (3f32, 0.70000005f32), (4f32, 0.85f32), (5f32, 1f32)], current : 0.0
    }; let __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x680".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name : "WHEEL/Buttons/1".to_string(),
        last_update : Instant :: now(), desc : "Wheel State".to_string(), key
        : None, is_ext : None, sim_freq : 8f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "WHEEL/Buttons/1"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Charging/Voltage"; let mut vec_points :
    Vec < SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
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
        id : "0x1806E5F4".to_string(), points : vec_points, points_intopic :
        None, unit : "V".to_string(), name :
        "BMS/Charging/Voltage".to_string(), last_update : Instant :: now(),
        desc : "BMS Message Send".to_string(), key : None, is_ext : None,
        sim_freq : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Charging/Voltage"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Charging/Current"; let mut vec_points :
    Vec < SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 0f32, max : 12f32, inc_min : 1f32, inc_max : 1f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : None, endianness : None, format
        : Some("divide10".to_string()), default : None, ieee754_f32 : None,
        value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x1806E5F4".to_string(), points : vec_points, points_intopic :
        None, unit : "A".to_string(), name :
        "BMS/Charging/Current".to_string(), last_update : Instant :: now(),
        desc : "BMS Message Send".to_string(), key : None, is_ext : None,
        sim_freq : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Charging/Current"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Charging/Control"; let mut vec_points :
    Vec < SimPoint > = Vec :: new(); let __new_value = SimValue :: Discrete
    { options : vec! [(0f32, 0.9f32), (255f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x1806E5F4".to_string(), points : vec_points, points_intopic :
        None, unit : "".to_string(), name :
        "BMS/Charging/Control".to_string(), last_update : Instant :: now(),
        desc : "BMS Message Send".to_string(), key : None, is_ext : None,
        sim_freq : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Charging/Control"; let
    _____START_DEBUG_FIELD_NAME = "Charger/Box/Voltage"; let mut vec_points :
    Vec < SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
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
        id : "0x18FF50E5".to_string(), points : vec_points, points_intopic :
        None, unit : "V".to_string(), name :
        "Charger/Box/Voltage".to_string(), last_update : Instant :: now(),
        desc : "Charger Box Status".to_string(), key : None, is_ext : None,
        sim_freq : 1000f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "Charger/Box/Voltage"; let
    _____START_DEBUG_FIELD_NAME = "Charger/Box/Current"; let mut vec_points :
    Vec < SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 0f32, max : 12f32, inc_min : 0.1f32, inc_max : 1f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 16usize, parse : None, signed : Some(true), endianness : None,
        format : Some("divide10".to_string()), default : None, ieee754_f32 :
        None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x18FF50E5".to_string(), points : vec_points, points_intopic :
        None, unit : "A".to_string(), name :
        "Charger/Box/Current".to_string(), last_update : Instant :: now(),
        desc : "Charger Box Status".to_string(), key : None, is_ext : None,
        sim_freq : 1000f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "Charger/Box/Current"; let
    _____START_DEBUG_FIELD_NAME = "Charger/Box/F_CommTimeout"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Discrete
    { options : vec! [(0f32, 0.95f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x18FF50E5".to_string(), points : vec_points, points_intopic :
        None, unit : "bool".to_string(), name :
        "Charger/Box/F_CommTimeout".to_string(), last_update : Instant ::
        now(), desc : "Charger Box Status".to_string(), key : None, is_ext :
        None, sim_freq : 1000f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "Charger/Box/F_CommTimeout"; let
    _____START_DEBUG_FIELD_NAME = "Charger/Box/F_WrongBatConnection"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Discrete
    { options : vec! [(0f32, 0.95f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x18FF50E5".to_string(), points : vec_points, points_intopic :
        None, unit : "bool".to_string(), name :
        "Charger/Box/F_WrongBatConnection".to_string(), last_update : Instant
        :: now(), desc : "Charger Box Status".to_string(), key : None, is_ext
        : None, sim_freq : 1000f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "Charger/Box/F_WrongBatConnection"; let
    _____START_DEBUG_FIELD_NAME = "Charger/Box/F_VoltageWrong"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Discrete
    { options : vec! [(0f32, 0.95f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x18FF50E5".to_string(), points : vec_points, points_intopic :
        None, unit : "bool".to_string(), name :
        "Charger/Box/F_VoltageWrong".to_string(), last_update : Instant ::
        now(), desc : "Charger Box Status".to_string(), key : None, is_ext :
        None, sim_freq : 1000f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "Charger/Box/F_VoltageWrong"; let
    _____START_DEBUG_FIELD_NAME = "Charger/Box/F_OverTemp"; let mut vec_points
    : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue :: Discrete
    { options : vec! [(0f32, 0.95f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x18FF50E5".to_string(), points : vec_points, points_intopic :
        None, unit : "bool".to_string(), name :
        "Charger/Box/F_OverTemp".to_string(), last_update : Instant :: now(),
        desc : "Charger Box Status".to_string(), key : None, is_ext : None,
        sim_freq : 1000f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "Charger/Box/F_OverTemp"; let
    _____START_DEBUG_FIELD_NAME = "Charger/Box/F_HardwareFailure"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Discrete
    { options : vec! [(0f32, 0.95f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x18FF50E5".to_string(), points : vec_points, points_intopic :
        None, unit : "bool".to_string(), name :
        "Charger/Box/F_HardwareFailure".to_string(), last_update : Instant ::
        now(), desc : "Charger Box Status".to_string(), key : None, is_ext :
        None, sim_freq : 1000f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "Charger/Box/F_HardwareFailure"; let
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
        unit : "C".to_string(), name : "MSB/FL/Temp".to_string(), last_update
        : Instant :: now(), desc : "Front Left MSB Env".to_string(), key :
        None, is_ext : None, sim_freq : 500f32,
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
        unit : "".to_string(), name : "MSB/FL/Humidity".to_string(),
        last_update : Instant :: now(), desc :
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
        unit : "mg".to_string(), name : "MSB/FL/Accel".to_string(),
        last_update : Instant :: now(), desc :
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
        unit : "mdps".to_string(), name : "MSB/FL/Gyro".to_string(),
        last_update : Instant :: now(), desc :
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
        unit : "".to_string(), name : "MSB/FL/Strain".to_string(), last_update
        : Instant :: now(), desc : "Front Left MSB Strain".to_string(), key :
        None, is_ext : None, sim_freq : 500f32,
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
        unit : "".to_string(), name : "MSB/FL/Shock".to_string(), last_update
        : Instant :: now(), desc : "Front Left Shockpot".to_string(), key :
        None, is_ext : None, sim_freq : 500f32,
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
        unit : "".to_string(), name : "MSB/FL/RideHeight".to_string(),
        last_update : Instant :: now(), desc :
        "Front Left Ride Height".to_string(), key : None, is_ext : None,
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
        unit : "C".to_string(), name : "MSB/FL/WheelTemp".to_string(),
        last_update : Instant :: now(), desc :
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
        unit : "C".to_string(), name : "MSB/FR/Temp".to_string(), last_update
        : Instant :: now(), desc : "Front Right MSB Env".to_string(), key :
        None, is_ext : None, sim_freq : 500f32,
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
        unit : "".to_string(), name : "MSB/FR/Humidity".to_string(),
        last_update : Instant :: now(), desc :
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
        unit : "mg".to_string(), name : "MSB/FR/Accel".to_string(),
        last_update : Instant :: now(), desc :
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
        unit : "mdps".to_string(), name : "MSB/FR/Gyro".to_string(),
        last_update : Instant :: now(), desc :
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
        unit : "".to_string(), name : "MSB/FR/Strain".to_string(), last_update
        : Instant :: now(), desc : "Front Right MSB Strain".to_string(), key :
        None, is_ext : None, sim_freq : 500f32,
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
        unit : "".to_string(), name : "MSB/FR/Shock".to_string(), last_update
        : Instant :: now(), desc : "Front Right Shockpot".to_string(), key :
        None, is_ext : None, sim_freq : 500f32,
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
        unit : "".to_string(), name : "MSB/FR/RideHeight".to_string(),
        last_update : Instant :: now(), desc :
        "Front Right Ride Height".to_string(), key : None, is_ext : None,
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
        unit : "C".to_string(), name : "MSB/FR/WheelTemp".to_string(),
        last_update : Instant :: now(), desc :
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
        unit : "C".to_string(), name : "MSB/BL/Temp".to_string(), last_update
        : Instant :: now(), desc : "Back Left MSB Env".to_string(), key :
        None, is_ext : None, sim_freq : 500f32,
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
        unit : "".to_string(), name : "MSB/BL/Humidity".to_string(),
        last_update : Instant :: now(), desc :
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
        unit : "mg".to_string(), name : "MSB/BL/Accel".to_string(),
        last_update : Instant :: now(), desc :
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
        unit : "mdps".to_string(), name : "MSB/BL/Gyro".to_string(),
        last_update : Instant :: now(), desc :
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
        unit : "".to_string(), name : "MSB/BL/Strain".to_string(), last_update
        : Instant :: now(), desc : "Back Left MSB Strain".to_string(), key :
        None, is_ext : None, sim_freq : 500f32,
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
        unit : "".to_string(), name : "MSB/BL/Shock".to_string(), last_update
        : Instant :: now(), desc : "Back Left Shockpot".to_string(), key :
        None, is_ext : None, sim_freq : 500f32,
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
        unit : "".to_string(), name : "MSB/BL/RideHeight".to_string(),
        last_update : Instant :: now(), desc :
        "Front Back Ride Height".to_string(), key : None, is_ext : None,
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
        unit : "C".to_string(), name : "MSB/BL/WheelTemp".to_string(),
        last_update : Instant :: now(), desc :
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
        unit : "C".to_string(), name : "MSB/BR/Temp".to_string(), last_update
        : Instant :: now(), desc : "Back Right MSB Env".to_string(), key :
        None, is_ext : None, sim_freq : 500f32,
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
        unit : "".to_string(), name : "MSB/BR/Humidity".to_string(),
        last_update : Instant :: now(), desc :
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
        unit : "mg".to_string(), name : "MSB/BR/Accel".to_string(),
        last_update : Instant :: now(), desc :
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
        unit : "mdps".to_string(), name : "MSB/BR/Gyro".to_string(),
        last_update : Instant :: now(), desc :
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
        unit : "".to_string(), name : "MSB/BR/Strain".to_string(), last_update
        : Instant :: now(), desc : "Back Right MSB Strain".to_string(), key :
        None, is_ext : None, sim_freq : 500f32,
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
        unit : "".to_string(), name : "MSB/BR/Shock".to_string(), last_update
        : Instant :: now(), desc : "Back Right Shockpot".to_string(), key :
        None, is_ext : None, sim_freq : 500f32,
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
        unit : "".to_string(), name : "MSB/BR/RideHeight".to_string(),
        last_update : Instant :: now(), desc :
        "Back Right Ride Height".to_string(), key : None, is_ext : None,
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
        unit : "C".to_string(), name : "MSB/BR/WheelTemp".to_string(),
        last_update : Instant :: now(), desc :
        "Back Right Wheel Temp".to_string(), key : None, is_ext : None,
        sim_freq : 500f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MSB/BR/WheelTemp"; let
    _____START_DEBUG_FIELD_NAME = "Experiment/Pack/PerCell/Cell-{}/Attr/{}";
    let mut vec_points_in_topic : Vec < SimPoint > = Vec :: new(); let
    __new_value = SimValue :: Range
    {
        min : 16f32, max : 20f32, inc_min : 1f32, inc_max : 3f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points_in_topic.push(__new_point); let __new_value = SimValue ::
    Range
    {
        min : 1f32, max : 5f32, inc_min : 1f32, inc_max : 3f32, round : false,
        current : 0.0
    }; let __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points_in_topic.push(__new_point); let mut vec_points : Vec <
    SimPoint > = Vec :: new(); let __new_value = SimValue :: Discrete
    { options : vec! [(0f32, 0.5f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_value = SimValue :: Discrete
    { options : vec! [(2f32, 0.5f32), (3f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_value = SimValue :: Discrete
    { options : vec! [(4f32, 0.5f32), (5f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_value = SimValue :: Discrete
    { options : vec! [(6f32, 0.5f32), (7f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x1337".to_string(), points : vec_points, points_intopic :
        Some(vec_points_in_topic), unit : "V".to_string(), name :
        "Experiment/Pack/PerCell/Cell-{}/Attr/{}".to_string(), last_update :
        Instant :: now(), desc :
        "Experimental Segment Temperatures".to_string(), key : None, is_ext :
        None, sim_freq : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "Experiment/Pack/PerCell/Cell-{}/Attr/{}";
    let _____START_DEBUG_FIELD_NAME =
    "Experiment/Pack/PerCell/Cell-{}/Attr/{}"; let mut vec_points_in_topic :
    Vec < SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 16f32, max : 20f32, inc_min : 1f32, inc_max : 3f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points_in_topic.push(__new_point); let __new_value = SimValue ::
    Range
    {
        min : 1f32, max : 5f32, inc_min : 1f32, inc_max : 3f32, round : false,
        current : 0.0
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
        id : "0x1337".to_string(), points : vec_points, points_intopic :
        Some(vec_points_in_topic), unit : "Ah".to_string(), name :
        "Experiment/Pack/PerCell/Cell-{}/Attr/{}".to_string(), last_update :
        Instant :: now(), desc :
        "Experimental Segment Temperatures".to_string(), key : None, is_ext :
        None, sim_freq : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "Experiment/Pack/PerCell/Cell-{}/Attr/{}";
    let _____START_DEBUG_FIELD_NAME =
    "Experiment/Pack/PerCell/Cell-{}/Attr/{}"; let mut vec_points_in_topic :
    Vec < SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 11f32, max : 15f32, inc_min : 1f32, inc_max : 3f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points_in_topic.push(__new_point); let __new_value = SimValue ::
    Range
    {
        min : 6f32, max : 10f32, inc_min : 1f32, inc_max : 3f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points_in_topic.push(__new_point); let mut vec_points : Vec <
    SimPoint > = Vec :: new(); let __new_value = SimValue :: Discrete
    { options : vec! [(8f32, 0.5f32), (9f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_value = SimValue :: Discrete
    { options : vec! [(10f32, 0.5f32), (11f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_value = SimValue :: Discrete
    { options : vec! [(12f32, 0.5f32), (13f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_value = SimValue :: Discrete
    { options : vec! [(14f32, 0.5f32), (15f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x1337".to_string(), points : vec_points, points_intopic :
        Some(vec_points_in_topic), unit : "V".to_string(), name :
        "Experiment/Pack/PerCell/Cell-{}/Attr/{}".to_string(), last_update :
        Instant :: now(), desc :
        "Experimental Segment Temperatures".to_string(), key : None, is_ext :
        None, sim_freq : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "Experiment/Pack/PerCell/Cell-{}/Attr/{}";
    let _____START_DEBUG_FIELD_NAME =
    "Experiment/Pack/PerCell/Cell-{}/Attr/{}"; let mut vec_points_in_topic :
    Vec < SimPoint > = Vec :: new(); let __new_value = SimValue :: Range
    {
        min : 11f32, max : 15f32, inc_min : 1f32, inc_max : 3f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points_in_topic.push(__new_point); let __new_value = SimValue ::
    Range
    {
        min : 6f32, max : 10f32, inc_min : 1f32, inc_max : 3f32, round :
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
        id : "0x1337".to_string(), points : vec_points, points_intopic :
        Some(vec_points_in_topic), unit : "Ah".to_string(), name :
        "Experiment/Pack/PerCell/Cell-{}/Attr/{}".to_string(), last_update :
        Instant :: now(), desc :
        "Experimental Segment Temperatures".to_string(), key : None, is_ext :
        None, sim_freq : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "Experiment/Pack/PerCell/Cell-{}/Attr/{}";
    let _____START_DEBUG_FIELD_NAME = "MPU/State/Mode"; let mut vec_points :
    Vec < SimPoint > = Vec :: new(); let __new_value = SimValue :: Discrete
    {
        options : vec!
        [(0f32, 0.01f32), (1f32, 0.78999996f32), (2f32, 0.89f32),
        (3f32, 0.99f32), (4f32, 1f32)], current : 0.0
    }; let __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x501".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name : "MPU/State/Mode".to_string(),
        last_update : Instant :: now(), desc : "MPU Status".to_string(), key :
        None, is_ext : None, sim_freq : 250f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MPU/State/Mode"; let
    _____START_DEBUG_FIELD_NAME = "MPU/State/ModeIndex"; let mut vec_points :
    Vec < SimPoint > = Vec :: new(); let __new_value = SimValue :: Discrete
    {
        options : vec!
        [(0f32, 0.2f32), (1f32, 0.4f32), (2f32, 0.6f32),
        (3f32, 0.70000005f32), (4f32, 0.8000001f32), (5f32, 0.9000001f32),
        (6f32, 1.0000001f32)], current : 0.0
    }; let __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x501".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name : "MPU/State/ModeIndex".to_string(),
        last_update : Instant :: now(), desc : "MPU Status".to_string(), key :
        None, is_ext : None, sim_freq : 250f32,
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
        unit : "mph".to_string(), name : "MPU/State/Speed".to_string(),
        last_update : Instant :: now(), desc : "MPU Status".to_string(), key :
        None, is_ext : None, sim_freq : 250f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MPU/State/Speed"; let
    _____START_DEBUG_FIELD_NAME = "MPU/State/TSMS"; let mut vec_points : Vec <
    SimPoint > = Vec :: new(); let __new_value = SimValue :: Discrete
    { options : vec! [(0f32, 0.8f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x501".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name : "MPU/State/TSMS".to_string(),
        last_update : Instant :: now(), desc : "MPU Status".to_string(), key :
        None, is_ext : None, sim_freq : 250f32,
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
        unit : "percentage".to_string(), name :
        "MPU/State/TorqueLimit".to_string(), last_update : Instant :: now(),
        desc : "MPU Status".to_string(), key : None, is_ext : None, sim_freq :
        250f32,
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
        unit : "g".to_string(), name : "MPU/Sense/Accel".to_string(),
        last_update : Instant :: now(), desc :
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
        unit : "".to_string(), name : "MPU/Sense/Gyro".to_string(),
        last_update : Instant :: now(), desc : "MPU Sense Gyro".to_string(),
        key : None, is_ext : None, sim_freq : 250f32,
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
        unit : "C".to_string(), name : "MPU/Sense/Temp_IMU".to_string(),
        last_update : Instant :: now(), desc : "MPU Sense Temp".to_string(),
        key : None, is_ext : None, sim_freq : 1000f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MPU/Sense/Temp_IMU"; let
    _____START_DEBUG_FIELD_NAME = "MPU/Fault/Code"; let mut vec_points : Vec <
    SimPoint > = Vec :: new(); let __new_value = SimValue :: Discrete
    {
        options : vec!
        [(0f32, 0.84f32), (1f32, 0.84999996f32), (2f32, 0.85999995f32),
        (4f32, 0.86999995f32), (8f32, 0.87999994f32), (16f32, 0.8899999f32),
        (32f32, 0.8999999f32), (64f32, 0.9099999f32), (128f32, 0.9199999f32),
        (256f32, 0.9299999f32), (512f32, 0.9399999f32),
        (1024f32, 0.94999987f32), (2048f32, 0.95999986f32),
        (3840f32, 0.96999985f32), (4096f32, 0.97999984f32),
        (8192f32, 0.98999983f32), (16384f32, 0.9999998f32)], current : 0.0
    }; let __new_point = SimPoint
    {
        size : 32usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x502".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name : "MPU/Fault/Code".to_string(),
        last_update : Instant :: now(), desc : "MPU Fault".to_string(), key :
        None, is_ext : None, sim_freq : 250f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MPU/Fault/Code"; let
    _____START_DEBUG_FIELD_NAME = "MPU/Fault/Severity"; let mut vec_points :
    Vec < SimPoint > = Vec :: new(); let __new_value = SimValue :: Discrete
    {
        options : vec!
        [(0f32, 0.75f32), (1f32, 0.8f32), (2f32, 0.85f32),
        (3f32, 0.90000004f32), (4f32, 0.95000005f32), (5f32, 1f32)], current :
        0.0
    }; let __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x502".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name : "MPU/Fault/Severity".to_string(),
        last_update : Instant :: now(), desc : "MPU Fault".to_string(), key :
        None, is_ext : None, sim_freq : 250f32,
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
        unit : "V".to_string(), name : "MPU/Sense/Voltage".to_string(),
        last_update : Instant :: now(), desc :
        "MPU Sense Voltage".to_string(), key : None, is_ext : None, sim_freq :
        1000f32,
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
        unit : "%".to_string(), name : "MPU/Sense/SOC".to_string(),
        last_update : Instant :: now(), desc :
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
        unit : "".to_string(), name : "MPU/Pedals/Accelerator_1".to_string(),
        last_update : Instant :: now(), desc : "MPU Accel Pedals".to_string(),
        key : None, is_ext : None, sim_freq : 100f32,
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
        unit : "".to_string(), name : "MPU/Pedals/Accelerator_2".to_string(),
        last_update : Instant :: now(), desc : "MPU Accel Pedals".to_string(),
        key : None, is_ext : None, sim_freq : 100f32,
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
        unit : "".to_string(), name : "MPU/Pedals/Brake_1".to_string(),
        last_update : Instant :: now(), desc : "MPU Brake Pedals".to_string(),
        key : None, is_ext : None, sim_freq : 100f32,
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
        unit : "".to_string(), name : "MPU/Pedals/Brake_2".to_string(),
        last_update : Instant :: now(), desc : "MPU Brake Pedals".to_string(),
        key : None, is_ext : None, sim_freq : 100f32,
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
        unit : "".to_string(), name : "MPU/Debug/Spare0".to_string(),
        last_update : Instant :: now(), desc : "MPU Debug".to_string(), key :
        None, is_ext : None, sim_freq : 1000f32,
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
        unit : "".to_string(), name : "MPU/Debug/Spare1".to_string(),
        last_update : Instant :: now(), desc : "MPU Debug".to_string(), key :
        None, is_ext : None, sim_freq : 1000f32,
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
        unit : "".to_string(), name : "MPU/Debug/Spare2".to_string(),
        last_update : Instant :: now(), desc : "MPU Debug".to_string(), key :
        None, is_ext : None, sim_freq : 1000f32,
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
        unit : "".to_string(), name : "MPU/Debug/Spare3".to_string(),
        last_update : Instant :: now(), desc : "MPU Debug".to_string(), key :
        None, is_ext : None, sim_freq : 1000f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MPU/Debug/Spare3"; let
    _____START_DEBUG_FIELD_NAME = "MPU/Shutdown/CockpitBRB"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Discrete
    { options : vec! [(0f32, 0.95f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x123".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name : "MPU/Shutdown/CockpitBRB".to_string(),
        last_update : Instant :: now(), desc : "MPU Shutdown".to_string(), key
        : None, is_ext : None, sim_freq : 1000f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MPU/Shutdown/CockpitBRB"; let
    _____START_DEBUG_FIELD_NAME = "MPU/Shutdown/BMS"; let mut vec_points : Vec
    < SimPoint > = Vec :: new(); let __new_value = SimValue :: Discrete
    { options : vec! [(0f32, 0.95f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x123".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name : "MPU/Shutdown/BMS".to_string(),
        last_update : Instant :: now(), desc : "MPU Shutdown".to_string(), key
        : None, is_ext : None, sim_freq : 1000f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MPU/Shutdown/BMS"; let
    _____START_DEBUG_FIELD_NAME = "MPU/Shutdown/Inertia"; let mut vec_points :
    Vec < SimPoint > = Vec :: new(); let __new_value = SimValue :: Discrete
    { options : vec! [(0f32, 0.95f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x123".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name : "MPU/Shutdown/Inertia".to_string(),
        last_update : Instant :: now(), desc : "MPU Shutdown".to_string(), key
        : None, is_ext : None, sim_freq : 1000f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MPU/Shutdown/Inertia"; let
    _____START_DEBUG_FIELD_NAME = "MPU/Shutdown/Spare_GPIO_1"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Discrete
    { options : vec! [(0f32, 0.95f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x123".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name : "MPU/Shutdown/Spare_GPIO_1".to_string(),
        last_update : Instant :: now(), desc : "MPU Shutdown".to_string(), key
        : None, is_ext : None, sim_freq : 1000f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MPU/Shutdown/Spare_GPIO_1"; let
    _____START_DEBUG_FIELD_NAME = "MPU/Shutdown/IMD"; let mut vec_points : Vec
    < SimPoint > = Vec :: new(); let __new_value = SimValue :: Discrete
    { options : vec! [(0f32, 0.95f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x123".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name : "MPU/Shutdown/IMD".to_string(),
        last_update : Instant :: now(), desc : "MPU Shutdown".to_string(), key
        : None, is_ext : None, sim_freq : 1000f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MPU/Shutdown/IMD"; let
    _____START_DEBUG_FIELD_NAME = "MPU/Shutdown/BSPD"; let mut vec_points :
    Vec < SimPoint > = Vec :: new(); let __new_value = SimValue :: Discrete
    { options : vec! [(0f32, 0.95f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x123".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name : "MPU/Shutdown/BSPD".to_string(),
        last_update : Instant :: now(), desc : "MPU Shutdown".to_string(), key
        : None, is_ext : None, sim_freq : 1000f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MPU/Shutdown/BSPD"; let
    _____START_DEBUG_FIELD_NAME = "MPU/Shutdown/BOTS"; let mut vec_points :
    Vec < SimPoint > = Vec :: new(); let __new_value = SimValue :: Discrete
    { options : vec! [(0f32, 0.95f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x123".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name : "MPU/Shutdown/BOTS".to_string(),
        last_update : Instant :: now(), desc : "MPU Shutdown".to_string(), key
        : None, is_ext : None, sim_freq : 1000f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MPU/Shutdown/BOTS"; let
    _____START_DEBUG_FIELD_NAME = "MPU/Shutdown/HVD_Interlock"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Discrete
    { options : vec! [(0f32, 0.95f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x123".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name :
        "MPU/Shutdown/HVD_Interlock".to_string(), last_update : Instant ::
        now(), desc : "MPU Shutdown".to_string(), key : None, is_ext : None,
        sim_freq : 1000f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MPU/Shutdown/HVD_Interlock"; let
    _____START_DEBUG_FIELD_NAME = "MPU/Shutdown/HVC_Interlock"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Discrete
    { options : vec! [(0f32, 0.95f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x123".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name :
        "MPU/Shutdown/HVC_Interlock".to_string(), last_update : Instant ::
        now(), desc : "MPU Shutdown".to_string(), key : None, is_ext : None,
        sim_freq : 1000f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MPU/Shutdown/HVC_Interlock"; let
    _____START_DEBUG_FIELD_NAME = "MPU/Fuses/Battbox"; let mut vec_points :
    Vec < SimPoint > = Vec :: new(); let __new_value = SimValue :: Discrete
    { options : vec! [(0f32, 0.95f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x111".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name : "MPU/Fuses/Battbox".to_string(),
        last_update : Instant :: now(), desc : "MPU Fuses".to_string(), key :
        None, is_ext : None, sim_freq : 1000f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MPU/Fuses/Battbox"; let
    _____START_DEBUG_FIELD_NAME = "MPU/Fuses/LVBox"; let mut vec_points : Vec
    < SimPoint > = Vec :: new(); let __new_value = SimValue :: Discrete
    { options : vec! [(0f32, 0.95f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x111".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name : "MPU/Fuses/LVBox".to_string(),
        last_update : Instant :: now(), desc : "MPU Fuses".to_string(), key :
        None, is_ext : None, sim_freq : 1000f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MPU/Fuses/LVBox"; let
    _____START_DEBUG_FIELD_NAME = "MPU/Fuses/FanRadiator"; let mut vec_points
    : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue :: Discrete
    { options : vec! [(0f32, 0.95f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x111".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name : "MPU/Fuses/FanRadiator".to_string(),
        last_update : Instant :: now(), desc : "MPU Fuses".to_string(), key :
        None, is_ext : None, sim_freq : 1000f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MPU/Fuses/FanRadiator"; let
    _____START_DEBUG_FIELD_NAME = "MPU/Fuses/MC"; let mut vec_points : Vec <
    SimPoint > = Vec :: new(); let __new_value = SimValue :: Discrete
    { options : vec! [(0f32, 0.95f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x111".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name : "MPU/Fuses/MC".to_string(), last_update
        : Instant :: now(), desc : "MPU Fuses".to_string(), key : None, is_ext
        : None, sim_freq : 1000f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MPU/Fuses/MC"; let
    _____START_DEBUG_FIELD_NAME = "MPU/Fuses/FanBattbox"; let mut vec_points :
    Vec < SimPoint > = Vec :: new(); let __new_value = SimValue :: Discrete
    { options : vec! [(0f32, 0.95f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x111".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name : "MPU/Fuses/FanBattbox".to_string(),
        last_update : Instant :: now(), desc : "MPU Fuses".to_string(), key :
        None, is_ext : None, sim_freq : 1000f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MPU/Fuses/FanBattbox"; let
    _____START_DEBUG_FIELD_NAME = "MPU/Fuses/Pump"; let mut vec_points : Vec <
    SimPoint > = Vec :: new(); let __new_value = SimValue :: Discrete
    { options : vec! [(0f32, 0.95f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x111".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name : "MPU/Fuses/Pump".to_string(),
        last_update : Instant :: now(), desc : "MPU Fuses".to_string(), key :
        None, is_ext : None, sim_freq : 1000f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MPU/Fuses/Pump"; let
    _____START_DEBUG_FIELD_NAME = "MPU/Fuses/Dashboard"; let mut vec_points :
    Vec < SimPoint > = Vec :: new(); let __new_value = SimValue :: Discrete
    { options : vec! [(0f32, 0.95f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x111".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name : "MPU/Fuses/Dashboard".to_string(),
        last_update : Instant :: now(), desc : "MPU Fuses".to_string(), key :
        None, is_ext : None, sim_freq : 1000f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MPU/Fuses/Dashboard"; let
    _____START_DEBUG_FIELD_NAME = "MPU/Fuses/Brakelight"; let mut vec_points :
    Vec < SimPoint > = Vec :: new(); let __new_value = SimValue :: Discrete
    { options : vec! [(0f32, 0.95f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x111".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name : "MPU/Fuses/Brakelight".to_string(),
        last_update : Instant :: now(), desc : "MPU Fuses".to_string(), key :
        None, is_ext : None, sim_freq : 1000f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MPU/Fuses/Brakelight"; let
    _____START_DEBUG_FIELD_NAME = "MPU/Fuses/BRB"; let mut vec_points : Vec <
    SimPoint > = Vec :: new(); let __new_value = SimValue :: Discrete
    { options : vec! [(0f32, 0.95f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x111".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name : "MPU/Fuses/BRB".to_string(), last_update
        : Instant :: now(), desc : "MPU Fuses".to_string(), key : None, is_ext
        : None, sim_freq : 1000f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "MPU/Fuses/BRB"; let
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
        unit : "Z".to_string(), name :
        "Calypso/Bidir/State/FirstOff/A".to_string(), last_update : Instant ::
        now(), desc : "Example Enc Msg".to_string(), key :
        Some("FirstOff".to_string()), is_ext : None, sim_freq : 750f32,
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
        unit : "G".to_string(), name :
        "Calypso/Bidir/State/FirstOff/B".to_string(), last_update : Instant ::
        now(), desc : "Example Enc Msg".to_string(), key :
        Some("FirstOff".to_string()), is_ext : None, sim_freq : 750f32,
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
        unit : "G".to_string(), name :
        "Calypso/Bidir/State/FirstOff/C".to_string(), last_update : Instant ::
        now(), desc : "Example Enc Msg".to_string(), key :
        Some("FirstOff".to_string()), is_ext : None, sim_freq : 750f32,
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
        unit : "G".to_string(), name :
        "Calypso/Bidir/State/FirstOff/D".to_string(), last_update : Instant ::
        now(), desc : "Example Enc Msg".to_string(), key :
        Some("FirstOff".to_string()), is_ext : None, sim_freq : 750f32,
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
        None, unit : "GG".to_string(), name :
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
        unit : "pts".to_string(), name : "Calypso/Bidir/Unknown".to_string(),
        last_update : Instant :: now(), desc :
        "Calypso bidir unknown key".to_string(), key : None, is_ext : None,
        sim_freq : 750f32,
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
        unit : "V".to_string(), name : "BMS/Pack/Voltage".to_string(),
        last_update : Instant :: now(), desc :
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
        unit : "A".to_string(), name : "BMS/Pack/Current".to_string(),
        last_update : Instant :: now(), desc :
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
        unit : "Ah".to_string(), name : "BMS/Pack/Amp-hours".to_string(),
        last_update : Instant :: now(), desc :
        "accumulator status".to_string(), key : None, is_ext : None, sim_freq
        : 700f32,
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
        unit : "%".to_string(), name : "BMS/Pack/SOC".to_string(), last_update
        : Instant :: now(), desc : "accumulator status".to_string(), key :
        None, is_ext : None, sim_freq : 700f32,
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
        unit : "%".to_string(), name : "BMS/Pack/Health".to_string(),
        last_update : Instant :: now(), desc :
        "accumulator status".to_string(), key : None, is_ext : None, sim_freq
        : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Pack/Health"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Status/State"; let mut vec_points : Vec
    < SimPoint > = Vec :: new(); let __new_value = SimValue :: Discrete
    {
        options : vec!
        [(0f32, 0.01f32), (1f32, 0.81f32), (2f32, 0.96000004f32),
        (3f32, 1f32)], current : 0.0
    }; let __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x81".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name : "BMS/Status/State".to_string(),
        last_update : Instant :: now(), desc : "BMS Status".to_string(), key :
        None, is_ext : None, sim_freq : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Status/State"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Status/F/CCL_Enforce"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Discrete
    { options : vec! [(0f32, 0.5f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x81".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name : "BMS/Status/F/CCL_Enforce".to_string(),
        last_update : Instant :: now(), desc : "BMS Status".to_string(), key :
        None, is_ext : None, sim_freq : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Status/F/CCL_Enforce"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Status/F/Charger_Can"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Discrete
    { options : vec! [(0f32, 0.5f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x81".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name : "BMS/Status/F/Charger_Can".to_string(),
        last_update : Instant :: now(), desc : "BMS Status".to_string(), key :
        None, is_ext : None, sim_freq : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Status/F/Charger_Can"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Status/F/Battery_Therm"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Discrete
    { options : vec! [(0f32, 0.5f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x81".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name :
        "BMS/Status/F/Battery_Therm".to_string(), last_update : Instant ::
        now(), desc : "BMS Status".to_string(), key : None, is_ext : None,
        sim_freq : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Status/F/Battery_Therm"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Status/F/Charger_Safety"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Discrete
    { options : vec! [(0f32, 0.5f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x81".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name :
        "BMS/Status/F/Charger_Safety".to_string(), last_update : Instant ::
        now(), desc : "BMS Status".to_string(), key : None, is_ext : None,
        sim_freq : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Status/F/Charger_Safety"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Status/F/DCL_Enforce"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Discrete
    { options : vec! [(0f32, 0.5f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x81".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name : "BMS/Status/F/DCL_Enforce".to_string(),
        last_update : Instant :: now(), desc : "BMS Status".to_string(), key :
        None, is_ext : None, sim_freq : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Status/F/DCL_Enforce"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Status/F/External_Can"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Discrete
    { options : vec! [(0f32, 0.5f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x81".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name : "BMS/Status/F/External_Can".to_string(),
        last_update : Instant :: now(), desc : "BMS Status".to_string(), key :
        None, is_ext : None, sim_freq : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Status/F/External_Can"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Status/F/Weak_Pack"; let mut vec_points
    : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue :: Discrete
    { options : vec! [(0f32, 0.5f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x81".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name : "BMS/Status/F/Weak_Pack".to_string(),
        last_update : Instant :: now(), desc : "BMS Status".to_string(), key :
        None, is_ext : None, sim_freq : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Status/F/Weak_Pack"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Status/F/Low_Cell_Volts"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Discrete
    { options : vec! [(0f32, 0.5f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x81".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name :
        "BMS/Status/F/Low_Cell_Volts".to_string(), last_update : Instant ::
        now(), desc : "BMS Status".to_string(), key : None, is_ext : None,
        sim_freq : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Status/F/Low_Cell_Volts"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Status/F/Charge_Reading"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Discrete
    { options : vec! [(0f32, 0.5f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x81".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name :
        "BMS/Status/F/Charge_Reading".to_string(), last_update : Instant ::
        now(), desc : "BMS Status".to_string(), key : None, is_ext : None,
        sim_freq : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Status/F/Charge_Reading"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Status/F/Current_Sense"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Discrete
    { options : vec! [(0f32, 0.5f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x81".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name :
        "BMS/Status/F/Current_Sense".to_string(), last_update : Instant ::
        now(), desc : "BMS Status".to_string(), key : None, is_ext : None,
        sim_freq : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Status/F/Current_Sense"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Status/F/IC_Comm"; let mut vec_points :
    Vec < SimPoint > = Vec :: new(); let __new_value = SimValue :: Discrete
    { options : vec! [(0f32, 0.5f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x81".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name : "BMS/Status/F/IC_Comm".to_string(),
        last_update : Instant :: now(), desc : "BMS Status".to_string(), key :
        None, is_ext : None, sim_freq : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Status/F/IC_Comm"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Status/F/Thermal_Err"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Discrete
    { options : vec! [(0f32, 0.5f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x81".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name : "BMS/Status/F/Thermal_Err".to_string(),
        last_update : Instant :: now(), desc : "BMS Status".to_string(), key :
        None, is_ext : None, sim_freq : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Status/F/Thermal_Err"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Status/F/Software"; let mut vec_points
    : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue :: Discrete
    { options : vec! [(0f32, 0.5f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x81".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name : "BMS/Status/F/Software".to_string(),
        last_update : Instant :: now(), desc : "BMS Status".to_string(), key :
        None, is_ext : None, sim_freq : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Status/F/Software"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Status/F/Open_Wire"; let mut vec_points
    : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue :: Discrete
    { options : vec! [(0f32, 0.5f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x81".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name : "BMS/Status/F/Open_Wire".to_string(),
        last_update : Instant :: now(), desc : "BMS Status".to_string(), key :
        None, is_ext : None, sim_freq : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Status/F/Open_Wire"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Status/F/Pack_Overheat"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Discrete
    { options : vec! [(0f32, 0.5f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x81".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name :
        "BMS/Status/F/Pack_Overheat".to_string(), last_update : Instant ::
        now(), desc : "BMS Status".to_string(), key : None, is_ext : None,
        sim_freq : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Status/F/Pack_Overheat"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Status/F/Cell_Undervoltage"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Discrete
    { options : vec! [(0f32, 0.5f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x81".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name :
        "BMS/Status/F/Cell_Undervoltage".to_string(), last_update : Instant ::
        now(), desc : "BMS Status".to_string(), key : None, is_ext : None,
        sim_freq : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Status/F/Cell_Undervoltage"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Status/F/Cell_Overvoltage"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Discrete
    { options : vec! [(0f32, 0.5f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x81".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name :
        "BMS/Status/F/Cell_Overvoltage".to_string(), last_update : Instant ::
        now(), desc : "BMS Status".to_string(), key : None, is_ext : None,
        sim_freq : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Status/F/Cell_Overvoltage"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Status/F/Cells_Not_Balancing"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Discrete
    { options : vec! [(0f32, 0.5f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 1usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x81".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name :
        "BMS/Status/F/Cells_Not_Balancing".to_string(), last_update : Instant
        :: now(), desc : "BMS Status".to_string(), key : None, is_ext : None,
        sim_freq : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Status/F/Cells_Not_Balancing"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Status/Temp_Average"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Range
    {
        min : 10f32, max : 50f32, inc_min : 0.01f32, inc_max : 0.5f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x81".to_string(), points : vec_points, points_intopic : None,
        unit : "C".to_string(), name : "BMS/Status/Temp_Average".to_string(),
        last_update : Instant :: now(), desc : "BMS Status".to_string(), key :
        None, is_ext : None, sim_freq : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Status/Temp_Average"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Status/Temp_Internal"; let mut
    vec_points : Vec < SimPoint > = Vec :: new(); let __new_value = SimValue
    :: Range
    {
        min : 10f32, max : 50f32, inc_min : 1f32, inc_max : 1f32, round :
        false, current : 0.0
    }; let __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x81".to_string(), points : vec_points, points_intopic : None,
        unit : "C".to_string(), name : "BMS/Status/Temp_Internal".to_string(),
        last_update : Instant :: now(), desc : "BMS Status".to_string(), key :
        None, is_ext : None, sim_freq : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Status/Temp_Internal"; let
    _____START_DEBUG_FIELD_NAME = "BMS/Status/Balancing"; let mut vec_points :
    Vec < SimPoint > = Vec :: new(); let __new_value = SimValue :: Discrete
    { options : vec! [(0f32, 0.4f32), (1f32, 1f32)], current : 0.0 }; let
    __new_point = SimPoint
    {
        size : 8usize, parse : None, signed : None, endianness : None, format
        : None, default : None, ieee754_f32 : None, value : __new_value,
    }; vec_points.push(__new_point); let __new_component = SimComponent
    {
        id : "0x81".to_string(), points : vec_points, points_intopic : None,
        unit : "".to_string(), name : "BMS/Status/Balancing".to_string(),
        last_update : Instant :: now(), desc : "BMS Status".to_string(), key :
        None, is_ext : None, sim_freq : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Status/Balancing"; let
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
        unit : "V".to_string(), name :
        "BMS/Cells/Volts_High_Value".to_string(), last_update : Instant ::
        now(), desc : "Cell Data".to_string(), key : None, is_ext : None,
        sim_freq : 700f32,
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
        unit : "".to_string(), name : "BMS/Cells/Volts_High_Chip".to_string(),
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
        unit : "".to_string(), name : "BMS/Cells/Volts_High_Cell".to_string(),
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
        unit : "V".to_string(), name :
        "BMS/Cells/Volts_Low_Value".to_string(), last_update : Instant ::
        now(), desc : "Cell Data".to_string(), key : None, is_ext : None,
        sim_freq : 700f32,
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
        unit : "".to_string(), name : "BMS/Cells/Volts_Low_Chip".to_string(),
        last_update : Instant :: now(), desc : "Cell Data".to_string(), key :
        None, is_ext : None, sim_freq : 700f32,
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
        unit : "".to_string(), name : "BMS/Cells/Volts_Low_Cell".to_string(),
        last_update : Instant :: now(), desc : "Cell Data".to_string(), key :
        None, is_ext : None, sim_freq : 700f32,
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
        unit : "V".to_string(), name :
        "BMS/Cells/Volts_Avg_Value".to_string(), last_update : Instant ::
        now(), desc : "Cell Data".to_string(), key : None, is_ext : None,
        sim_freq : 700f32,
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
        unit : "C".to_string(), name :
        "BMS/Cells/Temp_High_Value".to_string(), last_update : Instant ::
        now(), desc : "Cell Temperatures".to_string(), key : None, is_ext :
        None, sim_freq : 700f32,
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
        unit : "".to_string(), name : "BMS/Cells/Temp_High_Cell".to_string(),
        last_update : Instant :: now(), desc :
        "Cell Temperatures".to_string(), key : None, is_ext : None, sim_freq :
        700f32,
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
        unit : "".to_string(), name : "BMS/Cells/Temp_High_Chip".to_string(),
        last_update : Instant :: now(), desc :
        "Cell Temperatures".to_string(), key : None, is_ext : None, sim_freq :
        700f32,
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
        unit : "C".to_string(), name : "BMS/Cells/Temp_Low_Value".to_string(),
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
        unit : "".to_string(), name : "BMS/Cells/Temp_Low_Cell".to_string(),
        last_update : Instant :: now(), desc :
        "Cell Temperatures".to_string(), key : None, is_ext : None, sim_freq :
        700f32,
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
        unit : "".to_string(), name : "BMS/Cells/Temp_Low_Chip".to_string(),
        last_update : Instant :: now(), desc :
        "Cell Temperatures".to_string(), key : None, is_ext : None, sim_freq :
        700f32,
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
        unit : "C".to_string(), name : "BMS/Cells/Temp_Avg_Value".to_string(),
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
        unit : "C".to_string(), name : "BMS/Segment_Temp/1".to_string(),
        last_update : Instant :: now(), desc :
        "Segment Temperatures".to_string(), key : None, is_ext : None,
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
        unit : "C".to_string(), name : "BMS/Segment_Temp/2".to_string(),
        last_update : Instant :: now(), desc :
        "Segment Temperatures".to_string(), key : None, is_ext : None,
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
        unit : "C".to_string(), name : "BMS/Segment_Temp/3".to_string(),
        last_update : Instant :: now(), desc :
        "Segment Temperatures".to_string(), key : None, is_ext : None,
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
        unit : "C".to_string(), name : "BMS/Segment_Temp/4".to_string(),
        last_update : Instant :: now(), desc :
        "Segment Temperatures".to_string(), key : None, is_ext : None,
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
        unit : "C".to_string(), name : "BMS/Segment_Temp/5".to_string(),
        last_update : Instant :: now(), desc :
        "Segment Temperatures".to_string(), key : None, is_ext : None,
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
        unit : "C".to_string(), name : "BMS/Segment_Temp/6".to_string(),
        last_update : Instant :: now(), desc :
        "Segment Temperatures".to_string(), key : None, is_ext : None,
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
        unit : "".to_string(), name : "BMS/Pack/DCL".to_string(), last_update
        : Instant :: now(), desc : "Current Limits".to_string(), key : None,
        is_ext : None, sim_freq : 700f32,
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
        unit : "".to_string(), name : "BMS/Pack/CCL".to_string(), last_update
        : Instant :: now(), desc : "Current Limits".to_string(), key : None,
        is_ext : None, sim_freq : 700f32,
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
        unit : "".to_string(), name : "BMS/Pack/Current".to_string(),
        last_update : Instant :: now(), desc : "Current Limits".to_string(),
        key : None, is_ext : None, sim_freq : 700f32,
    }; __all_sim_components.push(__new_component); let
    _____END___DEBUG_FIELD_NAME = "BMS/Pack/Current";
    __all_sim_components.iter_mut().for_each(| c | c.initialize());
    __all_sim_components
}