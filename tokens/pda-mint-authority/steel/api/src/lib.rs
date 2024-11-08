pub mod instruction;
pub mod macros;
pub mod state;

pub mod prelude {
    pub use crate::instruction::*;
    pub use crate::state::*;
}

use steel::*;

// TODO Set program id
declare_id!("z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35");
