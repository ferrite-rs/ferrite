use std::marker::PhantomData;
use crate::base::*;

pub struct FixProtocol < F > {
  f : PhantomData < F >
}

impl < F >
  Protocol for
  FixProtocol < F >
where
  F : Protocol,
  F :: Payload :
    TyApp < Recur <
      Fix < F :: Payload >
    > >,
  < F :: Payload as
    TyApp < Recur <
      Fix < F :: Payload >
    > >
  > :: Type :
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
  > :: Type :
    TyApp < Recur <
      FixProtocol <
        < F as
          TyApp <
            S < A >
          >
        > :: Type
      >
    > >,
{
  type Type =
    FixProtocol <
      < F as
        TyApp <
          S < A >
        >
      > :: Type
    >;
}
