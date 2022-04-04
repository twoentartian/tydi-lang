package std;

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




type stream_bit8 = Stream(Bit(8));
streamlet test_s {
  input: stream_bit8 in,
  output: stream_bit8 out,
  output2: stream_bit8 out,
};

streamlet inner_s {
  output: stream_bit8 out,
  false_output: stream_bit8 out,
};

external impl inner_i of inner_s {

};

impl test_i of test_s {
  instance inst(inner_i),
  inst.output => output,
  inst.output => output2,
};