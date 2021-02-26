pub trait Nat: super::Nat {}

impl < N > Nat for N
where N: super::Nat
{ }
