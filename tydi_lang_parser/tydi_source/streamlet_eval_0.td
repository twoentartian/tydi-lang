package streamlet_eval_0;

type bit1_0 = Bit(1);
type bit1_1 = bit1_0;
type bit1_2 = bit1_1;

type Group g0 {
  a: bit1_2,
  b: bit1_2,
};

type bit8_stream = Stream(g0, d = 5);

streamlet streamlet0 {
  in : bit8_stream [2] in,
  out : bit8_stream [2] out,
};

streamlet streamlet1<i:int, t:type> {
  const const_value = 16,
  const port_length = 10,
  in : Stream(Bit(const_value)) in, //anonymous type
  out : bit8_stream [port_length] out,
};

streamlet streamlet2<i:int, t:type> {
  in : t in, //anonymous type
  out : bit8_stream [port_length] out,
};

impl impl0 of streamlet1<1, type Bit(1)> {

};