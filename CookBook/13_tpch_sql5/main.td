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
//construct the sql_date stream by providing its year, month, day
streamlet sql_date_constructor_s {
  year_input: year_stream in,
  month_input: month_stream in,
  day_input: day_stream in,
  date_output: date_stream out,
};

external impl sql_date_constructor_i of sql_date_constructor_s {

};

streamlet const_date_generator_s {
  date_output: date_stream out,
};

impl const_date_generator_i<day: int, month: int, year:int> of const_date_generator_s {
  instance day_gen(const_value_generator_i<type day_stream, day>),
  instance month_gen(const_value_generator_i<type month_stream, month>),
  instance year_gen(const_value_generator_i<type year_stream, year>),
  instance compositor(sql_date_constructor_i),

  day_gen.output => compositor.day_input,
  month_gen.output => compositor.month_input,
  year_gen.output => compositor.year_input,
  compositor.date_output => date_output,
};

type select_stream = Stream(Bit(1));
streamlet where_claus_s {
  c_custkey: key_stream in,
  o_custkey: key_stream in,
  l_orderkey: key_stream in,
  o_orderkey: key_stream in,
  l_suppkey: key_stream in,
  s_suppkey: key_stream in,
  c_nationkey: key_stream in,
  s_nationkey: key_stream in,
  n_nationkey: key_stream in,
  n_regionkey: key_stream in,
  r_regionkey: key_stream in,
  r_name: SQL_char25_stream in,
  o_orderdate: date_stream in,

  select: select_stream out,
};

streamlet const_char25_generator_s<value: str> {
  output: SQL_char25_stream out,
};

external impl const_char25_generator_i<value: str> of const_char25_generator_s<value> {

};

impl where_claus_i of where_claus_s {
  instance and_0(and_i<type select_stream, 9>),
  and_0.output => select,

  instance compare(comparator_is_equal_i<type key_stream>) [6],
  c_custkey => compare[0].input0,
  o_custkey => compare[0].input1,
  compare[0].output => and_0.input[0] @NoStrictType@,

  l_orderkey => compare[1].input0,
  o_orderkey => compare[1].input1,
  compare[1].output => and_0.input[1] @NoStrictType@,

  c_nationkey => compare[2].input0,
  s_nationkey => compare[2].input1,
  compare[2].output => and_0.input[2] @NoStrictType@,

  c_nationkey => compare[3].input0,
  n_nationkey => compare[3].input1,
  compare[3].output => and_0.input[3] @NoStrictType@,

  s_nationkey => compare[4].input0,
  s_nationkey => compare[4].input1,
  compare[4].output => and_0.input[4] @NoStrictType@,

  n_regionkey => compare[5].input0,
  r_regionkey => compare[5].input1,
  compare[5].output => and_0.input[5] @NoStrictType@,

  instance o_orderdate_standard_gen1(const_date_generator_i<1,1,1997>),
  instance compare2(comparator_is_larger_equal_i<type date_stream>),
  o_orderdate => compare2.input0,
  o_orderdate_standard_gen1.date_output => compare2.input1,
  compare2.output => and_0.input[6] @NoStrictType@,

  instance o_orderdate_standard_gen2(const_date_generator_i<1,1,1997>),
  instance compare3(comparator_is_smaller_i<type date_stream>),
  o_orderdate => compare3.input0,
  o_orderdate_standard_gen2.date_output => compare3.input1,
  compare3.output => and_0.input[7] @NoStrictType@,

  instance r_name_standard_gen(const_char25_generator_i<"Asia">),
  instance compare4(comparator_is_equal_i<type SQL_char25_stream>),
  r_name => compare4.input0,
  r_name_standard_gen.output => compare4.input1,
  compare4.output => and_0.input[8] @NoStrictType@,
};

streamlet data_filter_s {
  n_name_in: SQL_char25_stream in,
  n_name_out: SQL_char25_stream out,
  l_extendedprice_in: SQL_decimal_15_2_stream in,
  l_extendedprice_out: SQL_decimal_15_2_stream out,
  l_discount_in: SQL_decimal_15_2_stream in,
  l_discount_out: SQL_decimal_15_2_stream out,

  c_custkey: key_stream in,
  o_custkey: key_stream in,
  l_orderkey: key_stream in,
  o_orderkey: key_stream in,
  l_suppkey: key_stream in,
  s_suppkey: key_stream in,
  c_nationkey: key_stream in,
  s_nationkey: key_stream in,
  n_nationkey: key_stream in,
  n_regionkey: key_stream in,
  r_regionkey: key_stream in,
  r_name: SQL_char25_stream in,
  o_orderdate: date_stream in,
};

impl data_filter_i of data_filter_s {
  instance selection(where_claus_i),
  c_custkey => selection.c_custkey,
  o_custkey => selection.o_custkey,
  l_orderkey => selection.l_orderkey,
  o_orderkey => selection.o_orderkey,
  l_suppkey => selection.l_suppkey,
  s_suppkey => selection.s_suppkey,
  c_nationkey => selection.c_nationkey,
  s_nationkey => selection.s_nationkey,
  n_nationkey => selection.n_nationkey,
  n_regionkey => selection.n_regionkey,
  r_regionkey => selection.r_regionkey,
  r_name => selection.r_name,
  o_orderdate => selection.o_orderdate,

  instance n_name_filter(stream_filter_1bit_i<type SQL_char25_stream>),
  n_name_in => n_name_filter.input,
  selection.select => n_name_filter.select,
  n_name_filter.output => n_name_out,

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
  c_custkey: key_stream in,
  o_orderkey: key_stream in,
  l_orderkey: key_stream in,
  l_linenumber: key_stream in,
  s_suppkey: key_stream in,
  n_nationkey: key_stream in,
  r_regionkey: key_stream in,

  n_name: SQL_char25_stream out,
  revenue: SQL_decimal_15_2_stream out,
  err: Stream(Bit(1)) out,
};

impl main_i of main_s {
  instance data_src_customer(customer_i),
  c_custkey => data_src_customer.c_custkey,

  instance data_src_orders(orders_i),
  o_orderkey => data_src_orders.o_orderkey,

  instance data_src_lineitem(lineitem_i),
  l_orderkey => data_src_lineitem.l_orderkey,
  l_linenumber => data_src_lineitem.l_linenumber,

  instance data_src_supplier(supplier_i),
  s_suppkey => data_src_supplier.s_suppkey,

  instance data_src_nation(nation_i),
  n_nationkey => data_src_nation.n_nationkey,

  instance data_src_region(region_i),
  r_regionkey => data_src_region.r_regionkey,

  instance data_filter(data_filter_i),
  c_custkey => data_filter.c_custkey,
  data_src_orders.o_custkey => data_filter.o_custkey,
  l_orderkey => data_filter.l_orderkey,
  o_orderkey => data_filter.o_orderkey,
  data_src_lineitem.l_suppkey => data_filter.l_suppkey,
  s_suppkey => data_filter.s_suppkey,
  data_src_customer.c_nationkey => data_filter.c_nationkey,
  data_src_supplier.s_nationkey => data_filter.s_nationkey,
  n_nationkey => data_filter.n_nationkey,
  data_src_nation.n_regionkey => data_filter.n_regionkey,
  r_regionkey => data_filter.r_regionkey,
  data_src_region.r_name => data_filter.r_name,
  data_src_orders.o_orderdate => data_filter.o_orderdate,

  data_src_nation.n_name => data_filter.n_name_in,
  data_src_lineitem.l_extendedprice => data_filter.l_extendedprice_in,
  data_src_lineitem.l_discount => data_filter.l_discount_in,

  //select part
  data_filter.n_name_out => n_name,
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
  err_and.output => err,
};
