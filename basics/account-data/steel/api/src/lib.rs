mod instruction;
mod state;

use steel::*;

declare_id!("z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35");

pub mod prelude {
    pub use crate::state::*;
}