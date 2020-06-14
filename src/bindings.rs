//! Bindings Module
//!
//! Definition of BunnyMark's custom input bindings
use amethyst::input::BindingTypes;

use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

/// No Axis Bindings required
#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum AxisBinding {}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionBinding {
    AddBunnies,
}

impl Display for AxisBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for ActionBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct BunnyMarkBindings;
impl BindingTypes for BunnyMarkBindings {
    type Axis = AxisBinding;
    type Action = ActionBinding;
}
