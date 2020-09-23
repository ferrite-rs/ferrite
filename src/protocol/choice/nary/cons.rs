use crate::base::*;
use async_std::sync::{ Sender, Receiver };

pub struct ReceiverApp {}
pub struct SenderApp {}
pub struct SessionApp < C > {}

impl TyCon for ReceiverApp {}
impl TyCon for SenderApp {}

impl < C > TyCon
  for SessionApp < C >
where
  C: Context
{ }

impl < P > TypeApp < P > for ReceiverApp
{ type Applied = Receiver < P >; }

impl < P > TypeApp < P > for SenderApp
{ type Applied = Sender < P >; }

impl < C, A > TypeApp < A >
  for SessionApp < C >
{ type Applied = PartialSession < C, A >; }
