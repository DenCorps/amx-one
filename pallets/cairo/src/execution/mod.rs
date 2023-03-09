//! Module for executing Cairo programs.
/// Actual Cairo executor.
pub mod cairo_vm_executor;
/// Mock Cairo executor.
pub mod mock;

use crate::types::{CairoAssemblyProgram, CairoAssemblyProgramInput, CairoAssemblyProgramOutput};
use crate::{Config, Error};

/// Trait for executing Cairo programs.
pub trait CairoExecutor<T: Config> {
    /// Executes a Cairo program.
    /// # Arguments
    /// * `cairo_program` - The Cairo program to execute.
    /// * `input` - The input to pass to the Cairo program.
    /// # Returns
    /// The result of executing the Cairo program.
    fn execute(
        &self,
        cairo_program: &CairoAssemblyProgram<T>,
        input: &CairoAssemblyProgramInput,
    ) -> Result<CairoAssemblyProgramOutput, Error<T>>;
}
