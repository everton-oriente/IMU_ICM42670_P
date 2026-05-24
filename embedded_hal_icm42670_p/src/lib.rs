#![cfg_attr(not(feature = "use_std"), no_std)]

pub mod icm42670_p_driver;

mod reg;

#[cfg(test)]
mod tests {
    use super::*;
}