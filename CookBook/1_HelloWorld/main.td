package main; //package name must be the same with the file name

const int_value: int = 10;
const bool_value: bool = true;
const float_value: float = 1.0;
const str_value: str = "helloworld";
const clockdomain_value: clockdomain = 50MHz;

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
  input: Stream(rgb, u=rgb, c=5, t=float_value, r="forward", x=true) in,
  output: Stream(rgb, u=rgb, c=5, t=float_value, r="forward", x=true) out,
  input2: Stream(rgb) in,
  output2: Stream(rgb) out,
};

#implement documentation#
impl impl_rgb_bypass of rgb_bypass {
  input => output,
  input2 => output2,
};