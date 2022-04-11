package tpch;

type Group Date {
  year: Bit(32),
  month: Bit(4),
  day: Bit(5),
};

streamlet void_s<type_in: type> {
  input: type_in in,
};

external impl void_i<type_in: type> of void_s<type type_in> {

};

streamlet duplicator_s<data_type: type, output_channel: int> {
  input: data_type in,
  output: data_type [output_channel] out,
};

external impl duplicator_i<data_type: type, output_channel: int> of duplicator_s<type data_type, output_channel> {

};
