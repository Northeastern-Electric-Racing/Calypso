use crate::can_types::*;
use proc_macro2::Literal;
use proc_macro2::TokenStream as ProcMacro2TokenStream;
use quote::{format_ident, quote};

/**
 *  Function to generate SimComponentAttrs and SimComponents for a CANMsg
 */
pub fn gen_sim_msg(msg: &mut CANMsg) -> ProcMacro2TokenStream {
    // if let Some(freq) = msg.sim_freq {
    //     let mut attrs = ProcMacro2TokenStream::new();
    //     let mut sims = ProcMacro2TokenStream::new();

    //     for mut point in msg.points {
    //         // TODO use this
    //         sims.extend(gen_sim_point(point));
    //     }

    //     for mut field in msg.fields {
    //         // TODO refactor how sim works! 
    //         attrs.extend(gen_sim_component_attr(freq, msg.id.clone(), field));
    //         sims.extend(gen_sim_field(field, msg.points));
    //     }

    // } else {
    //     quote! { }
    // }
    quote! { }
}

/**
 *  Function to generate variable name for a SimComponentAttr  
 */
fn gen_sim_component_attr_name(field: &mut NetField) -> ProcMacro2TokenStream {
    // format_ident!("{}_attr", field.name.clone().to_lowercase().replace(['/', ' ', '-'], "_"));
    quote! {}
}


/**
 *  Function to generate SimComponentAttr for a NetField 
 */
fn gen_sim_component_attr(sim_freq: f32, id: String, field: &mut NetField) -> ProcMacro2TokenStream {
    // let n_canpoints: u32 = field.values.len().try_into().unwrap();
    // let attr_name = gen_sim_component_attr_name(field);
    // quote! { 
    //     let #attr_name: SimComponentAttr = SimComponentAttr {
    //         sim_freq: #sim_freq,
    //         n_canpoints: #n_canpoints,
    //         id: #id.to_string(),
    //     }; 
    // }

    quote! {}
}

/**
 *  Function to generate Sim struct for a CANPoint
 *
 *  Note that the creation of a new Sim (SimSweep or SimEnum) implicitly
 *  creates a new SimComponent
 */
fn gen_sim_point(point: &mut CANPoint) -> ProcMacro2TokenStream {
    // let sim_name = format_ident!(
    //     "{}",
        
    //     )
    // if let Some(sim) = point.sim {
    //     match sim {
    //         sim_sweep @ Sim::SimSweep => {
    //             // TODO handle sim sweep 
    //             quote! { }
    //         },
    //         sim_enum @ Sim::SimEnum => {
    //             // TODO handle sim enum 
    //             quote! { }
    //         }
    //     }
    // } else {
    //     quote! { }
    // }

    quote! { }
}
