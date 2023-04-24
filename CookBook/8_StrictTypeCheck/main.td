package main;

const int_value: int = 10;
const bool_value: bool = true;
const float_value: float = 1.0;
const str_value: str = "helloworld";
const clockdomain_value: clockdomain = "50MHz-1";

const clockdomain_value2: clockdomain;////default clockdomain

//type indicator is optional
const int_value2 = 10;

//define type
const color_depth = 8;
type Group rgb {
  r: Bit(color_depth),
  g: Bit(color_depth),
  b: Bit(color_depth),
};

type Group rgb2 {
  r: Bit(color_depth),
  g: Bit(color_depth),
  b: Bit(color_depth),
};

type stream_rgb = Stream(rgb, u=rgb, c=5, t=float_value, r="forward", x=true);

#streamlet documentation#
streamlet rgb_bypass {
  input: stream_rgb in `clockdomain_value,
  output: stream_rgb out `clockdomain_value,
  input2: Stream(rgb) in `clockdomain_value2,
  output2: Stream(rgb2) out `clockdomain_value2,
  input3: Stream(rgb, u=rgb, c=5, t=float_value, r="forward", x=true) in,
  output3: Stream(rgb, u=rgb, c=5, t=float_value, r="forward", x=true) out,
};

#implement documentation#
impl impl_rgb_bypass of rgb_bypass {
  input => output "con1",
  input2 => output2 "con2" @NoStrictType@,
  input3 => output3 "con3" @NoStrictType@,
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