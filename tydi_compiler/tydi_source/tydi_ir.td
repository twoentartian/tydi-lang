package tpch;

streamlet void_s<type_in: type> {
  stream_in: type_in in,
};

external impl void_i<type_in: type> of void_s<type type_in> {

};

//type bit8_stream = Stream(Bit(8));
//impl void_i_bit8(void_i<type bit8_stream>);

impl void_i_bit8(void_i<type Stream(Bit(8))>);