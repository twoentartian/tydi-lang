package main; //package name (rather than the file name) determines the name in memory.

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

#streamlet documentation#
streamlet rgb_bypass {
  input: Stream(rgb, u=rgb, c=5, t=float_value, r="forward", x=true) in `clockdomain_value,
  output: Stream(rgb, u=rgb, c=5, t=float_value, r="forward", x=true) out `clockdomain_value,
  input2: Stream(rgb) in `clockdomain_value2,
  output2: Stream(rgb) out `clockdomain_value2,
};

#implement documentation#
impl impl_rgb_bypass of rgb_bypass {
  input => output @NoStrictType@,
  input2 => output2,
};