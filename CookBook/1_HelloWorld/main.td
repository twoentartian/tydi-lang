package main; //package name must be the same with the file name


const int_value: int = 10;
const bool_value: bool = true;
const float_value: float = 1.0;
const str_value: str = "helloworld";
const clockdomain_value: clockdomain = 50MHz-1;

const clockdomain_value2: clockdomain;////TODO

//type indicator is optional
const int_value2 = 10;

//define type
const color_depth = 8;
type Group rgb {
  r: Bit(color_depth),
  g: Bit(color_depth),
  b: Bit(color_depth),
};

#streamlet documentation#
streamlet rgb_bypass {
  input: Stream(rgb, u=rgb, c=5, t=float_value, r="forward", x=true) in `100kHz,
  output: Stream(rgb, u=rgb, c=5, t=float_value, r="forward", x=true) out `100kHz,
  input2: Stream(rgb) in `10MHz,
  output2: Stream(rgb) out `10MHz,
};

#implement documentation#
impl impl_rgb_bypass of rgb_bypass {
  input => output,
  input2 => output2,
};