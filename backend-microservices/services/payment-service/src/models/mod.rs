pub mod payment;

pub use payment::*;

// Add PaymentIntent as an alias for now
pub type PaymentIntent = payment::Payment; 