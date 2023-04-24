package main;

type bit8_stream = Stream(Bit(8), d = 5, t = 2.5);

const eight = 8;

type Group rgb {
  r: Bit(eight),
  g: Bit(eight),
  b: Bit(eight),
};

type rgb_stream = Stream(rgb);


streamlet data_bypass<data_type: type> {
  input: data_type in,
  output: data_type out,
};

impl impl_data_bypass<data_type: type> of data_bypass<type data_type> {
  input => output,
};

streamlet data_demux<channel:int, data_type: type, cd:clockdomain> {
  inputs: data_type [channel] in `cd,
  outputs: data_type [channel] out `cd,
};

impl impl_data_demux<channel:int, data_type: type, cd:clockdomain> of data_demux<channel, type data_type, cd> {
  instance bypass(impl_data_bypass<type data_type>) [channel],
  for i in (0=1=>channel) {
    bypass[i].output => outputs[i],
    inputs[i] => bypass[i].input,
  }
};

const cd0: clockdomain = "100MHz";
const cd1: clockdomain;
impl data_demux_rgb_5(impl_data_demux<5, type rgb_stream, cd0>);
impl data_demux_rgb_100(impl_data_demux<20, type rgb_stream, cd1>);






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