pub mod error;
pub mod instruction;

use instruction::*;
use steel::*;

pub mod prelude {
    pub use crate::error::*;
    pub use crate::instruction::*;
}

declare_id!("z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35");
