package tpch;

type SQL_int = Bit(32);
type int_stream = Stream(SQL_int, d = 1);
type key_stream = int_stream;

const year_max = 10^5 - 1;
type year_t = Bit(ceil(log2(year_max)));
type year_stream = Stream(year_t);
const month_max = 12;
type month_t = Bit(ceil(log2(month_max)));
type month_stream = Stream(month_t);
const day_max = 31;
type day_t = Bit(ceil(log2(day_max)));
type day_stream = Stream(day_t);
type Group Date {
  year: year_t,
  month: month_t,
  day: day_t,
};
type date_stream = Stream(Date, d = 1);

type SQL_char = Bit(8);
type SQL_char1_stream = Stream(SQL_char, d = 1);
type varchar_stream = Stream(SQL_char, d = 2);

type SQL_char10 = Bit(8*10);
type SQL_char10_stream = Stream(SQL_char10, d = 1);
type SQL_char15 = Bit(8*15);
type SQL_char15_stream = Stream(SQL_char15, d = 1);
type SQL_char25 = Bit(8*25);
type SQL_char25_stream = Stream(SQL_char25, d = 1);

const max_decimal_15 = 10^15 - 1;
const bit_width_decimal_15 = ceil(log2(max_decimal_15));
type SQL_decimal_15 = Bit(bit_width_decimal_15);
type Group SQL_decimal_15_2 {
  const frac = 2,
  decimal: SQL_decimal_15,
};
type SQL_decimal_15_2_stream = Stream(SQL_decimal_15_2, d = 1);

////////////////////  tpch.part  ////////////////////
streamlet part_s {
  p_partkey: key_stream in,

  p_name: varchar_stream out,
  p_mfgr: SQL_char25_stream out,
  p_brand: SQL_char10_stream out,
  p_type: varchar_stream out,
  p_size: int_stream out,
  p_container: SQL_char10_stream out,
  p_retailprice: SQL_decimal_15_2_stream out,
  p_comment: varchar_stream out,
};

external impl part_i of part_s {

};

////////////////////  tpch.nation  ////////////////////
streamlet nation_s {
  n_nationkey: key_stream in,

  n_name: SQL_char25_stream out,
  n_regionkey: key_stream out,
  n_comment: varchar_stream out,
};

external impl nation_i of nation_s {

};

////////////////////  tpch.region  ////////////////////
streamlet region_s {
  r_regionkey: key_stream in,
  r_name: SQL_char25_stream out,
  r_comment: varchar_stream out,
};

external impl region_i of region_s {

};

////////////////////  tpch.supplier  ////////////////////
streamlet supplier_s {
  s_suppkey: key_stream in,
  s_name: SQL_char25_stream out,
  s_address: varchar_stream out,
  s_nationkey: key_stream out,
  s_phone: SQL_char15_stream out,
  s_acctbal: SQL_decimal_15_2_stream out,
  s_comment: varchar_stream out,
};

external impl supplier_i of supplier_s {

};

////////////////////  tpch.partsupp  ////////////////////
streamlet partsupp_s {
  ps_partkey: key_stream in,
  ps_suppkey: key_stream in,
  ps_availqty: int_stream out,
  ps_supplycost: SQL_decimal_15_2_stream out,
  ps_comment: varchar_stream out,
};

external impl partsupp_i of partsupp_s {

};

////////////////////  tpch.customer  ////////////////////
streamlet customer_s {
  c_custkey: key_stream in,
  c_name: varchar_stream out,
  c_address: varchar_stream out,
  c_nationkey: key_stream out,
  c_phone: SQL_char15_stream out,
  c_acctbal: SQL_decimal_15_2_stream out,
  c_mktsegment: SQL_char10_stream out,
  c_comment: varchar_stream out,
};

external impl customer_i of customer_s {

};

////////////////////  tpch.orders  ////////////////////
streamlet orders_s {
  o_orderkey: key_stream in,
  o_custkey: key_stream out,
  o_orderstatus: SQL_char1_stream out,
  o_totalprice: SQL_decimal_15_2_stream out,
  o_orderdate: date_stream out,
  o_orderpriority: SQL_char15_stream out,
  o_clerk: SQL_char15_stream out,
  o_shippriority: int_stream out,
  o_comment: varchar_stream out,
};

external impl orders_i of orders_s {

};

////////////////////  tpch.lineitem  ////////////////////
streamlet lineitem_s {
  l_orderkey: key_stream in,
  l_linenumber: key_stream in,
  l_partkey: key_stream out,
  l_suppkey: key_stream out,
  l_quantity: SQL_decimal_15_2_stream out,
  l_extendedprice: SQL_decimal_15_2_stream out,
  l_discount: SQL_decimal_15_2_stream out,
  l_tax: SQL_decimal_15_2_stream out,
  l_returnflag: SQL_char1_stream out,
  l_linestatus: SQL_char1_stream out,
  l_shipdate: date_stream out,
  l_commitdate: date_stream out,
  l_receiptdate: date_stream out,
  l_shipinstruct: SQL_char25_stream out,
  l_shipmode: SQL_char10_stream out,
  l_comment: varchar_stream out,
};

external impl lineitem_i of lineitem_s {

};



////////////////////  tydi standard lib  ////////////////////

//void component, always acknowledge the handshake
streamlet void_s<type_in: type> {
  input: type_in in,
};

external impl void_i<type_in: type> of void_s<type type_in> {

};

//padding zero to the highest bit
streamlet padding_zero_s<type_in: type, type_out: type> {
  stream_in: type_in in,
  stream_out: type_out out,
};

external impl padding_zero_i<type_in: type, type_out: type> of padding_zero_s<type type_in, type type_out> {

};

//comparator, compare two values: (input0 is larger) => 1, (input1 is larger) => 2, (input1 == input0) => 3
streamlet comparator_s<type_in: type> {
  input0: type_in in,
  input1: type_in in,
  output: Stream(Bit(2)) out,
};

external impl comparator_i<type_in: type> of comparator_s<type type_in> {

};

//const value generator, type_out should be a Stream(Bit(x)) type and the value should be the value mapped to Bit(x)
streamlet const_value_generator_s<type_out: type, value: int> {
  output: type_out out,
};

external impl const_value_generator_i<type_out: type, value: int> of const_value_generator_s<type type_out, value> {

};

//data duplicator
streamlet duplicator_s<data_type: type, output_channel: int> {
  input: data_type in,
  output: data_type [output_channel] out,
};

external impl duplicator_i<data_type: type, output_channel: int> of duplicator_s<type data_type, output_channel> {

};

//stream filter
type stream_filter_select_stream = Stream(Bit(2));
streamlet stream_filter_s<data_type: type> {
  input: data_type in,
  output: data_type out,
  select: stream_filter_select_stream in,
};

external impl stream_filter_i<data_type: type> of stream_filter_s<type data_type> {

};

//stream filter
type stream_filter_select_stream_1bit = Stream(Bit(1));
streamlet stream_filter_1bit_s<data_type: type> {
  input: data_type in,
  output: data_type out,
  select: stream_filter_select_stream_1bit in,
};

external impl stream_filter_1bit_i<data_type: type> of stream_filter_1bit_s<type data_type> {

};

//accumulator
streamlet accumulator_s<data_type: type> {
  type count_type = Stream(Bit(32)),
  input: data_type in,
  output: data_type out,
  count: count_type out,
  type overflow_type = Stream(Bit(1)),
  overflow: overflow_type out,
};

external impl accumulator_i<data_type: type> of accumulator_s<type data_type> {

};

//logical type converter
streamlet converter_s<input_type: type, output_type: type, channel: int> {
  input: input_type [channel] in,
  output: output_type [channel] out,
};

external impl converter_i<input_type: type, output_type: type, channel: int> of converter_s<type input_type, type output_type, channel> {

};

//and
streamlet and_s<data_type: type, input_channel: int> {
  input: data_type [input_channel] in,
  output: data_type out,
};

external impl and_i<data_type: type, input_channel: int> of and_s<type data_type, input_channel> {

};

streamlet or_s<data_type: type, input_channel: int> {
  input: data_type [input_channel] in,
  output: data_type out,
};

external impl or_i<data_type: type, input_channel: int> of or_s<type data_type, input_channel> {

};

//adder
streamlet adder_s<data_type: type> {
  input0: data_type in,
  input1: data_type in,
  output: data_type out,
  overflow: Stream(Bit(1)) out,
};

external impl adder_i<data_type: type> of adder_s<type data_type> {

};

//to negative
streamlet to_neg_s<data_type: type> {
  input: data_type in,
  output: data_type out,
};

external impl to_neg_i<data_type: type> of to_neg_s<type data_type> {

};

//multiplier
streamlet multiplier_s<data_type: type> {
  input0: data_type in,
  input1: data_type in,
  output: data_type out,
  overflow: Stream(Bit(1)) out,
};

external impl multiplier_i<data_type: type> of multiplier_s<type data_type> {

};

//divider
streamlet divider_s<data_type: type> {
  dividend: data_type in,
  divisor: data_type in,
  quotient: data_type out,
};

external impl divider_i<data_type: type> of divider_s<type data_type> {

};

//comparator, is larger
streamlet comparator_is_larger_s<type_in: type> {
  input0: type_in in,
  input1: type_in in,
  output: Stream(Bit(1)) out,
};

external impl comparator_is_larger_i<type_in: type> of comparator_is_larger_s<type type_in> {

};

//comparator, is smaller
streamlet comparator_is_smaller_s<type_in: type> {
  input0: type_in in,
  input1: type_in in,
  output: Stream(Bit(1)) out,
};

external impl comparator_is_smaller_i<type_in: type> of comparator_is_smaller_s<type type_in> {

};

//comparator, is larger or equal
streamlet comparator_is_larger_equal_s<type_in: type> {
  input0: type_in in,
  input1: type_in in,
  output: Stream(Bit(1)) out,
};

external impl comparator_is_larger_equal_i<type_in: type> of comparator_is_larger_equal_s<type type_in> {

};

//comparator, is smaller or equal
streamlet comparator_is_smaller_equal_s<type_in: type> {
  input0: type_in in,
  input1: type_in in,
  output: Stream(Bit(1)) out,
};

external impl comparator_is_smaller_equal_i<type_in: type> of comparator_is_smaller_equal_s<type type_in> {

};

//comparator, is equal
streamlet comparator_is_equal_s<type_in: type> {
  input0: type_in in,
  input1: type_in in,
  output: Stream(Bit(1)) out,
};

external impl comparator_is_equal_i<type_in: type> of comparator_is_equal_s<type type_in> {

};

////////////////////  Project file  ////////////////////
type select_stream = Stream(Bit(1));

//const_char10_generator
streamlet const_char10_generator_s<value: str> {
  output: SQL_char10_stream out,
};

external impl const_char10_generator_i<value: str> of const_char10_generator_s<value> {

};

//const_char25_generator
streamlet const_char25_generator_s<value: str> {
  output: SQL_char25_stream out,
};

external impl const_char25_generator_i<value: str> of const_char25_generator_s<value> {

};

//is_equal_to_const_char10_s
streamlet is_equal_to_const_char10_s<value: str> {
  input: SQL_char10_stream in,
  output: Stream(Bit(1)) out,
};

impl is_equal_to_const_char10_i<value: str> of is_equal_to_const_char10_s<value> {
  instance compare(comparator_is_equal_i<type SQL_char10_stream>),
  instance const_gen(const_char10_generator_i<value>),
  const_gen.output => compare.input0,
  input => compare.input1,
  compare.output => output @NoStrictType@,
};

//is_equal_to_const_char25_s
streamlet is_equal_to_const_char25_s<value: str> {
  input: SQL_char25_stream in,
  output: Stream(Bit(1)) out,
};

impl is_equal_to_const_char25_i<value: str> of is_equal_to_const_char25_s<value> {
  instance compare(comparator_is_equal_i<type SQL_char25_stream>),
  instance const_gen(const_char25_generator_i<value>),
  const_gen.output => compare.input0,
  input => compare.input1,
  compare.output => output @NoStrictType@,
};

streamlet where_claus_s {
  p_partkey: key_stream in,
  l_partkey: key_stream in,
  p_brand: SQL_char10_stream in,
  p_container: SQL_char10_stream in,
  l_quantity: SQL_decimal_15_2_stream in,
  p_size: int_stream in,
  l_shipmode: SQL_char10_stream in,
  l_shipinstruct: SQL_char25_stream in,

  select: select_stream out,
};

impl where_claus_i of where_claus_s {
  instance or_0(or_i<type select_stream, 3>),
  or_0.output => select,

  //First or component

  const bands = {"band0", "band1", "band2"},
  const containers = {"SM CASE", "SM BOX", "SM PACK", "SM PKG",
                    "MED BAG", "MED BOX", "MED PKG", "MED PACK",
                    "LG CASE", "LG BOX", "LG PACK", "LG PKG"},
  const shipmodes = {"AIR","AIR REG"},
  const shipinstruct = "DELIVER IN PERSON",
  const size_min = {1,1,1},
  const size_max = {5,10,15},
  const quantity_min = {4,5,6},
  const quantity_max = {14,15,16},


  //1st or component
  instance and_0(and_i<type select_stream, 9>),
  and_0.output => or_0.input[0],
  instance key_equal_0(comparator_is_equal_i<type key_stream>),
  p_partkey => key_equal_0.input0,
  l_partkey => key_equal_0.input1,
  key_equal_0.output => and_0.input[0] @NoStrictType@,

  instance p_brand_equal_0(is_equal_to_const_char10_i<bands[0]>),
  p_brand => p_brand_equal_0.input,
  p_brand_equal_0.output => and_0.input[1] @NoStrictType@,

  instance container_or_0(or_i<type select_stream, 4>),
  instance p_container_0_0(is_equal_to_const_char10_i<containers[0]>),
  instance p_container_0_1(is_equal_to_const_char10_i<containers[1]>),
  instance p_container_0_2(is_equal_to_const_char10_i<containers[2]>),
  instance p_container_0_3(is_equal_to_const_char10_i<containers[3]>),
  p_container => p_container_0_0.input,
  p_container => p_container_0_1.input,
  p_container => p_container_0_2.input,
  p_container => p_container_0_3.input,
  p_container_0_0.output => container_or_0.input[0] @NoStrictType@,
  p_container_0_1.output => container_or_0.input[1] @NoStrictType@,
  p_container_0_2.output => container_or_0.input[2] @NoStrictType@,
  p_container_0_3.output => container_or_0.input[3] @NoStrictType@,
  container_or_0.output => and_0.input[2] @NoStrictType@,

  instance l_quantity_min_gen_0(const_value_generator_i<type SQL_decimal_15_2_stream, quantity_min[0]>),
  instance l_quantity_min_compare_0(comparator_is_larger_equal_i<type SQL_decimal_15_2_stream>),
  l_quantity => l_quantity_min_compare_0.input0,
  l_quantity_min_gen_0.output => l_quantity_min_compare_0.input1,
  l_quantity_min_compare_0.output => and_0.input[3] @NoStrictType@,

  instance l_quantity_max_gen_0(const_value_generator_i<type SQL_decimal_15_2_stream, quantity_max[0]>),
  instance l_quantity_max_compare_0(comparator_is_smaller_equal_i<type SQL_decimal_15_2_stream>),
  l_quantity => l_quantity_max_compare_0.input0,
  l_quantity_max_gen_0.output => l_quantity_max_compare_0.input1,
  l_quantity_max_compare_0.output => and_0.input[4] @NoStrictType@,

  instance p_size_min_gen_0(const_value_generator_i<type int_stream, size_min[0]>),
  instance p_size_min_compare_0(comparator_is_larger_equal_i<type int_stream>),
  p_size => p_size_min_compare_0.input0,
  p_size_min_gen_0.output => p_size_min_compare_0.input1,
  p_size_min_compare_0.output => and_0.input[5] @NoStrictType@,

  instance p_size_max_gen_0(const_value_generator_i<type int_stream, size_max[0]>),
  instance p_size_max_compare_0(comparator_is_smaller_equal_i<type int_stream>),
  p_size => p_size_max_compare_0.input0,
  p_size_max_gen_0.output => p_size_max_compare_0.input1,
  p_size_max_compare_0.output => and_0.input[6] @NoStrictType@,

  instance l_shipmode_or_0(or_i<type select_stream, 2>),
  instance l_shipmode_comp_0_0(is_equal_to_const_char10_i<shipmodes[0]>),
  instance l_shipmode_comp_0_1(is_equal_to_const_char10_i<shipmodes[1]>),
  l_shipmode => l_shipmode_comp_0_0.input,
  l_shipmode => l_shipmode_comp_0_1.input,
  l_shipmode_comp_0_0.output => l_shipmode_or_0.input[0] @NoStrictType@,
  l_shipmode_comp_0_1.output => l_shipmode_or_0.input[1] @NoStrictType@,
  l_shipmode_or_0.output => and_0.input[7] @NoStrictType@,

  instance l_shipinstruct_0(is_equal_to_const_char25_i<shipinstruct>),
  l_shipinstruct => l_shipinstruct_0.input,
  l_shipinstruct_0.output => and_0.input[8] @NoStrictType@,


  //2nd or component
  instance and_1(and_i<type select_stream, 9>),
  and_1.output => or_0.input[1],

  key_equal_0.output => and_1.input[0] @NoStrictType@,

  instance p_brand_equal_1(is_equal_to_const_char10_i<bands[1]>),
  p_brand => p_brand_equal_1.input,
  p_brand_equal_1.output => and_1.input[1] @NoStrictType@,

  instance container_or_1(or_i<type select_stream, 4>),
  instance p_container_1_0(is_equal_to_const_char10_i<containers[4]>),
  instance p_container_1_1(is_equal_to_const_char10_i<containers[5]>),
  instance p_container_1_2(is_equal_to_const_char10_i<containers[6]>),
  instance p_container_1_3(is_equal_to_const_char10_i<containers[7]>),
  p_container => p_container_1_0.input,
  p_container => p_container_1_1.input,
  p_container => p_container_1_2.input,
  p_container => p_container_1_3.input,
  p_container_1_0.output => container_or_1.input[0] @NoStrictType@,
  p_container_1_1.output => container_or_1.input[1] @NoStrictType@,
  p_container_1_2.output => container_or_1.input[2] @NoStrictType@,
  p_container_1_3.output => container_or_1.input[3] @NoStrictType@,
  container_or_1.output => and_1.input[2] @NoStrictType@,

  instance l_quantity_min_gen_1(const_value_generator_i<type SQL_decimal_15_2_stream, quantity_min[1]>),
  instance l_quantity_min_compare_1(comparator_is_larger_equal_i<type SQL_decimal_15_2_stream>),
  l_quantity => l_quantity_min_compare_1.input0,
  l_quantity_min_gen_1.output => l_quantity_min_compare_1.input1,
  l_quantity_min_compare_1.output => and_1.input[3] @NoStrictType@,

  instance l_quantity_max_gen_1(const_value_generator_i<type SQL_decimal_15_2_stream, quantity_max[1]>),
  instance l_quantity_max_compare_1(comparator_is_smaller_equal_i<type SQL_decimal_15_2_stream>),
  l_quantity => l_quantity_max_compare_1.input0,
  l_quantity_max_gen_1.output => l_quantity_max_compare_1.input1,
  l_quantity_max_compare_1.output => and_1.input[4] @NoStrictType@,

  instance p_size_min_gen_1(const_value_generator_i<type int_stream, size_min[1]>),
  instance p_size_min_compare_1(comparator_is_larger_equal_i<type int_stream>),
  p_size => p_size_min_compare_1.input0,
  p_size_min_gen_1.output => p_size_min_compare_1.input1,
  p_size_min_compare_1.output => and_1.input[5] @NoStrictType@,

  instance p_size_max_gen_1(const_value_generator_i<type int_stream, size_max[1]>),
  instance p_size_max_compare_1(comparator_is_smaller_equal_i<type int_stream>),
  p_size => p_size_max_compare_1.input0,
  p_size_max_gen_1.output => p_size_max_compare_1.input1,
  p_size_max_compare_1.output => and_1.input[6] @NoStrictType@,

  instance l_shipmode_or_1(or_i<type select_stream, 2>),
  instance l_shipmode_comp_1_0(is_equal_to_const_char10_i<shipmodes[0]>),
  instance l_shipmode_comp_1_1(is_equal_to_const_char10_i<shipmodes[1]>),
  l_shipmode => l_shipmode_comp_1_0.input,
  l_shipmode => l_shipmode_comp_1_1.input,
  l_shipmode_comp_1_0.output => l_shipmode_or_1.input[0] @NoStrictType@,
  l_shipmode_comp_1_1.output => l_shipmode_or_1.input[1] @NoStrictType@,
  l_shipmode_or_1.output => and_1.input[7] @NoStrictType@,

  l_shipinstruct_0.output => and_1.input[8] @NoStrictType@,


  //3rd or component
  instance and_2(and_i<type select_stream, 9>),
  and_2.output => or_0.input[2],

  key_equal_0.output => and_2.input[0] @NoStrictType@,

  instance p_brand_equal_2(is_equal_to_const_char10_i<bands[2]>),
  p_brand => p_brand_equal_2.input,
  p_brand_equal_0.output => and_2.input[1] @NoStrictType@,

  instance container_or_2(or_i<type select_stream, 4>),
  instance p_container_2_0(is_equal_to_const_char10_i<containers[8]>),
  instance p_container_2_1(is_equal_to_const_char10_i<containers[9]>),
  instance p_container_2_2(is_equal_to_const_char10_i<containers[10]>),
  instance p_container_2_3(is_equal_to_const_char10_i<containers[11]>),
  p_container => p_container_2_0.input,
  p_container => p_container_2_1.input,
  p_container => p_container_2_2.input,
  p_container => p_container_2_3.input,
  p_container_2_0.output => container_or_2.input[0] @NoStrictType@,
  p_container_2_1.output => container_or_2.input[1] @NoStrictType@,
  p_container_2_2.output => container_or_2.input[2] @NoStrictType@,
  p_container_2_3.output => container_or_2.input[3] @NoStrictType@,
  container_or_2.output => and_2.input[2] @NoStrictType@,

  instance l_quantity_min_gen_2(const_value_generator_i<type SQL_decimal_15_2_stream, quantity_min[2]>),
  instance l_quantity_min_compare_2(comparator_is_larger_equal_i<type SQL_decimal_15_2_stream>),
  l_quantity => l_quantity_min_compare_2.input0,
  l_quantity_min_gen_2.output => l_quantity_min_compare_2.input1,
  l_quantity_min_compare_2.output => and_2.input[3] @NoStrictType@,

  instance l_quantity_max_gen_2(const_value_generator_i<type SQL_decimal_15_2_stream, quantity_max[2]>),
  instance l_quantity_max_compare_2(comparator_is_smaller_equal_i<type SQL_decimal_15_2_stream>),
  l_quantity => l_quantity_max_compare_2.input0,
  l_quantity_max_gen_2.output => l_quantity_max_compare_2.input1,
  l_quantity_max_compare_2.output => and_2.input[4] @NoStrictType@,

  instance p_size_min_gen_2(const_value_generator_i<type int_stream, size_min[2]>),
  instance p_size_min_compare_2(comparator_is_larger_equal_i<type int_stream>),
  p_size => p_size_min_compare_2.input0,
  p_size_min_gen_2.output => p_size_min_compare_2.input1,
  p_size_min_compare_2.output => and_2.input[5] @NoStrictType@,

  instance p_size_max_gen_2(const_value_generator_i<type int_stream, size_max[2]>),
  instance p_size_max_compare_2(comparator_is_smaller_equal_i<type int_stream>),
  p_size => p_size_max_compare_2.input0,
  p_size_max_gen_2.output => p_size_max_compare_2.input1,
  p_size_max_compare_2.output => and_2.input[6] @NoStrictType@,

  instance l_shipmode_or_2(or_i<type select_stream, 2>),
  instance l_shipmode_comp_2_0(is_equal_to_const_char10_i<shipmodes[0]>),
  instance l_shipmode_comp_2_1(is_equal_to_const_char10_i<shipmodes[1]>),
  l_shipmode => l_shipmode_comp_2_0.input,
  l_shipmode => l_shipmode_comp_2_1.input,
  l_shipmode_comp_2_0.output => l_shipmode_or_2.input[0] @NoStrictType@,
  l_shipmode_comp_2_1.output => l_shipmode_or_2.input[1] @NoStrictType@,
  l_shipmode_or_2.output => and_2.input[7] @NoStrictType@,

  l_shipinstruct_0.output => and_2.input[8] @NoStrictType@,
};


streamlet data_filter_s {
  p_partkey: key_stream in,
  l_partkey: key_stream in,
  p_brand: SQL_char10_stream in,
  p_container: SQL_char10_stream in,
  l_quantity: SQL_decimal_15_2_stream in,
  p_size: int_stream in,
  l_shipmode: SQL_char10_stream in,
  l_shipinstruct: SQL_char25_stream in,

  l_extendedprice_in: SQL_decimal_15_2_stream in,
  l_extendedprice_out: SQL_decimal_15_2_stream out,
  l_discount_in: SQL_decimal_15_2_stream in,
  l_discount_out: SQL_decimal_15_2_stream out,
};

impl data_filter_i of data_filter_s {
  instance selection(where_claus_i),
  p_partkey => selection.p_partkey,
  l_partkey => selection.l_partkey,
  p_brand => selection.p_brand,
  p_container => selection.p_container,
  l_quantity => selection.l_quantity,
  p_size => selection.p_size,
  l_shipmode => selection.l_shipmode,
  l_shipinstruct => selection.l_shipinstruct,

  instance l_extendedprice_filter(stream_filter_1bit_i<type SQL_decimal_15_2_stream>),
  l_extendedprice_in => l_extendedprice_filter.input,
  selection.select => l_extendedprice_filter.select,
  l_extendedprice_filter.output => l_extendedprice_out,

  instance l_discount_filter(stream_filter_1bit_i<type SQL_decimal_15_2_stream>),
  l_discount_in => l_discount_filter.input,
  selection.select => l_discount_filter.select,
  l_discount_filter.output => l_discount_out,
};

streamlet main_s {
  l_orderkey: key_stream in,
  l_linenumber: key_stream in,
  p_partkey: key_stream in,

  revenue: SQL_decimal_15_2_stream out,
  err: Stream(Bit(1)) out,
};

impl main_i of main_s {
  instance data_src_lineitem(lineitem_i),
  l_orderkey => data_src_lineitem.l_orderkey,
  l_linenumber => data_src_lineitem.l_linenumber,

  instance data_src_part(part_i),
  p_partkey => data_src_part.p_partkey,

  instance data_filter(data_filter_i),

  p_partkey => data_filter.p_partkey,
  data_src_lineitem.l_partkey => data_filter.l_partkey,
  data_src_part.p_brand => data_filter.p_brand,
  data_src_part.p_container => data_filter.p_container,
  data_src_lineitem.l_quantity => data_filter.l_quantity,
  data_src_part.p_size => data_filter.p_size,
  data_src_lineitem.l_shipmode => data_filter.l_shipmode,
  data_src_lineitem.l_shipinstruct => data_filter.l_shipinstruct,

  data_src_lineitem.l_discount => data_filter.l_discount_in,
  data_src_lineitem.l_extendedprice => data_filter.l_extendedprice_in,

  //select part
  instance const_value_gen(const_value_generator_i<type SQL_decimal_15_2_stream, 1>),
  instance to_neg(to_neg_i<type SQL_decimal_15_2_stream>),
  data_filter.l_discount_out => to_neg.input,
  instance adder(adder_i<type SQL_decimal_15_2_stream>),
  to_neg.output => adder.input1,
  const_value_gen.output => adder.input0,
  instance multiplier(multiplier_i<type SQL_decimal_15_2_stream>),
  adder.output => multiplier.input0,
  data_filter.l_extendedprice_out => multiplier.input1,
  instance accu(accumulator_i<type SQL_decimal_15_2_stream>),
  multiplier.output => accu.input,
  accu.output => revenue,

  //error
  type err_stream = Stream(Bit(1)),
  instance err_and(and_i<type err_stream, 3>),
  multiplier.overflow => err_and.input[0] @NoStrictType@,
  accu.overflow => err_and.input[1] @NoStrictType@,
  adder.overflow => err_and.input[2] @NoStrictType@,
  err_and.output => err @NoStrictType@,
};
