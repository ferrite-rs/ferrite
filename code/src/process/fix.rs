use std::marker::PhantomData;
use crate::base::*;

pub struct FixProtocol < F > {
  f : PhantomData < F >
}

impl < F, G >
  Protocol for
  FixProtocol < G >
where
  G : Send + 'static,
  G : TyApp < Z, Applied=F >,
  F : Protocol,
  F :: Payload :
    TyApp < Unfix <
      Fix < F :: Payload >
    > >,
  < F :: Payload as
    TyApp < Unfix <
      Fix < F :: Payload >
    > >
  > :: Applied :
    Send
{
  type Payload = Fix < F :: Payload >;
}

impl < A, F >
  TyApp <
    A
  > for
  FixProtocol < F >
where
  F :
    TyApp <
      S < A >
    >,
  F :
    TyApp < Unfix <
      FixProtocol < F >
    > >,
  < F as
    TyApp <
      S < A >
    >
  > :: Applied :
    TyApp < Unfix <
      FixProtocol <
        < F as
          TyApp <
            S < A >
          >
        > :: Applied
      >
    > >,
{
  type Applied =
    FixProtocol <
      < F as
        TyApp <
          S < A >
        >
      > :: Applied
    >;
}
