pub mod error;
pub mod instruction;

pub mod prelude {
    pub use crate::error::*;
    pub use crate::instruction::*;
}

use steel::*;

declare_id!("z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35");
