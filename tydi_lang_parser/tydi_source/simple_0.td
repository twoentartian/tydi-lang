package simple_0;

import simple_1;

const i1: int = 1 + 100;
const i2 = 0xff + 1;
const i3 = 0b111 + 1;

const f1 = 1.2 + 1.6;
const f2 = 1 + 1.6;
const f3 = 1.2 + 1;

const f4 = i1 + 1.0;
const f5 = f4 - f3;
const f5_correct = f5 == 99.8;

const str0 = "" + 1;
const flag = true;
const str1 = str0 + ":" + flag;
const str2 = "(" + str1 + ")";

const mod0 = 1 + 10 % 3;

const power0 = 1 + 2 * 3 ^ 4;
const power0_correct = power0 == 163;

const external_var0 = simple_1.i1 + 10;
const external_flag0 = false || simple_1.flag;

const divide0 = 2.0 / 1.0;
const divide1 = 2.0 / 1;
//const divide2 = 2.0 / 0;

const compound0 = 1 + 2.0 * (5.0 - 2);

const greater0 = 5 > 4;

const bitaddor0 = 1 & 2 + 1 | 2;
const bitaddor0_correct = bitaddor0 == 3;

const logical_left_0 = 1 << 2;
const logical_left_0_correct = logical_left_0 == 4;

const logical_right_0 = 16 >> 2;
const logical_eight_0_correct = logical_right_0 == 4;

//const logical_error_0 = (0-2) << 2;

const array_range_0 = (0=1=>10);
const array_range_1 = (0=2=>10);
const array_range_2 = (0=3=>10);

const array_exp_0 = {1,2,3,4,5};
const array_exp_1 = {true,true,false};
const array_exp_2 = {"123", "456"};
const array_exp_3 = {1.1,2.1,3.1};
const array_exp_4 = array_exp_3 + 50.5;
const array_exp_5 = 50.5 + array_exp_3;
const array_exp_6 = true + {true,false};

const bitwise_not_0 = ~(5)15; // 0b01111 bitwise not => 0b10000//16
const bitwise_not_0_correct = 0 == bitwise_not_0;

const round_0 = round(3.4);
const ceil_0 = ceil(3.4);
const floor_0 = floor(3.4);

const array_int_0 = {1,2,3,4};
const array_float_0 = {1.0,2.0,3.0,4.0};
const array_str_0 = {"1","2","3","4"};
const array_bool_0 = {true,true,false,false};
const array_int_0_correct = array_int_0[3] == 4;
const array_float_0_correct = array_float_0[3] == 4.0;
const array_str_0_correct = array_str_0[3] == "4";
const array_bool_0_correct = array_bool_0[3] == false;

const unary_0 = -divide1;
const unary_1 = !array_int_0_correct;