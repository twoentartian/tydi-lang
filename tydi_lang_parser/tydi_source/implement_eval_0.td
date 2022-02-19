package implement_eval_0;

import implement_eval_1;

const v = 1;

type Group temp_group {
  const v = 10,
  const v1 = 20,
};

const var = type temp_group.v;

type bit8_stream = Stream(Bit(8), d = 5);

streamlet basic0 {
  in: Stream(Bit(1)) in,
  out: Stream(Bit(1)) out,
};

streamlet streamlet1<i:int, t:type> {
  const v = i,
  in : Stream(t) in, //anonymous type
  out : bit8_stream [i] out,
};

impl impl0 of implement_eval_1.basic1 {
  const v = 5,
};

impl impl1 of streamlet1<var, type Bit(1)> {
  const v = impl impl0.v,
  //const v = 5,
  const v2 = v * 2,
  instance test_inst(impl0) [v],
  test_inst[0].out[0] =v2=> test_inst[v-1].in,

};

impl impl2<i:int, ts: impl of implement_eval_1.basic1> of implement_eval_1.basic1 {
  instance test_inst(impl0),
};

impl impl3 of streamlet1<var, type Bit(1)> {
  instance test_inst(impl2<10, impl impl0>),

};