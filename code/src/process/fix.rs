use std::marker::PhantomData;
use crate::base::*;


pub struct FixProtocol2 < F > {
  f : PhantomData < F >
}

impl < F >
  Protocol for
  FixProtocol2 < F >
where
  F : Send + 'static,
  F : TyApp < FixProtocol2 < F > >,
  < F as
    TyApp < FixProtocol2 < F > >
  > :: Applied : Protocol
{
  type Payload =
    < < F as
        TyApp < FixProtocol2 < F > >
      > :: Applied
      as Protocol
    > :: Payload;
}

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
    TyApp < Recur <
      Fix < F :: Payload >
    > >,
  < F :: Payload as
    TyApp < Recur <
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
    TyApp < Recur <
      FixProtocol < F >
    > >,
  < F as
    TyApp <
      S < A >
    >
  > :: Applied :
    TyApp < Recur <
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
