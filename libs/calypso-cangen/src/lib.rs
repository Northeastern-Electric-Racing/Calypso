pub mod can_gen_decode;
pub mod can_gen_encode;
pub mod can_gen_simulate;
pub mod can_types;
pub mod validate;
pub mod version;

/**
 *  Path to CAN spec JSON files
 *  Used by all daedalus macros
 *  Filepath is relative to project root (i.e. /Calypso)
 */
pub const CANGEN_SPEC_PATH: &str = "./Embedded-Base/cangen/can-messages/";
