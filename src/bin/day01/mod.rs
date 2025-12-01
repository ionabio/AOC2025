pub mod constants;
pub mod parsing;
pub mod simulation;
pub mod systems;
pub mod types;
pub mod ui;
pub mod visuals;

// Re-export commonly used items
pub use constants::*;
pub use systems::{load_default_input, run_simulation};
pub use types::*;
pub use ui::ui_system;
pub use visuals::{setup_visuals, update_pointer_visual};

