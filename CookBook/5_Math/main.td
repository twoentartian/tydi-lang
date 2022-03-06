package main; //package name must be the same with the file name

const two = 1 << 1;
const four = two << 1;
const eight = two * four;
const seven = eight - 1;
const three_point_five = (seven+0.0) / 2; // float
const test_four = round(three_point_five);
const test_four2 = ceil(three_point_five);
const test_three = floor(three_point_five);

const test_log_2_f = log2(2^3);
const test_log_2 = round(test_log_2_f);

const error_msgs = {"msg0","msg1"};
assert(round(test_log_2_f) == 3, error_msgs[0]);
assert(round(test_four) == 4);
assert(round(test_four2) == 4);
assert(round(test_three) == 3, error_msgs[0]);

type Group rgb {
  r: Bit(eight),
  g: Bit(test_four * test_four2 * test_three * test_log_2),
  b: Bit(eight),
};

type rgb_stream = Stream(rgb);

streamlet component {
  input: rgb_stream in,
  output: rgb_stream out,
};

impl component_impl0 of component {
  input => output,
};

impl component_impl1 of component {
  input => output,
};