package test;

import external_package;

const flag:bool = true;
const num_instance:int = 8;
const num_stream:int = 2;

const external_var = pack.a;
const f = 1.0;
const g = 1;
const v = f./g;
const v = g./f;

type f = Bit(1);

type location = Union location_ {
  const i = 1,
  x : external.location,
  y : Bit(32),
};

type stream0 = Stream(Bit(4));
type stream1 = Stream(Bit(8));

streamlet sl0<i:int, t:type> {
  const i0 = 1,
  in : stream0 in,
  out : stream0 out,

  in_ : stream1 in,

  in_array : t [num_stream] in,
  out_array : t [num_stream] out,
};

streamlet sl1 {
  in : stream0 in,
  out : stream0 out,

  in_array : stream0 [num_stream] in,
  out_array : stream0 [num_stream] out,
};

streamlet sl2 {
  in : external. stream0 in,
  out : stream0 out,

  in_array : stream0 [num_stream] in,
  out_array : stream0 [num_stream] out,
};

impl temp_impl of external.sl0<1, type Bit(1)> {

};
impl tmux<n: int, ts: impl of sl0<num_instance, type Bit(1)>> of sl0<n, type stream0> {
  const i = 10,
  instance test_inst(ts),
  instance external_inst(external.streamlet),

  test_inst.out => test_inst.in,

  process{},
};
impl test of sl0<1, type Bit(1)> {
  instance inst0(tmux<1, impl test>),
};

//////////////////////////////REGION2

impl temp_impl2 of sl1 {

};
impl tmux2<n: int, ts: impl of sl1> of sl0<n, type stream0> {
  instance test_inst(ts),

  test_inst.out =1=> test_inst.in,

  process{},
};
impl test2 of sl1 {
  instance inst0(tmux2<1,impl temp_impl2>),
};


//////////////////////////////REGION3
streamlet sl4<i:int> {
  in : stream0 in,
  out : stream0 out,
};


impl temp_impl3 of sl4<1> {

};
impl tmux3<n: int, ts: external. impl of sl4<1> > of sl0<n, type stream0> {
  instance test_inst(ts),
  test_inst.out => test_inst.in,
  process{},
};
impl test3 of sl1 {
  instance inst0(tmux3<1,  e. impl temp_impl3>),
};