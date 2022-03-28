package tpch;

streamlet test {
  type bit1 = Stream(Bit(1)),
  port: bit1 in,
};

streamlet test2<t:type> {
  type bit2 = t,
  port: bit2 in,
};

impl test_impl of test {
  type bit3 = streamlet test.bit1,
};

type t1 = Stream(Bit(1));
impl test2_impl of test2<type t1> {
  type bit4 = streamlet test2<type t1>.bit2,
};

streamlet streamlet1 {
  port: impl test2_impl.bit4 in,
};