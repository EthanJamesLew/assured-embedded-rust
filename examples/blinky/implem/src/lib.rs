//! Implementation for lustre node `blinky` (see [Blinky](struct.Blinky.html)).
//!
//! Code generated by the [Kind 2 model checker][kind 2].
//!
//! [kind 2]: http://kind2-mc.github.io/kind2/ (The Kind 2 model checker)

// Deactiving lint warnings the transformation does not respect.
#![no_std]
#![allow(
  non_upper_case_globals, non_snake_case, non_camel_case_types,
  unused_variables, unused_parens
)]

// TODO: [CODEGEN] don't dump this into module
use Lustre::*;

// No Entry point.


/// Stores the state for **top node** `blinky`.
///
/// # Inputs
///
/// | Lustre identifier | Type |
/// |:---:|:---|
/// | `toggle` | Bool |
///
/// # Outputs
///
/// | Lustre identifier | Type |
/// |:---:|:---|
/// | `light_index` | Int |
/// | `time` | Int |
///
/// # Sub systems
///
/// | Lustre identifier | Struct | Inputs | Outputs | Position |
/// |:---:|:---:|:---:|:---:|:---:|
/// | `hasHappened` | [HasHappened](struct.HasHappened.html) | `gklocal_8` | `call_9` | [blinky.lus line 40](../src/lus/blinky.lus.html#40) |
/// | `hasHappened` | [HasHappened](struct.HasHappened.html) | `gklocal_5` | `call_6` | [blinky.lus line 43](../src/lus/blinky.lus.html#43) |
/// | `sofar` | [Sofar](struct.Sofar.html) | `on` | `call_1` | [blinky.lus line 43](../src/lus/blinky.lus.html#43) |
///
/// # Assertions
///
/// /// No assertions for this system.
///
/// # Assumptions
///
/// No assumptions for this system.
///
#[derive(Copy, Clone)] 
pub struct Blinky {
  /// Input: `blinky.usr.toggle`
  pub svar_toggle: Bool,

  /// Output: `blinky.usr.light_index`
  pub svar_light_index: Int,
  /// Output: `blinky.usr.time`
  pub svar_time: Int,

  /// Local: `blinky.res.sofar`
  pub svar_sofar: Bool,
  /// Local, ghost: `blinky.contract.2.usr.on`
  pub svar_on: Bool,
  /// Local, call: `blinky.res.call_9`
  pub svar_call_9: Bool,
  /// Local, call: `blinky.res.call_6`
  pub svar_call_6: Bool,
  /// Local, call: `blinky.res.call_1`
  pub svar_call_1: Bool,
  /// Local, invisible local: `blinky.res.gklocal_8`
  pub svar_gklocal_8: Bool,
  /// Local, invisible local: `blinky.res.gklocal_5`
  pub svar_gklocal_5: Bool,
  /// Local, invisble ghost: `blinky.res.glocal_16`
  pub svar_glocal_16: Bool,
  /// Local, invisble ghost: `blinky.res.glocal_15`
  pub svar_glocal_15: Bool,
  /// Local, invisble ghost: `blinky.res.glocal_14`
  pub svar_glocal_14: Bool,
  /// Local, invisble ghost: `blinky.res.glocal_13`
  pub svar_glocal_13: Bool,
  /// Local, invisble ghost: `blinky.res.glocal_12`
  pub svar_glocal_12: Bool,
  /// Local, invisble ghost: `blinky.res.glocal_11`
  pub svar_glocal_11: Bool,
  /// Local, invisble ghost: `blinky.res.glocal_10`
  pub svar_glocal_10: Bool,
  /// Local, invisble ghost: `blinky.res.glocal_7`
  pub svar_glocal_7: Bool,
  /// Local: `blinky.res.glocal_4`
  pub svar_glocal_4: Int,
  /// Local: `blinky.res.glocal_3`
  pub svar_glocal_3: Int,
  /// Local: `blinky.res.glocal_2`
  pub svar_glocal_2: Int,
  /// Local, local: `blinky.impl.usr.running`
  pub svar_running: Bool,

  /// Call to `sofar` ([blinky.lus line 43](../src/lus/blinky.lus.html#43)).
  pub sofar_2: Sofar,
  /// Call to `hasHappened` ([blinky.lus line 43](../src/lus/blinky.lus.html#43)).
  pub hasHappened_1: HasHappened,
  /// Call to `hasHappened` ([blinky.lus line 40](../src/lus/blinky.lus.html#40)).
  pub hasHappened_0: HasHappened,
}

impl System for Blinky {
  type Input = (
    Bool, // svar_toggle (blinky.usr.toggle)
  ) ;
  type Output = (
    Int, // svar_light_index (blinky.usr.light_index)
    Int, // svar_time (blinky.usr.time)
  ) ;
  fn arity() -> usize { 1 }
  fn init(input: Self::Input) -> Result<Self, ()> {
    // |===| Retrieving inputs.
    let svar_toggle = input.0 ;
    
    // |===| Computing initial state.
    let svar_gklocal_5 = true ;
    let svar_glocal_12 = true ;
    let svar_glocal_15 = true ;
    let svar_on = svar_toggle ;
    let svar_time = 0 ;
    let svar_light_index = 0 ;
    let svar_running = (! (! svar_toggle)) ;
    let svar_sofar = true ;
    let sofar_2 = Sofar::init( (
      svar_on,
    ) )?;
     
              let (
      svar_call_1,
    ) = sofar_2.output() ;
    
    let hasHappened_1 = HasHappened::init( (
      svar_gklocal_5,
    ) )?;
     
              let (
      svar_call_6,
    ) = hasHappened_1.output() ;
    
    let svar_glocal_16 = (svar_time >= 0) ;
    let svar_glocal_14 = (svar_light_index < 4) ;
    let svar_glocal_13 = (svar_light_index >= 0) ;
    let svar_glocal_11 = ((! svar_on) | (svar_light_index == 0)) ;
    let svar_gklocal_8 = (svar_light_index == svar_time) ;
    let svar_glocal_7 = ((! (svar_call_1 & (svar_time >= 4))) | svar_call_6) ;
    let svar_glocal_4 = svar_light_index ;
    let svar_glocal_3 = svar_light_index ;
    let svar_glocal_2 = svar_light_index ;
    let hasHappened_0 = HasHappened::init( (
      svar_gklocal_8,
    ) )?;
     
              let (
      svar_call_9,
    ) = hasHappened_0.output() ;
    
    let svar_glocal_10 = ((! (svar_call_1 & (svar_time == 4))) | svar_call_9) ;
    
    // |===| Checking assertions.
    
    
    
    
    // |===| Returning initial state.
    Ok( Blinky {
      // |===| Inputs.
      svar_toggle: svar_toggle,
      
      // |===| Outputs.
      svar_light_index: svar_light_index,
      svar_time: svar_time,
      
      // |===| Locals.
      svar_sofar: svar_sofar,
      svar_on: svar_on,
      svar_call_9: svar_call_9,
      svar_call_6: svar_call_6,
      svar_call_1: svar_call_1,
      svar_gklocal_8: svar_gklocal_8,
      svar_gklocal_5: svar_gklocal_5,
      svar_glocal_16: svar_glocal_16,
      svar_glocal_15: svar_glocal_15,
      svar_glocal_14: svar_glocal_14,
      svar_glocal_13: svar_glocal_13,
      svar_glocal_12: svar_glocal_12,
      svar_glocal_11: svar_glocal_11,
      svar_glocal_10: svar_glocal_10,
      svar_glocal_7: svar_glocal_7,
      svar_glocal_4: svar_glocal_4,
      svar_glocal_3: svar_glocal_3,
      svar_glocal_2: svar_glocal_2,
      svar_running: svar_running,
      
      // |===| Calls.
      sofar_2: sofar_2,
      hasHappened_1: hasHappened_1,
      hasHappened_0: hasHappened_0,
    } )
  }

  fn next(&mut self, input: Self::Input) -> Result<(), ()> {
    // |===| Retrieving inputs.
    let svar_toggle = input.0 ;
    
    // |===| Computing next state.
    let svar_glocal_2 = self.svar_light_index ;
    let svar_glocal_3 = self.svar_glocal_2 ;
    let svar_glocal_4 = self.svar_glocal_3 ;
    let svar_on = ((self.svar_on & (! svar_toggle)) | ((! self.svar_on) & svar_toggle)) ;
    let svar_time = (self.svar_time + 1) ;
    let svar_running = (! (self.svar_running == svar_toggle)) ;
    let svar_sofar = self.svar_sofar ;
    self.sofar_2.next( (
      svar_on,
    ) )?;
    let sofar_2 = self.sofar_2;
    let (
      svar_call_1,
    ) = sofar_2.output() ;
    let svar_light_index = ( if svar_running { ((self.svar_light_index + 1) % 4) } else {self.svar_light_index } ) ;
    let svar_glocal_16 = (svar_time >= 0) ;
    let svar_glocal_15 = (svar_time == (self.svar_time + 1)) ;
    let svar_glocal_14 = (svar_light_index < 4) ;
    let svar_glocal_13 = (svar_light_index >= 0) ;
    let svar_glocal_12 = ((! (! svar_on)) | (svar_light_index == self.svar_light_index)) ;
    let svar_glocal_11 = ((! svar_on) | (! (svar_light_index == self.svar_light_index))) ;
    let svar_gklocal_8 = (svar_light_index == svar_time) ;
    let svar_gklocal_5 = (svar_light_index == self.svar_glocal_4) ;
    self.hasHappened_1.next( (
      svar_gklocal_5,
    ) )?;
    let hasHappened_1 = self.hasHappened_1;
    let (
      svar_call_6,
    ) = hasHappened_1.output() ;
    self.hasHappened_0.next( (
      svar_gklocal_8,
    ) )?;
    let hasHappened_0 = self.hasHappened_0;
    let (
      svar_call_9,
    ) = hasHappened_0.output() ;
    let svar_glocal_10 = ((! (svar_call_1 & (svar_time == 4))) | svar_call_9) ;
    let svar_glocal_7 = ((! (svar_call_1 & (svar_time >= 4))) | svar_call_6) ;
    
    // |===| Checking assertions.
    
    
    // |===| Checking assumptions.
    
    
    // |===| Updating next state.
    // |===| Inputs.
    self.svar_toggle = svar_toggle ;
    
    // |===| Outputs.
    self.svar_light_index = svar_light_index ;
    self.svar_time = svar_time ;
    
    // |===| Locals.
    self.svar_sofar = svar_sofar ;
    self.svar_on = svar_on ;
    self.svar_call_9 = svar_call_9 ;
    self.svar_call_6 = svar_call_6 ;
    self.svar_call_1 = svar_call_1 ;
    self.svar_gklocal_8 = svar_gklocal_8 ;
    self.svar_gklocal_5 = svar_gklocal_5 ;
    self.svar_glocal_16 = svar_glocal_16 ;
    self.svar_glocal_15 = svar_glocal_15 ;
    self.svar_glocal_14 = svar_glocal_14 ;
    self.svar_glocal_13 = svar_glocal_13 ;
    self.svar_glocal_12 = svar_glocal_12 ;
    self.svar_glocal_11 = svar_glocal_11 ;
    self.svar_glocal_10 = svar_glocal_10 ;
    self.svar_glocal_7 = svar_glocal_7 ;
    self.svar_glocal_4 = svar_glocal_4 ;
    self.svar_glocal_3 = svar_glocal_3 ;
    self.svar_glocal_2 = svar_glocal_2 ;
    self.svar_running = svar_running ;
    
    // |===| Calls.
    self.sofar_2 = sofar_2 ;
    self.hasHappened_1 = hasHappened_1 ;
    self.hasHappened_0 = hasHappened_0 ;
    
    // |===| Return Nothing Result.
    Ok( () )
  }

  fn output(&self) -> Self::Output {(
    self.svar_light_index,
    self.svar_time,
  )}
}

/// Stores the state for sub-node `hasHappened`.
///
/// # Inputs
///
/// | Lustre identifier | Type |
/// |:---:|:---|
/// | `X` | Bool |
///
/// # Outputs
///
/// | Lustre identifier | Type |
/// |:---:|:---|
/// | `Y` | Bool |
///
/// # Sub systems
///
/// No subsystems for this system.
///
/// # Assertions
///
/// /// No assertions for this system.
///
/// # Assumptions
///
/// No assumptions for this system.
///
#[derive(Copy, Clone)] 
pub struct HasHappened {
  /// Input: `hasHappened.usr.X`
  pub svar_X: Bool,

  /// Output: `hasHappened.usr.Y`
  pub svar_Y: Bool,


}

impl System for HasHappened {
  type Input = (
    Bool, // svar_X (hasHappened.usr.X)
  ) ;
  type Output = (
    Bool, // svar_Y (hasHappened.usr.Y)
  ) ;
  fn arity() -> usize { 1 }
  fn init(input: Self::Input) -> Result<Self, ()> {
    // |===| Retrieving inputs.
    let svar_X = input.0 ;
    
    // |===| Computing initial state.
    let svar_Y = svar_X ;
    
    // |===| Checking assertions.
    
    
    
    
    // |===| Returning initial state.
    Ok( HasHappened {
      // |===| Inputs.
      svar_X: svar_X,
      
      // |===| Outputs.
      svar_Y: svar_Y,
      
      // |===| Locals.
      
      
      // |===| Calls.
      
    } )
  }

  fn next(&mut self, input: Self::Input) -> Result<(), ()> {
    // |===| Retrieving inputs.
    let svar_X = input.0 ;
    
    // |===| Computing next state.
    let svar_Y = (svar_X | self.svar_Y) ;
    
    // |===| Checking assertions.
    
    
    // |===| Checking assumptions.
    
    
    // |===| Updating next state.
    // |===| Inputs.
    self.svar_X = svar_X ;
    
    // |===| Outputs.
    self.svar_Y = svar_Y ;
    
    // |===| Locals.
    
    
    // |===| Calls.
    
    
    // |===| Return Nothing Result.
    Ok( () )
  }

  fn output(&self) -> Self::Output {(
    self.svar_Y,
  )}
}

/// Stores the state for sub-node `sofar`.
///
/// # Inputs
///
/// | Lustre identifier | Type |
/// |:---:|:---|
/// | `X` | Bool |
///
/// # Outputs
///
/// | Lustre identifier | Type |
/// |:---:|:---|
/// | `Y` | Bool |
///
/// # Sub systems
///
/// No subsystems for this system.
///
/// # Assertions
///
/// /// No assertions for this system.
///
/// # Assumptions
///
/// No assumptions for this system.
///
#[derive(Copy, Clone)] 
pub struct Sofar {
  /// Input: `sofar.usr.X`
  pub svar_X: Bool,

  /// Output: `sofar.usr.Y`
  pub svar_Y: Bool,


}

impl System for Sofar {
  type Input = (
    Bool, // svar_X (sofar.usr.X)
  ) ;
  type Output = (
    Bool, // svar_Y (sofar.usr.Y)
  ) ;
  fn arity() -> usize { 1 }
  fn init(input: Self::Input) -> Result<Self, ()> {
    // |===| Retrieving inputs.
    let svar_X = input.0 ;
    
    // |===| Computing initial state.
    let svar_Y = svar_X ;
    
    // |===| Checking assertions.
    
    
    
    
    // |===| Returning initial state.
    Ok( Sofar {
      // |===| Inputs.
      svar_X: svar_X,
      
      // |===| Outputs.
      svar_Y: svar_Y,
      
      // |===| Locals.
      
      
      // |===| Calls.
      
    } )
  }

  fn next(&mut self, input: Self::Input) -> Result<(), ()> {
    // |===| Retrieving inputs.
    let svar_X = input.0 ;
    
    // |===| Computing next state.
    let svar_Y = (svar_X & self.svar_Y) ;
    
    // |===| Checking assertions.
    
    
    // |===| Checking assumptions.
    
    
    // |===| Updating next state.
    // |===| Inputs.
    self.svar_X = svar_X ;
    
    // |===| Outputs.
    self.svar_Y = svar_Y ;
    
    // |===| Locals.
    
    
    // |===| Calls.
    
    
    // |===| Return Nothing Result.
    Ok( () )
  }

  fn output(&self) -> Self::Output {(
    self.svar_Y,
  )}
}



/// Lustre Language Traits
pub mod Lustre {
  /// Lustre Types
  pub type Int = i32;
  pub type Real = f32;
  pub type Bool = bool;

  /// Lustre System (Component)
  pub trait System: Sized {
    // component types
    type Input;
    type Output;

    /// get size of inputs
    fn arity() -> usize;

    /// run once to get the intial state
    fn init(inp: Self::Input) -> Result<Self, ()>;

    /// update function that will run in a loop
    fn next(&mut self, inp: Self::Input) -> Result<(), ()>;

    /// get output at this time
    fn output(&self) -> Self::Output;
  }
  
}


