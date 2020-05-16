
pub trait Refl {
  type Equals;
}

impl < A >
  Refl
  for A
{
  type Equals = Self;
}
