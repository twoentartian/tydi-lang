package main;

const x = 4;

type Group rgb {
  const x = 8, //shadowing

  assert(x == 8),

  r: Bit(x),
  g: Bit(x),
  b: Bit(x),
};

assert(type rgb.x == 8);
assert(x == 4);

streamlet component<data:type> {
  const x = type data.x, //deliver the x to the outer scope

  assert(x == 8),

  input: Stream(data) in,
  output: Stream(data) out,
};

assert(streamlet component<type rgb>.x == 8);

impl component_impl<data:type> of component<type data> {
  const x = streamlet component<type data>.x, //deliver the x to the outer scope
  assert(x == 8),

  if (true) {
    assert(x == 8),
  }
  else {
    assert(x == 7),
  }

  for i in (1=1=>2) {
    assert(x == 8),
  }

  input => output,
};

assert(impl component_impl<type rgb>.x == 8);

impl component_impl0(component_impl<type rgb>);







//void component, always acknowledge the handshake
streamlet void_s<type_in: type> {
  input: type_in in,
};

external impl void_i<type_in: type> of void_s<type type_in> {

};

//data duplicator
streamlet duplicator_s<data_type: type, output_channel: int> {
  input: data_type in,
  output: data_type [output_channel] out,
};

external impl duplicator_i<data_type: type, output_channel: int> of duplicator_s<type data_type, output_channel> {

};