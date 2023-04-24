package main;

streamlet s {
  port_in: Stream(Bit(8)) in,
  port_out: Stream(Bit(8)) out,
};

external impl i of s {
  process {

  },
};//all contents in external implement will be ignored except for simulation processing.

impl i2 of s {
  instance inner(i),

  inner.port_out => port_out,
  port_in => inner.port_in,
};







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