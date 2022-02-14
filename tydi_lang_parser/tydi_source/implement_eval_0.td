package implement_eval_0;

import implement_eval_1;

const var = 10;

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

};

impl impl1 of streamlet1<var, type Bit(1)> {
  const v = 0,
  instance test_inst(impl0),
  test_inst.out[0] => test_inst.in[0],

};