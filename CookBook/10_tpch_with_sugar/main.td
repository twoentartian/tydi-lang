package std;

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

streamlet data_filter_s {
  l_partkey_in: key_stream in,
  l_suppkey_in: key_stream in,
  l_quantity_in: SQL_decimal_15_2_stream in,
  l_extendedprice_in: SQL_decimal_15_2_stream in,
  l_discount_in: SQL_decimal_15_2_stream in,
  l_tax_in: SQL_decimal_15_2_stream in,
  l_returnflag_in: SQL_char1_stream in,
  l_linestatus_in: SQL_char1_stream in,
  l_shipdate_in: date_stream in,
  l_commitdate_in: date_stream in,
  l_receiptdate_in: date_stream in,
  l_shipinstruct_in: SQL_char25_stream in,
  l_shipmode_in: SQL_char10_stream in,
  l_comment_in: varchar_stream in,

  l_partkey_out: key_stream out,
  l_suppkey_out: key_stream out,
  l_quantity_out: SQL_decimal_15_2_stream out,
  l_extendedprice_out: SQL_decimal_15_2_stream out,
  l_discount_out: SQL_decimal_15_2_stream out,
  l_tax_out: SQL_decimal_15_2_stream out,
  l_returnflag_out: SQL_char1_stream out,
  l_linestatus_out: SQL_char1_stream out,
  l_shipdate_out: date_stream out,
  l_commitdate_out: date_stream out,
  l_receiptdate_out: date_stream out,
  l_shipinstruct_out: SQL_char25_stream out,
  l_shipmode_out: SQL_char10_stream out,
  l_comment_out: varchar_stream out,

};

impl data_filter_i of data_filter_s {
  instance baseline_date(const_date_generator_i<1,12,1998>),
  instance compare_date(comparator_i<type date_stream>),
  l_shipdate_in => compare_date.input0,
  baseline_date.date_output => compare_date.input1,

  instance l_partkey_bypass(stream_filter_i<type key_stream>),
  l_partkey_in => l_partkey_bypass.input,
  compare_date.output => l_partkey_bypass.select,
  l_partkey_bypass.output => l_partkey_out,

  instance l_suppkey_bypass(stream_filter_i<type key_stream>),
  l_suppkey_in => l_suppkey_bypass.input,
  compare_date.output => l_suppkey_bypass.select,
  l_suppkey_bypass.output => l_suppkey_out,

  instance l_quantity_bypass(stream_filter_i<type SQL_decimal_15_2_stream>),
  l_quantity_in => l_quantity_bypass.input,
  compare_date.output => l_quantity_bypass.select,
  l_quantity_bypass.output => l_quantity_out,

  instance l_extendedprice_bypass(stream_filter_i<type SQL_decimal_15_2_stream>),
  l_extendedprice_in => l_extendedprice_bypass.input,
  compare_date.output => l_extendedprice_bypass.select,
  l_extendedprice_bypass.output => l_extendedprice_out,

  instance l_discount_bypass(stream_filter_i<type SQL_decimal_15_2_stream>),
  l_discount_in => l_discount_bypass.input,
  compare_date.output => l_discount_bypass.select,
  l_discount_bypass.output => l_discount_out,

  instance l_tax_bypass(stream_filter_i<type SQL_decimal_15_2_stream>),
  l_tax_in => l_tax_bypass.input,
  compare_date.output => l_tax_bypass.select,
  l_tax_bypass.output => l_tax_out,

  instance l_returnflag_bypass(stream_filter_i<type SQL_char1_stream>),
  l_returnflag_in => l_returnflag_bypass.input,
  compare_date.output => l_returnflag_bypass.select,
  l_returnflag_bypass.output => l_returnflag_out,

  instance l_linestatus_bypass(stream_filter_i<type SQL_char1_stream>),
  l_linestatus_in => l_linestatus_bypass.input,
  compare_date.output => l_linestatus_bypass.select,
  l_linestatus_bypass.output => l_linestatus_out,

  instance l_shipdate_bypass(stream_filter_i<type date_stream>),
  l_shipdate_in => l_shipdate_bypass.input,
  compare_date.output => l_shipdate_bypass.select,
  l_shipdate_bypass.output => l_shipdate_out,

  instance l_commitdate_bypass(stream_filter_i<type date_stream>),
  l_commitdate_in => l_commitdate_bypass.input,
  compare_date.output => l_commitdate_bypass.select,
  l_commitdate_bypass.output => l_commitdate_out,

  instance l_receiptdate_bypass(stream_filter_i<type date_stream>),
  l_receiptdate_in => l_receiptdate_bypass.input,
  compare_date.output => l_receiptdate_bypass.select,
  l_receiptdate_bypass.output => l_receiptdate_out,

  instance l_shipinstruct_bypass(stream_filter_i<type SQL_char25_stream>),
  l_shipinstruct_in => l_shipinstruct_bypass.input,
  compare_date.output => l_shipinstruct_bypass.select,
  l_shipinstruct_bypass.output => l_shipinstruct_out,

  instance l_shipmode_bypass(stream_filter_i<type SQL_char10_stream>),
  l_shipmode_in => l_shipmode_bypass.input,
  compare_date.output => l_shipmode_bypass.select,
  l_shipmode_bypass.output => l_shipmode_out,

  instance l_comment_bypass(stream_filter_i<type varchar_stream>),
  l_comment_in => l_comment_bypass.input,
  compare_date.output => l_comment_bypass.select,
  l_comment_bypass.output => l_comment_out,
};

// col: sum_qty, sum_base_price, avg_price
streamlet sum_qty_s {
  l_quantity: SQL_decimal_15_2_stream in,
  l_extendedprice: SQL_decimal_15_2_stream in,
  sum_qty: SQL_decimal_15_2_stream out,
  sum_base_price: SQL_decimal_15_2_stream out,
  avg_price: SQL_decimal_15_2_stream out,
  error: Stream(Bit(1)) out,
};

impl sum_qty_i of sum_qty_s {
  type count_type = streamlet accumulator_s<type SQL_decimal_15_2_stream>.count_type,
  instance accu0(accumulator_i<type SQL_decimal_15_2_stream>),
  instance accu1(accumulator_i<type SQL_decimal_15_2_stream>),

  l_quantity => accu0.input,
  accu0.output => sum_qty,

  l_extendedprice => accu1.input,

  instance avg_price_divider(divider_i<type SQL_decimal_15_2_stream>),
  accu1.output => sum_base_price,
  accu1.output => avg_price_divider.dividend,

  instance converter(converter_i<type count_type, type SQL_decimal_15_2_stream, 1>),
  accu1.count => converter.input[0],
  converter.output[0] => avg_price_divider.divisor,
  avg_price_divider.quotient => avg_price,

  //error
  type error_stream = Stream(Bit(1)),
  instance and(and_i<type error_stream, 2>),
  accu0.overflow => and.input[0] @NoStrictType@,
  accu1.overflow => and.input[1] @NoStrictType@,
  and.output => error @NoStrictType@,
};


// col: sum_disc_price, sum_charge
streamlet sum_disc_price_s {
  l_extendedprice: SQL_decimal_15_2_stream in,
  l_discount: SQL_decimal_15_2_stream in,
  l_tax: SQL_decimal_15_2_stream in,
  sum_disc_price: SQL_decimal_15_2_stream out,
  sum_charge: SQL_decimal_15_2_stream out,

  error: Stream(Bit(1)) out,
};

impl sum_disc_price_i of sum_disc_price_s {
  instance const_decimal_15_value(const_value_generator_i<type SQL_decimal_15_2_stream, 1>),
  instance neg(to_neg_i<type SQL_decimal_15_2_stream>),
  instance adder(adder_i<type SQL_decimal_15_2_stream>),

  //calculate sum_disc_price
  const_decimal_15_value.output => adder.input0,
  l_discount => neg.input,
  neg.output => adder.input1,
  instance multiplier(multiplier_i<type SQL_decimal_15_2_stream>),
  adder.output => multiplier.input0,
  l_extendedprice => multiplier.input1,
  multiplier.output => sum_disc_price, //sum_disc_price

  //calculate sum_charge
  instance multiplier2(multiplier_i<type SQL_decimal_15_2_stream>),
  instance const_decimal_15_value2(const_value_generator_i<type SQL_decimal_15_2_stream, 1>),
  instance adder2(adder_i<type SQL_decimal_15_2_stream>),
  l_tax => adder2.input0,
  const_decimal_15_value2.output => adder2.input1,
  adder2.output => multiplier2.input0,
  multiplier.output => multiplier2.input1,
  multiplier2.output => sum_charge,  //sum_charge

  //error handling
  type error_stream = Stream(Bit(1)),
  instance and(and_i<type error_stream,4>),
  multiplier.overflow => and.input[0] @NoStrictType@,
  adder.overflow => and.input[1] @NoStrictType@,
  adder2.overflow => and.input[2] @NoStrictType@,
  multiplier2.overflow => and.input[3] @NoStrictType@,
  and.output => error @NoStrictType@,
};

// col: avg_qty, avg_disc, count_order
streamlet avg_qty_s {
  l_quantity: SQL_decimal_15_2_stream in,
  l_discount: SQL_decimal_15_2_stream in,
  avg_qty: SQL_decimal_15_2_stream out,
  avg_disc: SQL_decimal_15_2_stream out,
  count_order: Stream(Bit(32)) out,

  error: Stream(Bit(1)) out,
};

impl avg_qty_i of avg_qty_s {
  instance accu0(accumulator_i<type SQL_decimal_15_2_stream>),
  instance divider0(divider_i<type SQL_decimal_15_2_stream>),
  instance accu1(accumulator_i<type SQL_decimal_15_2_stream>),
  instance divider1(divider_i<type SQL_decimal_15_2_stream>),
  instance converter(converter_i<type count_type, type SQL_decimal_15_2_stream, 2>),

  l_quantity => accu0.input,
  accu0.output => divider0.dividend,
  accu0.count => converter.input[1],
  converter.output[1] => divider0.divisor,
  divider0.quotient => avg_qty,

  l_discount => accu1.input,
  accu1.output => divider1.dividend,
  type count_type = streamlet accumulator_s<type SQL_decimal_15_2_stream>.count_type,

  accu1.count => converter.input[0],
  converter.output[0] => divider1.divisor,
  accu1.count => count_order @NoStrictType@,
  divider1.quotient => avg_disc,

  //error
  type error_stream = Stream(Bit(1)),
  instance and(and_i<type error_stream, 2>),
  accu0.overflow => and.input[0] @NoStrictType@,
  accu1.overflow => and.input[1] @NoStrictType@,
  and.output => error @NoStrictType@,
};



//main component
streamlet main_s {
  l_orderkey: key_stream in,
  l_linenumber: key_stream in,

  l_returnflag: SQL_char1_stream out,
  l_linestatus: SQL_char1_stream out,

  sum_qty: SQL_decimal_15_2_stream out,         //part0
  sum_base_price: SQL_decimal_15_2_stream out,  //part0
  sum_disc_price: SQL_decimal_15_2_stream out,  //part1
  sum_charge: SQL_decimal_15_2_stream out,      //part1
  avg_qty: SQL_decimal_15_2_stream out,         //part2
  avg_price: SQL_decimal_15_2_stream out,       //part0
  avg_disc: SQL_decimal_15_2_stream out,        //part2
  count_order: Stream(Bit(32)) out,             //part2

  err: Stream(Bit(1)) out,
};

impl main_i of main_s {
  instance data_src(lineitem_i),
  l_orderkey => data_src.l_orderkey,
  l_linenumber => data_src.l_linenumber,

  instance data_filter(data_filter_i),
  data_src.l_partkey => data_filter.l_partkey_in,
  data_src.l_suppkey => data_filter.l_suppkey_in,
  data_src.l_quantity => data_filter.l_quantity_in,
  data_src.l_extendedprice => data_filter.l_extendedprice_in,
  data_src.l_discount => data_filter.l_discount_in,
  data_src.l_tax => data_filter.l_tax_in,
  data_src.l_returnflag => data_filter.l_returnflag_in,
  data_src.l_linestatus => data_filter.l_linestatus_in,
  data_src.l_shipdate => data_filter.l_shipdate_in,
  data_src.l_commitdate => data_filter.l_commitdate_in,
  data_src.l_receiptdate => data_filter.l_receiptdate_in,
  data_src.l_shipinstruct => data_filter.l_shipinstruct_in,
  data_src.l_shipmode => data_filter.l_shipmode_in,
  data_src.l_comment => data_filter.l_comment_in,

  data_filter.l_returnflag_out => l_returnflag,
  data_filter.l_linestatus_out => l_linestatus,

  //part0
  instance part0(sum_qty_i),
  data_filter.l_quantity_out => part0.l_quantity,
  data_filter.l_extendedprice_out => part0.l_extendedprice,
  part0.sum_qty => sum_qty,
  part0.sum_base_price => sum_base_price,
  part0.avg_price => avg_price,

  //part1
  instance part1(sum_disc_price_i),
  data_filter.l_extendedprice_out => part1.l_extendedprice,
  data_filter.l_discount_out => part1.l_discount,
  data_filter.l_tax_out => part1.l_tax,
  part1.sum_disc_price => sum_disc_price,
  part1.sum_charge => sum_charge,

  //part2
  instance part2(avg_qty_i),
  data_filter.l_quantity_out => part2.l_quantity,
  data_filter.l_discount_out => part2.l_discount,
  part2.avg_qty => avg_qty,
  part2.avg_disc => avg_disc,
  part2.count_order => count_order @NoStrictType@,

  //error
  type error_stream = Stream(Bit(1)),
  instance and(and_i<type error_stream, 3>),
  part0.error => and.input[0] @NoStrictType@,
  part1.error => and.input[1] @NoStrictType@,
  part2.error => and.input[2] @NoStrictType@,
  and.output => err @NoStrictType@,
};
