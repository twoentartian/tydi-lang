package main;

type bit8_stream = Stream(Bit(8), d = 5, t = 2.5);

const eight = 8;

type Group rgb {
  r: Bit(eight),
  g: Bit(eight),
  b: Bit(eight),
};

type rgb_stream = Stream(rgb);

//we define a streamlet called "component"
streamlet component {
  input: rgb_stream in,
  output: rgb_stream out,
};

//define three implement of "component"
impl component_impl0 of component {
  input => output,
};

impl component_impl1 of component {
  input => output,
};

impl component_impl2 of component {
  input => output,
};

//an example of using abstract implement
streamlet larger_component {
  input: rgb_stream [2] in,
  output: rgb_stream [2] out,
};

impl impl_larger_component<ts: impl of component> of larger_component {
  instance inst(ts) [2],
  for i in (0=1=>2) {
    input[i] => inst[i].input,
    inst[i].output => output[i],
  }
};

impl impl_larger_component0(impl_larger_component<impl component_impl0>);
impl impl_larger_component1(impl_larger_component<impl component_impl1>);






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