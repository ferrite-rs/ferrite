extern crate log;
use crate::public::*;

enum CanvasOps {
  FillText ( String, f32 ),
  MoveTo ( f32, f32 ),
  LineTo ( f32, f32 ),
}

type Canvas =
  FixProtocol <
    ExternalChoice <
      SendValue <
        CanvasOps,
        Z
      >,
      End
    >
  >;

type CanvasManager =
  LinearToShared <
    SendChannel <
      Canvas,
      Z
    >
  >;
