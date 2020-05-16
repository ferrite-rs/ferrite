extern crate log;
use crate::public::*;

pub enum CanvasOps {
  FillText ( String, f32 ),
  MoveTo ( f32, f32 ),
  LineTo ( f32, f32 ),
}

pub type Canvas =
  Fix <
    ExternalChoice <
      SendValue <
        CanvasOps,
        Z
      >,
      End
    >
  >;

pub type CanvasManager =
  LinearToShared <
    SendChannel <
      Canvas,
      Z
    >
  >;
