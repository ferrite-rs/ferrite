pub trait ExtractChoice<Row> {
  fn extract(row: Row) -> Self;
}

pub fn extract<Choice, Row>(row: Row) -> Choice
where
  Choice: ExtractChoice<Row>,
{
  Choice::extract(row)
}
