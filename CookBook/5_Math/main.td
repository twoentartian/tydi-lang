package main;

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

const array = {0,1,2,3,4}; //array expression
assert(array[0] == 0);
assert(array[4] == 4);
const array1 = (0=1=>5); //(start - step -> end) expression
assert(array1[0] == 0);
assert(array1[4] == 4);

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