use crate::can_types::*;
use proc_macro2::Literal;
use proc_macro2::TokenStream as ProcMacro2TokenStream;
use quote::{format_ident, quote};


/**
 *  Trait to generate ProcMacro2TokenStreams for simulate function macro
 */
pub trait CANGenSimulate {
    fn gen_simulate_component(&mut self) -> ProcMacro2TokenStream;
}

/**
 *  Function to generate simulate component for a CANMsg
 */
impl CANGenSimulate for CANMsg {
    fn gen_simulate_component(&mut self) -> ProcMacro2TokenStream {
        match self.sim_freq {
           Some(freq) => {
                let field_components = self
                    .fields
                    .iter_mut()
                    .map(|field| field.gen_simulate_component())
                    .collect::<Vec<ProcMacro2TokenStream>>()
                    .into_iter()
                    .fold(ProcMacro2TokenStream::new(), |mut acc, ts| {
                        acc.extend(ts);
                        acc
                    });

                quote! {
                    #field_components
                }
           },
           None => {
               quote! { }
           }
        }
    }
}

/**
 *  Function to generate simulate component for a NetField
 */
impl CANGenSimulate for NetField {
    // Only simulate if all sim values are present
    let simulatable: bool = 
        self.sim_min.is_some() &&
        self.sim_max.is_some() &&
        self.sim_inc_min.is_some() &&
        self.sim_inc_max.is_some();

    match simulatable {
        true => {
            let attr_name = format_ident!(
                "{}_attr",
                self.name.clone().to_lowercase().replace(' ', "_")
            );
            let sim_min: f32 = match self.sim_min {
                Some(sm) => sm,
                None => -1.0
            };
            let sim_max: f32 = match self.sim_min {
                Some(sm) => sm,
                None => -1.0
            };
            let sim_inc_min: f32 = match self.sim_inc_min {
                Some(sm) => sm,
                None => -1.0
            };
            let sim_inc_max: f32 = match self.sim_inc_max {
                Some(sm) => sm,
                None => -1.0
            };
            let n_canpoints: usize = len(self.points);
            let id = self.id;
            quote! {
                let #attr_name: SimulatedComponentAttr = SimulatedComponentAttr {
                    sim_min: #sin_min,
                    sim_max: #sim_max,
                    sim_inc_min: #sim_inc_min,
                    sim_inc_min: #sim_inc_min,
                    sim_freq: 0, // TODO figure out how to pass from parent Msg
                    n_canpoints: #n_canpoints,
                    id: #id.to_string(),
                }; 
            }
        },
        false => {
            quote! { }
        }
   }
}
