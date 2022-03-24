package tpch;

type SQL_int = Bit(32);
type int_stream = Stream(SQL_int, d = 1);
type key_stream = int_stream;

const year_max = 10^5 - 1;
type year_t = Bit(ceil(log2(year_max)));
const month_max = 12;
type month_t = Bit(ceil(log2(month_max)));
const day_max = 31;
type day_t = Bit(ceil(log2(day_max)));
type Group Date {
  year: year_t,
  month: month_t,
  day: day_t,
};
type data_stream = Stream(Date, d = 1);

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
type SQL_decimal_15 = Bit(ceil(log2(max_decimal_15)));
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
  o_orderdate: data_stream out,
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
  l_quantity: SQL_char15_stream out,
  l_extendedprice: SQL_char15_stream out,
  l_discount: SQL_char15_stream out,
  l_tax: SQL_char15_stream out,
  l_returnflag: SQL_char1_stream out,
  l_linestatus: SQL_char1_stream out,
  l_shipdate: data_stream out,
  l_commitdate: data_stream out,
  l_receiptdate: data_stream out,
  l_shipinstruct: SQL_char25_stream out,
  l_shipmode: SQL_char10_stream out,
  l_comment: varchar_stream out,
};

external impl lineitem_i of lineitem_s {

};

////////////////////  tydi standard lib  ////////////////////
streamlet void_s<type_in: type> {
  stream_in: type_in in,
};

external impl void_i<type_in: type> of void_s<type type_in> {

};

type stream_bit8 = Stream(Bit(8));
impl void_i_bit8(void_i<type Stream(Bit(8))>);

