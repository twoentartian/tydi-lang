library ieee;
use ieee.std_logic_1164.all;

library work;
use work.proj.all;

entity test_project__tpch__main_i_com is
  port (
    clk : in std_logic;
    rst : in std_logic;
    l_linenumber_valid : in std_logic;
    l_linenumber_ready : out std_logic;
    l_linenumber_data : in std_logic_vector(31 downto 0);
    l_linenumber_last : in std_logic;
    l_linenumber_strb : in std_logic;
    o_orderkey_valid : in std_logic;
    o_orderkey_ready : out std_logic;
    o_orderkey_data : in std_logic_vector(31 downto 0);
    o_orderkey_last : in std_logic;
    o_orderkey_strb : in std_logic;
    l_orderkey_valid : in std_logic;
    l_orderkey_ready : out std_logic;
    l_orderkey_data : in std_logic_vector(31 downto 0);
    l_orderkey_last : in std_logic;
    l_orderkey_strb : in std_logic;
    c_custkey_valid : in std_logic;
    c_custkey_ready : out std_logic;
    c_custkey_data : in std_logic_vector(31 downto 0);
    c_custkey_last : in std_logic;
    c_custkey_strb : in std_logic;
    o_shippriority_valid : out std_logic;
    o_shippriority_ready : in std_logic;
    o_shippriority_data : out std_logic_vector(31 downto 0);
    o_shippriority_last : out std_logic;
    o_shippriority_strb : out std_logic;
    revenue_valid : out std_logic;
    revenue_ready : in std_logic;
    revenue_data : out std_logic_vector(49 downto 0);
    revenue_last : out std_logic;
    revenue_strb : out std_logic;
    err_valid : out std_logic;
    err_ready : in std_logic;
    err_data : out std_logic;
    err_strb : out std_logic;
    o_orderdate_valid : out std_logic;
    o_orderdate_ready : in std_logic;
    o_orderdate_data : out std_logic_vector(25 downto 0);
    o_orderdate_last : out std_logic;
    o_orderdate_strb : out std_logic;
    l_orderkey_out_valid : out std_logic;
    l_orderkey_out_ready : in std_logic;
    l_orderkey_out_data : out std_logic_vector(31 downto 0);
    l_orderkey_out_last : out std_logic;
    l_orderkey_out_strb : out std_logic
  );
end test_project__tpch__main_i_com;

architecture test_project__tpch__main_i of test_project__tpch__main_i_com is
  signal accu__count_valid : std_logic;
  signal accu__count_ready : std_logic;
  signal accu__count_data : std_logic_vector(31 downto 0);
  signal accu__count_strb : std_logic;
  signal accu__input_valid : std_logic;
  signal accu__input_ready : std_logic;
  signal accu__input_data : std_logic_vector(49 downto 0);
  signal accu__input_last : std_logic;
  signal accu__input_strb : std_logic;
  signal accu__output_valid : std_logic;
  signal accu__output_ready : std_logic;
  signal accu__output_data : std_logic_vector(49 downto 0);
  signal accu__output_last : std_logic;
  signal accu__output_strb : std_logic;
  signal accu__overflow_valid : std_logic;
  signal accu__overflow_ready : std_logic;
  signal accu__overflow_data : std_logic;
  signal accu__overflow_strb : std_logic;
  signal adder__input0_valid : std_logic;
  signal adder__input0_ready : std_logic;
  signal adder__input0_data : std_logic_vector(49 downto 0);
  signal adder__input0_last : std_logic;
  signal adder__input0_strb : std_logic;
  signal adder__input1_valid : std_logic;
  signal adder__input1_ready : std_logic;
  signal adder__input1_data : std_logic_vector(49 downto 0);
  signal adder__input1_last : std_logic;
  signal adder__input1_strb : std_logic;
  signal adder__output_valid : std_logic;
  signal adder__output_ready : std_logic;
  signal adder__output_data : std_logic_vector(49 downto 0);
  signal adder__output_last : std_logic;
  signal adder__output_strb : std_logic;
  signal adder__overflow_valid : std_logic;
  signal adder__overflow_ready : std_logic;
  signal adder__overflow_data : std_logic;
  signal adder__overflow_strb : std_logic;
  signal const_value_gen__output_valid : std_logic;
  signal const_value_gen__output_ready : std_logic;
  signal const_value_gen__output_data : std_logic_vector(49 downto 0);
  signal const_value_gen__output_last : std_logic;
  signal const_value_gen__output_strb : std_logic;
  signal data_filter__c_custkey_valid : std_logic;
  signal data_filter__c_custkey_ready : std_logic;
  signal data_filter__c_custkey_data : std_logic_vector(31 downto 0);
  signal data_filter__c_custkey_last : std_logic;
  signal data_filter__c_custkey_strb : std_logic;
  signal data_filter__c_mktsegment_valid : std_logic;
  signal data_filter__c_mktsegment_ready : std_logic;
  signal data_filter__c_mktsegment_data : std_logic_vector(79 downto 0);
  signal data_filter__c_mktsegment_last : std_logic;
  signal data_filter__c_mktsegment_strb : std_logic;
  signal data_filter__l_discount_in_valid : std_logic;
  signal data_filter__l_discount_in_ready : std_logic;
  signal data_filter__l_discount_in_data : std_logic_vector(49 downto 0);
  signal data_filter__l_discount_in_last : std_logic;
  signal data_filter__l_discount_in_strb : std_logic;
  signal data_filter__l_discount_out_valid : std_logic;
  signal data_filter__l_discount_out_ready : std_logic;
  signal data_filter__l_discount_out_data : std_logic_vector(49 downto 0);
  signal data_filter__l_discount_out_last : std_logic;
  signal data_filter__l_discount_out_strb : std_logic;
  signal data_filter__l_extendedprice_in_valid : std_logic;
  signal data_filter__l_extendedprice_in_ready : std_logic;
  signal data_filter__l_extendedprice_in_data : std_logic_vector(49 downto 0);
  signal data_filter__l_extendedprice_in_last : std_logic;
  signal data_filter__l_extendedprice_in_strb : std_logic;
  signal data_filter__l_extendedprice_out_valid : std_logic;
  signal data_filter__l_extendedprice_out_ready : std_logic;
  signal data_filter__l_extendedprice_out_data : std_logic_vector(49 downto 0);
  signal data_filter__l_extendedprice_out_last : std_logic;
  signal data_filter__l_extendedprice_out_strb : std_logic;
  signal data_filter__l_orderkey_valid : std_logic;
  signal data_filter__l_orderkey_ready : std_logic;
  signal data_filter__l_orderkey_data : std_logic_vector(31 downto 0);
  signal data_filter__l_orderkey_last : std_logic;
  signal data_filter__l_orderkey_strb : std_logic;
  signal data_filter__l_orderkey_in_valid : std_logic;
  signal data_filter__l_orderkey_in_ready : std_logic;
  signal data_filter__l_orderkey_in_data : std_logic_vector(31 downto 0);
  signal data_filter__l_orderkey_in_last : std_logic;
  signal data_filter__l_orderkey_in_strb : std_logic;
  signal data_filter__l_orderkey_out_valid : std_logic;
  signal data_filter__l_orderkey_out_ready : std_logic;
  signal data_filter__l_orderkey_out_data : std_logic_vector(31 downto 0);
  signal data_filter__l_orderkey_out_last : std_logic;
  signal data_filter__l_orderkey_out_strb : std_logic;
  signal data_filter__l_shipdate_valid : std_logic;
  signal data_filter__l_shipdate_ready : std_logic;
  signal data_filter__l_shipdate_data : std_logic_vector(25 downto 0);
  signal data_filter__l_shipdate_last : std_logic;
  signal data_filter__l_shipdate_strb : std_logic;
  signal data_filter__o_custkey_valid : std_logic;
  signal data_filter__o_custkey_ready : std_logic;
  signal data_filter__o_custkey_data : std_logic_vector(31 downto 0);
  signal data_filter__o_custkey_last : std_logic;
  signal data_filter__o_custkey_strb : std_logic;
  signal data_filter__o_orderdate_valid : std_logic;
  signal data_filter__o_orderdate_ready : std_logic;
  signal data_filter__o_orderdate_data : std_logic_vector(25 downto 0);
  signal data_filter__o_orderdate_last : std_logic;
  signal data_filter__o_orderdate_strb : std_logic;
  signal data_filter__o_orderdate_in_valid : std_logic;
  signal data_filter__o_orderdate_in_ready : std_logic;
  signal data_filter__o_orderdate_in_data : std_logic_vector(25 downto 0);
  signal data_filter__o_orderdate_in_last : std_logic;
  signal data_filter__o_orderdate_in_strb : std_logic;
  signal data_filter__o_orderdate_out_valid : std_logic;
  signal data_filter__o_orderdate_out_ready : std_logic;
  signal data_filter__o_orderdate_out_data : std_logic_vector(25 downto 0);
  signal data_filter__o_orderdate_out_last : std_logic;
  signal data_filter__o_orderdate_out_strb : std_logic;
  signal data_filter__o_orderkey_valid : std_logic;
  signal data_filter__o_orderkey_ready : std_logic;
  signal data_filter__o_orderkey_data : std_logic_vector(31 downto 0);
  signal data_filter__o_orderkey_last : std_logic;
  signal data_filter__o_orderkey_strb : std_logic;
  signal data_filter__o_shippriority_in_valid : std_logic;
  signal data_filter__o_shippriority_in_ready : std_logic;
  signal data_filter__o_shippriority_in_data : std_logic_vector(31 downto 0);
  signal data_filter__o_shippriority_in_last : std_logic;
  signal data_filter__o_shippriority_in_strb : std_logic;
  signal data_filter__o_shippriority_out_valid : std_logic;
  signal data_filter__o_shippriority_out_ready : std_logic;
  signal data_filter__o_shippriority_out_data : std_logic_vector(31 downto 0);
  signal data_filter__o_shippriority_out_last : std_logic;
  signal data_filter__o_shippriority_out_strb : std_logic;
  signal data_src_customer__c_acctbal_valid : std_logic;
  signal data_src_customer__c_acctbal_ready : std_logic;
  signal data_src_customer__c_acctbal_data : std_logic_vector(49 downto 0);
  signal data_src_customer__c_acctbal_last : std_logic;
  signal data_src_customer__c_acctbal_strb : std_logic;
  signal data_src_customer__c_address_valid : std_logic;
  signal data_src_customer__c_address_ready : std_logic;
  signal data_src_customer__c_address_data : std_logic_vector(7 downto 0);
  signal data_src_customer__c_address_last : std_logic_vector(1 downto 0);
  signal data_src_customer__c_address_strb : std_logic;
  signal data_src_customer__c_comment_valid : std_logic;
  signal data_src_customer__c_comment_ready : std_logic;
  signal data_src_customer__c_comment_data : std_logic_vector(7 downto 0);
  signal data_src_customer__c_comment_last : std_logic_vector(1 downto 0);
  signal data_src_customer__c_comment_strb : std_logic;
  signal data_src_customer__c_custkey_valid : std_logic;
  signal data_src_customer__c_custkey_ready : std_logic;
  signal data_src_customer__c_custkey_data : std_logic_vector(31 downto 0);
  signal data_src_customer__c_custkey_last : std_logic;
  signal data_src_customer__c_custkey_strb : std_logic;
  signal data_src_customer__c_mktsegment_valid : std_logic;
  signal data_src_customer__c_mktsegment_ready : std_logic;
  signal data_src_customer__c_mktsegment_data : std_logic_vector(79 downto 0);
  signal data_src_customer__c_mktsegment_last : std_logic;
  signal data_src_customer__c_mktsegment_strb : std_logic;
  signal data_src_customer__c_name_valid : std_logic;
  signal data_src_customer__c_name_ready : std_logic;
  signal data_src_customer__c_name_data : std_logic_vector(7 downto 0);
  signal data_src_customer__c_name_last : std_logic_vector(1 downto 0);
  signal data_src_customer__c_name_strb : std_logic;
  signal data_src_customer__c_nationkey_valid : std_logic;
  signal data_src_customer__c_nationkey_ready : std_logic;
  signal data_src_customer__c_nationkey_data : std_logic_vector(31 downto 0);
  signal data_src_customer__c_nationkey_last : std_logic;
  signal data_src_customer__c_nationkey_strb : std_logic;
  signal data_src_customer__c_phone_valid : std_logic;
  signal data_src_customer__c_phone_ready : std_logic;
  signal data_src_customer__c_phone_data : std_logic_vector(119 downto 0);
  signal data_src_customer__c_phone_last : std_logic;
  signal data_src_customer__c_phone_strb : std_logic;
  signal data_src_lineitem__l_comment_valid : std_logic;
  signal data_src_lineitem__l_comment_ready : std_logic;
  signal data_src_lineitem__l_comment_data : std_logic_vector(7 downto 0);
  signal data_src_lineitem__l_comment_last : std_logic_vector(1 downto 0);
  signal data_src_lineitem__l_comment_strb : std_logic;
  signal data_src_lineitem__l_commitdate_valid : std_logic;
  signal data_src_lineitem__l_commitdate_ready : std_logic;
  signal data_src_lineitem__l_commitdate_data : std_logic_vector(25 downto 0);
  signal data_src_lineitem__l_commitdate_last : std_logic;
  signal data_src_lineitem__l_commitdate_strb : std_logic;
  signal data_src_lineitem__l_discount_valid : std_logic;
  signal data_src_lineitem__l_discount_ready : std_logic;
  signal data_src_lineitem__l_discount_data : std_logic_vector(49 downto 0);
  signal data_src_lineitem__l_discount_last : std_logic;
  signal data_src_lineitem__l_discount_strb : std_logic;
  signal data_src_lineitem__l_extendedprice_valid : std_logic;
  signal data_src_lineitem__l_extendedprice_ready : std_logic;
  signal data_src_lineitem__l_extendedprice_data : std_logic_vector(49 downto 0);
  signal data_src_lineitem__l_extendedprice_last : std_logic;
  signal data_src_lineitem__l_extendedprice_strb : std_logic;
  signal data_src_lineitem__l_linenumber_valid : std_logic;
  signal data_src_lineitem__l_linenumber_ready : std_logic;
  signal data_src_lineitem__l_linenumber_data : std_logic_vector(31 downto 0);
  signal data_src_lineitem__l_linenumber_last : std_logic;
  signal data_src_lineitem__l_linenumber_strb : std_logic;
  signal data_src_lineitem__l_linestatus_valid : std_logic;
  signal data_src_lineitem__l_linestatus_ready : std_logic;
  signal data_src_lineitem__l_linestatus_data : std_logic_vector(7 downto 0);
  signal data_src_lineitem__l_linestatus_last : std_logic;
  signal data_src_lineitem__l_linestatus_strb : std_logic;
  signal data_src_lineitem__l_orderkey_valid : std_logic;
  signal data_src_lineitem__l_orderkey_ready : std_logic;
  signal data_src_lineitem__l_orderkey_data : std_logic_vector(31 downto 0);
  signal data_src_lineitem__l_orderkey_last : std_logic;
  signal data_src_lineitem__l_orderkey_strb : std_logic;
  signal data_src_lineitem__l_partkey_valid : std_logic;
  signal data_src_lineitem__l_partkey_ready : std_logic;
  signal data_src_lineitem__l_partkey_data : std_logic_vector(31 downto 0);
  signal data_src_lineitem__l_partkey_last : std_logic;
  signal data_src_lineitem__l_partkey_strb : std_logic;
  signal data_src_lineitem__l_quantity_valid : std_logic;
  signal data_src_lineitem__l_quantity_ready : std_logic;
  signal data_src_lineitem__l_quantity_data : std_logic_vector(49 downto 0);
  signal data_src_lineitem__l_quantity_last : std_logic;
  signal data_src_lineitem__l_quantity_strb : std_logic;
  signal data_src_lineitem__l_receiptdate_valid : std_logic;
  signal data_src_lineitem__l_receiptdate_ready : std_logic;
  signal data_src_lineitem__l_receiptdate_data : std_logic_vector(25 downto 0);
  signal data_src_lineitem__l_receiptdate_last : std_logic;
  signal data_src_lineitem__l_receiptdate_strb : std_logic;
  signal data_src_lineitem__l_returnflag_valid : std_logic;
  signal data_src_lineitem__l_returnflag_ready : std_logic;
  signal data_src_lineitem__l_returnflag_data : std_logic_vector(7 downto 0);
  signal data_src_lineitem__l_returnflag_last : std_logic;
  signal data_src_lineitem__l_returnflag_strb : std_logic;
  signal data_src_lineitem__l_shipdate_valid : std_logic;
  signal data_src_lineitem__l_shipdate_ready : std_logic;
  signal data_src_lineitem__l_shipdate_data : std_logic_vector(25 downto 0);
  signal data_src_lineitem__l_shipdate_last : std_logic;
  signal data_src_lineitem__l_shipdate_strb : std_logic;
  signal data_src_lineitem__l_shipinstruct_valid : std_logic;
  signal data_src_lineitem__l_shipinstruct_ready : std_logic;
  signal data_src_lineitem__l_shipinstruct_data : std_logic_vector(199 downto 0);
  signal data_src_lineitem__l_shipinstruct_last : std_logic;
  signal data_src_lineitem__l_shipinstruct_strb : std_logic;
  signal data_src_lineitem__l_shipmode_valid : std_logic;
  signal data_src_lineitem__l_shipmode_ready : std_logic;
  signal data_src_lineitem__l_shipmode_data : std_logic_vector(79 downto 0);
  signal data_src_lineitem__l_shipmode_last : std_logic;
  signal data_src_lineitem__l_shipmode_strb : std_logic;
  signal data_src_lineitem__l_suppkey_valid : std_logic;
  signal data_src_lineitem__l_suppkey_ready : std_logic;
  signal data_src_lineitem__l_suppkey_data : std_logic_vector(31 downto 0);
  signal data_src_lineitem__l_suppkey_last : std_logic;
  signal data_src_lineitem__l_suppkey_strb : std_logic;
  signal data_src_lineitem__l_tax_valid : std_logic;
  signal data_src_lineitem__l_tax_ready : std_logic;
  signal data_src_lineitem__l_tax_data : std_logic_vector(49 downto 0);
  signal data_src_lineitem__l_tax_last : std_logic;
  signal data_src_lineitem__l_tax_strb : std_logic;
  signal data_src_orders__o_clerk_valid : std_logic;
  signal data_src_orders__o_clerk_ready : std_logic;
  signal data_src_orders__o_clerk_data : std_logic_vector(119 downto 0);
  signal data_src_orders__o_clerk_last : std_logic;
  signal data_src_orders__o_clerk_strb : std_logic;
  signal data_src_orders__o_comment_valid : std_logic;
  signal data_src_orders__o_comment_ready : std_logic;
  signal data_src_orders__o_comment_data : std_logic_vector(7 downto 0);
  signal data_src_orders__o_comment_last : std_logic_vector(1 downto 0);
  signal data_src_orders__o_comment_strb : std_logic;
  signal data_src_orders__o_custkey_valid : std_logic;
  signal data_src_orders__o_custkey_ready : std_logic;
  signal data_src_orders__o_custkey_data : std_logic_vector(31 downto 0);
  signal data_src_orders__o_custkey_last : std_logic;
  signal data_src_orders__o_custkey_strb : std_logic;
  signal data_src_orders__o_orderdate_valid : std_logic;
  signal data_src_orders__o_orderdate_ready : std_logic;
  signal data_src_orders__o_orderdate_data : std_logic_vector(25 downto 0);
  signal data_src_orders__o_orderdate_last : std_logic;
  signal data_src_orders__o_orderdate_strb : std_logic;
  signal data_src_orders__o_orderkey_valid : std_logic;
  signal data_src_orders__o_orderkey_ready : std_logic;
  signal data_src_orders__o_orderkey_data : std_logic_vector(31 downto 0);
  signal data_src_orders__o_orderkey_last : std_logic;
  signal data_src_orders__o_orderkey_strb : std_logic;
  signal data_src_orders__o_orderpriority_valid : std_logic;
  signal data_src_orders__o_orderpriority_ready : std_logic;
  signal data_src_orders__o_orderpriority_data : std_logic_vector(119 downto 0);
  signal data_src_orders__o_orderpriority_last : std_logic;
  signal data_src_orders__o_orderpriority_strb : std_logic;
  signal data_src_orders__o_orderstatus_valid : std_logic;
  signal data_src_orders__o_orderstatus_ready : std_logic;
  signal data_src_orders__o_orderstatus_data : std_logic_vector(7 downto 0);
  signal data_src_orders__o_orderstatus_last : std_logic;
  signal data_src_orders__o_orderstatus_strb : std_logic;
  signal data_src_orders__o_shippriority_valid : std_logic;
  signal data_src_orders__o_shippriority_ready : std_logic;
  signal data_src_orders__o_shippriority_data : std_logic_vector(31 downto 0);
  signal data_src_orders__o_shippriority_last : std_logic;
  signal data_src_orders__o_shippriority_strb : std_logic;
  signal data_src_orders__o_totalprice_valid : std_logic;
  signal data_src_orders__o_totalprice_ready : std_logic;
  signal data_src_orders__o_totalprice_data : std_logic_vector(49 downto 0);
  signal data_src_orders__o_totalprice_last : std_logic;
  signal data_src_orders__o_totalprice_strb : std_logic;
  signal duplicate_c_custkey__input_valid : std_logic;
  signal duplicate_c_custkey__input_ready : std_logic;
  signal duplicate_c_custkey__input_data : std_logic_vector(31 downto 0);
  signal duplicate_c_custkey__input_last : std_logic;
  signal duplicate_c_custkey__input_strb : std_logic;
  signal duplicate_c_custkey__outputAT0_valid : std_logic;
  signal duplicate_c_custkey__outputAT0_ready : std_logic;
  signal duplicate_c_custkey__outputAT0_data : std_logic_vector(31 downto 0);
  signal duplicate_c_custkey__outputAT0_last : std_logic;
  signal duplicate_c_custkey__outputAT0_strb : std_logic;
  signal duplicate_c_custkey__outputAT1_valid : std_logic;
  signal duplicate_c_custkey__outputAT1_ready : std_logic;
  signal duplicate_c_custkey__outputAT1_data : std_logic_vector(31 downto 0);
  signal duplicate_c_custkey__outputAT1_last : std_logic;
  signal duplicate_c_custkey__outputAT1_strb : std_logic;
  signal duplicate_data_src_orders_o_orderdate__input_valid : std_logic;
  signal duplicate_data_src_orders_o_orderdate__input_ready : std_logic;
  signal duplicate_data_src_orders_o_orderdate__input_data : std_logic_vector(25 downto 0);
  signal duplicate_data_src_orders_o_orderdate__input_last : std_logic;
  signal duplicate_data_src_orders_o_orderdate__input_strb : std_logic;
  signal duplicate_data_src_orders_o_orderdate__outputAT0_valid : std_logic;
  signal duplicate_data_src_orders_o_orderdate__outputAT0_ready : std_logic;
  signal duplicate_data_src_orders_o_orderdate__outputAT0_data : std_logic_vector(25 downto 0);
  signal duplicate_data_src_orders_o_orderdate__outputAT0_last : std_logic;
  signal duplicate_data_src_orders_o_orderdate__outputAT0_strb : std_logic;
  signal duplicate_data_src_orders_o_orderdate__outputAT1_valid : std_logic;
  signal duplicate_data_src_orders_o_orderdate__outputAT1_ready : std_logic;
  signal duplicate_data_src_orders_o_orderdate__outputAT1_data : std_logic_vector(25 downto 0);
  signal duplicate_data_src_orders_o_orderdate__outputAT1_last : std_logic;
  signal duplicate_data_src_orders_o_orderdate__outputAT1_strb : std_logic;
  signal duplicate_l_orderkey__input_valid : std_logic;
  signal duplicate_l_orderkey__input_ready : std_logic;
  signal duplicate_l_orderkey__input_data : std_logic_vector(31 downto 0);
  signal duplicate_l_orderkey__input_last : std_logic;
  signal duplicate_l_orderkey__input_strb : std_logic;
  signal duplicate_l_orderkey__outputAT0_valid : std_logic;
  signal duplicate_l_orderkey__outputAT0_ready : std_logic;
  signal duplicate_l_orderkey__outputAT0_data : std_logic_vector(31 downto 0);
  signal duplicate_l_orderkey__outputAT0_last : std_logic;
  signal duplicate_l_orderkey__outputAT0_strb : std_logic;
  signal duplicate_l_orderkey__outputAT1_valid : std_logic;
  signal duplicate_l_orderkey__outputAT1_ready : std_logic;
  signal duplicate_l_orderkey__outputAT1_data : std_logic_vector(31 downto 0);
  signal duplicate_l_orderkey__outputAT1_last : std_logic;
  signal duplicate_l_orderkey__outputAT1_strb : std_logic;
  signal duplicate_l_orderkey__outputAT2_valid : std_logic;
  signal duplicate_l_orderkey__outputAT2_ready : std_logic;
  signal duplicate_l_orderkey__outputAT2_data : std_logic_vector(31 downto 0);
  signal duplicate_l_orderkey__outputAT2_last : std_logic;
  signal duplicate_l_orderkey__outputAT2_strb : std_logic;
  signal duplicate_o_orderkey__input_valid : std_logic;
  signal duplicate_o_orderkey__input_ready : std_logic;
  signal duplicate_o_orderkey__input_data : std_logic_vector(31 downto 0);
  signal duplicate_o_orderkey__input_last : std_logic;
  signal duplicate_o_orderkey__input_strb : std_logic;
  signal duplicate_o_orderkey__outputAT0_valid : std_logic;
  signal duplicate_o_orderkey__outputAT0_ready : std_logic;
  signal duplicate_o_orderkey__outputAT0_data : std_logic_vector(31 downto 0);
  signal duplicate_o_orderkey__outputAT0_last : std_logic;
  signal duplicate_o_orderkey__outputAT0_strb : std_logic;
  signal duplicate_o_orderkey__outputAT1_valid : std_logic;
  signal duplicate_o_orderkey__outputAT1_ready : std_logic;
  signal duplicate_o_orderkey__outputAT1_data : std_logic_vector(31 downto 0);
  signal duplicate_o_orderkey__outputAT1_last : std_logic;
  signal duplicate_o_orderkey__outputAT1_strb : std_logic;
  signal err_and__inputAT0_valid : std_logic;
  signal err_and__inputAT0_ready : std_logic;
  signal err_and__inputAT0_data : std_logic;
  signal err_and__inputAT0_strb : std_logic;
  signal err_and__inputAT1_valid : std_logic;
  signal err_and__inputAT1_ready : std_logic;
  signal err_and__inputAT1_data : std_logic;
  signal err_and__inputAT1_strb : std_logic;
  signal err_and__inputAT2_valid : std_logic;
  signal err_and__inputAT2_ready : std_logic;
  signal err_and__inputAT2_data : std_logic;
  signal err_and__inputAT2_strb : std_logic;
  signal err_and__output_valid : std_logic;
  signal err_and__output_ready : std_logic;
  signal err_and__output_data : std_logic;
  signal err_and__output_strb : std_logic;
  signal multiplier__input0_valid : std_logic;
  signal multiplier__input0_ready : std_logic;
  signal multiplier__input0_data : std_logic_vector(49 downto 0);
  signal multiplier__input0_last : std_logic;
  signal multiplier__input0_strb : std_logic;
  signal multiplier__input1_valid : std_logic;
  signal multiplier__input1_ready : std_logic;
  signal multiplier__input1_data : std_logic_vector(49 downto 0);
  signal multiplier__input1_last : std_logic;
  signal multiplier__input1_strb : std_logic;
  signal multiplier__output_valid : std_logic;
  signal multiplier__output_ready : std_logic;
  signal multiplier__output_data : std_logic_vector(49 downto 0);
  signal multiplier__output_last : std_logic;
  signal multiplier__output_strb : std_logic;
  signal multiplier__overflow_valid : std_logic;
  signal multiplier__overflow_ready : std_logic;
  signal multiplier__overflow_data : std_logic;
  signal multiplier__overflow_strb : std_logic;
  signal to_neg__input_valid : std_logic;
  signal to_neg__input_ready : std_logic;
  signal to_neg__input_data : std_logic_vector(49 downto 0);
  signal to_neg__input_last : std_logic;
  signal to_neg__input_strb : std_logic;
  signal to_neg__output_valid : std_logic;
  signal to_neg__output_ready : std_logic;
  signal to_neg__output_data : std_logic_vector(49 downto 0);
  signal to_neg__output_last : std_logic;
  signal to_neg__output_strb : std_logic;
  signal void_accu_count__input_valid : std_logic;
  signal void_accu_count__input_ready : std_logic;
  signal void_accu_count__input_data : std_logic_vector(31 downto 0);
  signal void_accu_count__input_strb : std_logic;
  signal void_data_src_customer_c_acctbal__input_valid : std_logic;
  signal void_data_src_customer_c_acctbal__input_ready : std_logic;
  signal void_data_src_customer_c_acctbal__input_data : std_logic_vector(49 downto 0);
  signal void_data_src_customer_c_acctbal__input_last : std_logic;
  signal void_data_src_customer_c_acctbal__input_strb : std_logic;
  signal void_data_src_customer_c_address__input_valid : std_logic;
  signal void_data_src_customer_c_address__input_ready : std_logic;
  signal void_data_src_customer_c_address__input_data : std_logic_vector(7 downto 0);
  signal void_data_src_customer_c_address__input_last : std_logic_vector(1 downto 0);
  signal void_data_src_customer_c_address__input_strb : std_logic;
  signal void_data_src_customer_c_comment__input_valid : std_logic;
  signal void_data_src_customer_c_comment__input_ready : std_logic;
  signal void_data_src_customer_c_comment__input_data : std_logic_vector(7 downto 0);
  signal void_data_src_customer_c_comment__input_last : std_logic_vector(1 downto 0);
  signal void_data_src_customer_c_comment__input_strb : std_logic;
  signal void_data_src_customer_c_name__input_valid : std_logic;
  signal void_data_src_customer_c_name__input_ready : std_logic;
  signal void_data_src_customer_c_name__input_data : std_logic_vector(7 downto 0);
  signal void_data_src_customer_c_name__input_last : std_logic_vector(1 downto 0);
  signal void_data_src_customer_c_name__input_strb : std_logic;
  signal void_data_src_customer_c_nationkey__input_valid : std_logic;
  signal void_data_src_customer_c_nationkey__input_ready : std_logic;
  signal void_data_src_customer_c_nationkey__input_data : std_logic_vector(31 downto 0);
  signal void_data_src_customer_c_nationkey__input_last : std_logic;
  signal void_data_src_customer_c_nationkey__input_strb : std_logic;
  signal void_data_src_customer_c_phone__input_valid : std_logic;
  signal void_data_src_customer_c_phone__input_ready : std_logic;
  signal void_data_src_customer_c_phone__input_data : std_logic_vector(119 downto 0);
  signal void_data_src_customer_c_phone__input_last : std_logic;
  signal void_data_src_customer_c_phone__input_strb : std_logic;
  signal void_data_src_lineitem_l_comment__input_valid : std_logic;
  signal void_data_src_lineitem_l_comment__input_ready : std_logic;
  signal void_data_src_lineitem_l_comment__input_data : std_logic_vector(7 downto 0);
  signal void_data_src_lineitem_l_comment__input_last : std_logic_vector(1 downto 0);
  signal void_data_src_lineitem_l_comment__input_strb : std_logic;
  signal void_data_src_lineitem_l_commitdate__input_valid : std_logic;
  signal void_data_src_lineitem_l_commitdate__input_ready : std_logic;
  signal void_data_src_lineitem_l_commitdate__input_data : std_logic_vector(25 downto 0);
  signal void_data_src_lineitem_l_commitdate__input_last : std_logic;
  signal void_data_src_lineitem_l_commitdate__input_strb : std_logic;
  signal void_data_src_lineitem_l_linestatus__input_valid : std_logic;
  signal void_data_src_lineitem_l_linestatus__input_ready : std_logic;
  signal void_data_src_lineitem_l_linestatus__input_data : std_logic_vector(7 downto 0);
  signal void_data_src_lineitem_l_linestatus__input_last : std_logic;
  signal void_data_src_lineitem_l_linestatus__input_strb : std_logic;
  signal void_data_src_lineitem_l_partkey__input_valid : std_logic;
  signal void_data_src_lineitem_l_partkey__input_ready : std_logic;
  signal void_data_src_lineitem_l_partkey__input_data : std_logic_vector(31 downto 0);
  signal void_data_src_lineitem_l_partkey__input_last : std_logic;
  signal void_data_src_lineitem_l_partkey__input_strb : std_logic;
  signal void_data_src_lineitem_l_quantity__input_valid : std_logic;
  signal void_data_src_lineitem_l_quantity__input_ready : std_logic;
  signal void_data_src_lineitem_l_quantity__input_data : std_logic_vector(49 downto 0);
  signal void_data_src_lineitem_l_quantity__input_last : std_logic;
  signal void_data_src_lineitem_l_quantity__input_strb : std_logic;
  signal void_data_src_lineitem_l_receiptdate__input_valid : std_logic;
  signal void_data_src_lineitem_l_receiptdate__input_ready : std_logic;
  signal void_data_src_lineitem_l_receiptdate__input_data : std_logic_vector(25 downto 0);
  signal void_data_src_lineitem_l_receiptdate__input_last : std_logic;
  signal void_data_src_lineitem_l_receiptdate__input_strb : std_logic;
  signal void_data_src_lineitem_l_returnflag__input_valid : std_logic;
  signal void_data_src_lineitem_l_returnflag__input_ready : std_logic;
  signal void_data_src_lineitem_l_returnflag__input_data : std_logic_vector(7 downto 0);
  signal void_data_src_lineitem_l_returnflag__input_last : std_logic;
  signal void_data_src_lineitem_l_returnflag__input_strb : std_logic;
  signal void_data_src_lineitem_l_shipinstruct__input_valid : std_logic;
  signal void_data_src_lineitem_l_shipinstruct__input_ready : std_logic;
  signal void_data_src_lineitem_l_shipinstruct__input_data : std_logic_vector(199 downto 0);
  signal void_data_src_lineitem_l_shipinstruct__input_last : std_logic;
  signal void_data_src_lineitem_l_shipinstruct__input_strb : std_logic;
  signal void_data_src_lineitem_l_shipmode__input_valid : std_logic;
  signal void_data_src_lineitem_l_shipmode__input_ready : std_logic;
  signal void_data_src_lineitem_l_shipmode__input_data : std_logic_vector(79 downto 0);
  signal void_data_src_lineitem_l_shipmode__input_last : std_logic;
  signal void_data_src_lineitem_l_shipmode__input_strb : std_logic;
  signal void_data_src_lineitem_l_suppkey__input_valid : std_logic;
  signal void_data_src_lineitem_l_suppkey__input_ready : std_logic;
  signal void_data_src_lineitem_l_suppkey__input_data : std_logic_vector(31 downto 0);
  signal void_data_src_lineitem_l_suppkey__input_last : std_logic;
  signal void_data_src_lineitem_l_suppkey__input_strb : std_logic;
  signal void_data_src_lineitem_l_tax__input_valid : std_logic;
  signal void_data_src_lineitem_l_tax__input_ready : std_logic;
  signal void_data_src_lineitem_l_tax__input_data : std_logic_vector(49 downto 0);
  signal void_data_src_lineitem_l_tax__input_last : std_logic;
  signal void_data_src_lineitem_l_tax__input_strb : std_logic;
  signal void_data_src_orders_o_clerk__input_valid : std_logic;
  signal void_data_src_orders_o_clerk__input_ready : std_logic;
  signal void_data_src_orders_o_clerk__input_data : std_logic_vector(119 downto 0);
  signal void_data_src_orders_o_clerk__input_last : std_logic;
  signal void_data_src_orders_o_clerk__input_strb : std_logic;
  signal void_data_src_orders_o_comment__input_valid : std_logic;
  signal void_data_src_orders_o_comment__input_ready : std_logic;
  signal void_data_src_orders_o_comment__input_data : std_logic_vector(7 downto 0);
  signal void_data_src_orders_o_comment__input_last : std_logic_vector(1 downto 0);
  signal void_data_src_orders_o_comment__input_strb : std_logic;
  signal void_data_src_orders_o_orderpriority__input_valid : std_logic;
  signal void_data_src_orders_o_orderpriority__input_ready : std_logic;
  signal void_data_src_orders_o_orderpriority__input_data : std_logic_vector(119 downto 0);
  signal void_data_src_orders_o_orderpriority__input_last : std_logic;
  signal void_data_src_orders_o_orderpriority__input_strb : std_logic;
  signal void_data_src_orders_o_orderstatus__input_valid : std_logic;
  signal void_data_src_orders_o_orderstatus__input_ready : std_logic;
  signal void_data_src_orders_o_orderstatus__input_data : std_logic_vector(7 downto 0);
  signal void_data_src_orders_o_orderstatus__input_last : std_logic;
  signal void_data_src_orders_o_orderstatus__input_strb : std_logic;
  signal void_data_src_orders_o_totalprice__input_valid : std_logic;
  signal void_data_src_orders_o_totalprice__input_ready : std_logic;
  signal void_data_src_orders_o_totalprice__input_data : std_logic_vector(49 downto 0);
  signal void_data_src_orders_o_totalprice__input_last : std_logic;
  signal void_data_src_orders_o_totalprice__input_strb : std_logic;
begin
  accu: test_project__tpch__accumulator_iATStreamIWSQL_decimal_15_2_streamIM_com port map(
    clk => clk,
    rst => rst,
    input_valid => accu__input_valid,
    input_ready => accu__input_ready,
    input_data => accu__input_data,
    input_last => accu__input_last,
    input_strb => accu__input_strb,
    count_valid => accu__count_valid,
    count_ready => accu__count_ready,
    count_data => accu__count_data,
    count_strb => accu__count_strb,
    output_valid => accu__output_valid,
    output_ready => accu__output_ready,
    output_data => accu__output_data,
    output_last => accu__output_last,
    output_strb => accu__output_strb,
    overflow_valid => accu__overflow_valid,
    overflow_ready => accu__overflow_ready,
    overflow_data => accu__overflow_data,
    overflow_strb => accu__overflow_strb
  );
  adder: test_project__tpch__adder_iATStreamIWSQL_decimal_15_2_streamIM_com port map(
    clk => clk,
    rst => rst,
    input1_valid => adder__input1_valid,
    input1_ready => adder__input1_ready,
    input1_data => adder__input1_data,
    input1_last => adder__input1_last,
    input1_strb => adder__input1_strb,
    input0_valid => adder__input0_valid,
    input0_ready => adder__input0_ready,
    input0_data => adder__input0_data,
    input0_last => adder__input0_last,
    input0_strb => adder__input0_strb,
    overflow_valid => adder__overflow_valid,
    overflow_ready => adder__overflow_ready,
    overflow_data => adder__overflow_data,
    overflow_strb => adder__overflow_strb,
    output_valid => adder__output_valid,
    output_ready => adder__output_ready,
    output_data => adder__output_data,
    output_last => adder__output_last,
    output_strb => adder__output_strb
  );
  const_value_gen: test_project__tpch__const_value_generator_iATStreamIWSQL_decimal_15_2_streamIMAT1_com port map(
    clk => clk,
    rst => rst,
    output_valid => const_value_gen__output_valid,
    output_ready => const_value_gen__output_ready,
    output_data => const_value_gen__output_data,
    output_last => const_value_gen__output_last,
    output_strb => const_value_gen__output_strb
  );
  data_filter: test_project__tpch__data_filter_i_com port map(
    clk => clk,
    rst => rst,
    l_orderkey_in_valid => data_filter__l_orderkey_in_valid,
    l_orderkey_in_ready => data_filter__l_orderkey_in_ready,
    l_orderkey_in_data => data_filter__l_orderkey_in_data,
    l_orderkey_in_last => data_filter__l_orderkey_in_last,
    l_orderkey_in_strb => data_filter__l_orderkey_in_strb,
    o_orderdate_in_valid => data_filter__o_orderdate_in_valid,
    o_orderdate_in_ready => data_filter__o_orderdate_in_ready,
    o_orderdate_in_data => data_filter__o_orderdate_in_data,
    o_orderdate_in_last => data_filter__o_orderdate_in_last,
    o_orderdate_in_strb => data_filter__o_orderdate_in_strb,
    l_orderkey_valid => data_filter__l_orderkey_valid,
    l_orderkey_ready => data_filter__l_orderkey_ready,
    l_orderkey_data => data_filter__l_orderkey_data,
    l_orderkey_last => data_filter__l_orderkey_last,
    l_orderkey_strb => data_filter__l_orderkey_strb,
    o_orderkey_valid => data_filter__o_orderkey_valid,
    o_orderkey_ready => data_filter__o_orderkey_ready,
    o_orderkey_data => data_filter__o_orderkey_data,
    o_orderkey_last => data_filter__o_orderkey_last,
    o_orderkey_strb => data_filter__o_orderkey_strb,
    o_custkey_valid => data_filter__o_custkey_valid,
    o_custkey_ready => data_filter__o_custkey_ready,
    o_custkey_data => data_filter__o_custkey_data,
    o_custkey_last => data_filter__o_custkey_last,
    o_custkey_strb => data_filter__o_custkey_strb,
    c_custkey_valid => data_filter__c_custkey_valid,
    c_custkey_ready => data_filter__c_custkey_ready,
    c_custkey_data => data_filter__c_custkey_data,
    c_custkey_last => data_filter__c_custkey_last,
    c_custkey_strb => data_filter__c_custkey_strb,
    l_extendedprice_in_valid => data_filter__l_extendedprice_in_valid,
    l_extendedprice_in_ready => data_filter__l_extendedprice_in_ready,
    l_extendedprice_in_data => data_filter__l_extendedprice_in_data,
    l_extendedprice_in_last => data_filter__l_extendedprice_in_last,
    l_extendedprice_in_strb => data_filter__l_extendedprice_in_strb,
    l_shipdate_valid => data_filter__l_shipdate_valid,
    l_shipdate_ready => data_filter__l_shipdate_ready,
    l_shipdate_data => data_filter__l_shipdate_data,
    l_shipdate_last => data_filter__l_shipdate_last,
    l_shipdate_strb => data_filter__l_shipdate_strb,
    o_shippriority_in_valid => data_filter__o_shippriority_in_valid,
    o_shippriority_in_ready => data_filter__o_shippriority_in_ready,
    o_shippriority_in_data => data_filter__o_shippriority_in_data,
    o_shippriority_in_last => data_filter__o_shippriority_in_last,
    o_shippriority_in_strb => data_filter__o_shippriority_in_strb,
    c_mktsegment_valid => data_filter__c_mktsegment_valid,
    c_mktsegment_ready => data_filter__c_mktsegment_ready,
    c_mktsegment_data => data_filter__c_mktsegment_data,
    c_mktsegment_last => data_filter__c_mktsegment_last,
    c_mktsegment_strb => data_filter__c_mktsegment_strb,
    l_discount_in_valid => data_filter__l_discount_in_valid,
    l_discount_in_ready => data_filter__l_discount_in_ready,
    l_discount_in_data => data_filter__l_discount_in_data,
    l_discount_in_last => data_filter__l_discount_in_last,
    l_discount_in_strb => data_filter__l_discount_in_strb,
    o_orderdate_valid => data_filter__o_orderdate_valid,
    o_orderdate_ready => data_filter__o_orderdate_ready,
    o_orderdate_data => data_filter__o_orderdate_data,
    o_orderdate_last => data_filter__o_orderdate_last,
    o_orderdate_strb => data_filter__o_orderdate_strb,
    o_orderdate_out_valid => data_filter__o_orderdate_out_valid,
    o_orderdate_out_ready => data_filter__o_orderdate_out_ready,
    o_orderdate_out_data => data_filter__o_orderdate_out_data,
    o_orderdate_out_last => data_filter__o_orderdate_out_last,
    o_orderdate_out_strb => data_filter__o_orderdate_out_strb,
    l_extendedprice_out_valid => data_filter__l_extendedprice_out_valid,
    l_extendedprice_out_ready => data_filter__l_extendedprice_out_ready,
    l_extendedprice_out_data => data_filter__l_extendedprice_out_data,
    l_extendedprice_out_last => data_filter__l_extendedprice_out_last,
    l_extendedprice_out_strb => data_filter__l_extendedprice_out_strb,
    l_discount_out_valid => data_filter__l_discount_out_valid,
    l_discount_out_ready => data_filter__l_discount_out_ready,
    l_discount_out_data => data_filter__l_discount_out_data,
    l_discount_out_last => data_filter__l_discount_out_last,
    l_discount_out_strb => data_filter__l_discount_out_strb,
    l_orderkey_out_valid => data_filter__l_orderkey_out_valid,
    l_orderkey_out_ready => data_filter__l_orderkey_out_ready,
    l_orderkey_out_data => data_filter__l_orderkey_out_data,
    l_orderkey_out_last => data_filter__l_orderkey_out_last,
    l_orderkey_out_strb => data_filter__l_orderkey_out_strb,
    o_shippriority_out_valid => data_filter__o_shippriority_out_valid,
    o_shippriority_out_ready => data_filter__o_shippriority_out_ready,
    o_shippriority_out_data => data_filter__o_shippriority_out_data,
    o_shippriority_out_last => data_filter__o_shippriority_out_last,
    o_shippriority_out_strb => data_filter__o_shippriority_out_strb
  );
  data_src_customer: test_project__tpch__customer_i_com port map(
    clk => clk,
    rst => rst,
    c_custkey_valid => data_src_customer__c_custkey_valid,
    c_custkey_ready => data_src_customer__c_custkey_ready,
    c_custkey_data => data_src_customer__c_custkey_data,
    c_custkey_last => data_src_customer__c_custkey_last,
    c_custkey_strb => data_src_customer__c_custkey_strb,
    c_nationkey_valid => data_src_customer__c_nationkey_valid,
    c_nationkey_ready => data_src_customer__c_nationkey_ready,
    c_nationkey_data => data_src_customer__c_nationkey_data,
    c_nationkey_last => data_src_customer__c_nationkey_last,
    c_nationkey_strb => data_src_customer__c_nationkey_strb,
    c_comment_valid => data_src_customer__c_comment_valid,
    c_comment_ready => data_src_customer__c_comment_ready,
    c_comment_data => data_src_customer__c_comment_data,
    c_comment_last => data_src_customer__c_comment_last,
    c_comment_strb => data_src_customer__c_comment_strb,
    c_mktsegment_valid => data_src_customer__c_mktsegment_valid,
    c_mktsegment_ready => data_src_customer__c_mktsegment_ready,
    c_mktsegment_data => data_src_customer__c_mktsegment_data,
    c_mktsegment_last => data_src_customer__c_mktsegment_last,
    c_mktsegment_strb => data_src_customer__c_mktsegment_strb,
    c_address_valid => data_src_customer__c_address_valid,
    c_address_ready => data_src_customer__c_address_ready,
    c_address_data => data_src_customer__c_address_data,
    c_address_last => data_src_customer__c_address_last,
    c_address_strb => data_src_customer__c_address_strb,
    c_phone_valid => data_src_customer__c_phone_valid,
    c_phone_ready => data_src_customer__c_phone_ready,
    c_phone_data => data_src_customer__c_phone_data,
    c_phone_last => data_src_customer__c_phone_last,
    c_phone_strb => data_src_customer__c_phone_strb,
    c_name_valid => data_src_customer__c_name_valid,
    c_name_ready => data_src_customer__c_name_ready,
    c_name_data => data_src_customer__c_name_data,
    c_name_last => data_src_customer__c_name_last,
    c_name_strb => data_src_customer__c_name_strb,
    c_acctbal_valid => data_src_customer__c_acctbal_valid,
    c_acctbal_ready => data_src_customer__c_acctbal_ready,
    c_acctbal_data => data_src_customer__c_acctbal_data,
    c_acctbal_last => data_src_customer__c_acctbal_last,
    c_acctbal_strb => data_src_customer__c_acctbal_strb
  );
  data_src_lineitem: test_project__tpch__lineitem_i_com port map(
    clk => clk,
    rst => rst,
    l_orderkey_valid => data_src_lineitem__l_orderkey_valid,
    l_orderkey_ready => data_src_lineitem__l_orderkey_ready,
    l_orderkey_data => data_src_lineitem__l_orderkey_data,
    l_orderkey_last => data_src_lineitem__l_orderkey_last,
    l_orderkey_strb => data_src_lineitem__l_orderkey_strb,
    l_linenumber_valid => data_src_lineitem__l_linenumber_valid,
    l_linenumber_ready => data_src_lineitem__l_linenumber_ready,
    l_linenumber_data => data_src_lineitem__l_linenumber_data,
    l_linenumber_last => data_src_lineitem__l_linenumber_last,
    l_linenumber_strb => data_src_lineitem__l_linenumber_strb,
    l_returnflag_valid => data_src_lineitem__l_returnflag_valid,
    l_returnflag_ready => data_src_lineitem__l_returnflag_ready,
    l_returnflag_data => data_src_lineitem__l_returnflag_data,
    l_returnflag_last => data_src_lineitem__l_returnflag_last,
    l_returnflag_strb => data_src_lineitem__l_returnflag_strb,
    l_suppkey_valid => data_src_lineitem__l_suppkey_valid,
    l_suppkey_ready => data_src_lineitem__l_suppkey_ready,
    l_suppkey_data => data_src_lineitem__l_suppkey_data,
    l_suppkey_last => data_src_lineitem__l_suppkey_last,
    l_suppkey_strb => data_src_lineitem__l_suppkey_strb,
    l_shipdate_valid => data_src_lineitem__l_shipdate_valid,
    l_shipdate_ready => data_src_lineitem__l_shipdate_ready,
    l_shipdate_data => data_src_lineitem__l_shipdate_data,
    l_shipdate_last => data_src_lineitem__l_shipdate_last,
    l_shipdate_strb => data_src_lineitem__l_shipdate_strb,
    l_tax_valid => data_src_lineitem__l_tax_valid,
    l_tax_ready => data_src_lineitem__l_tax_ready,
    l_tax_data => data_src_lineitem__l_tax_data,
    l_tax_last => data_src_lineitem__l_tax_last,
    l_tax_strb => data_src_lineitem__l_tax_strb,
    l_comment_valid => data_src_lineitem__l_comment_valid,
    l_comment_ready => data_src_lineitem__l_comment_ready,
    l_comment_data => data_src_lineitem__l_comment_data,
    l_comment_last => data_src_lineitem__l_comment_last,
    l_comment_strb => data_src_lineitem__l_comment_strb,
    l_linestatus_valid => data_src_lineitem__l_linestatus_valid,
    l_linestatus_ready => data_src_lineitem__l_linestatus_ready,
    l_linestatus_data => data_src_lineitem__l_linestatus_data,
    l_linestatus_last => data_src_lineitem__l_linestatus_last,
    l_linestatus_strb => data_src_lineitem__l_linestatus_strb,
    l_quantity_valid => data_src_lineitem__l_quantity_valid,
    l_quantity_ready => data_src_lineitem__l_quantity_ready,
    l_quantity_data => data_src_lineitem__l_quantity_data,
    l_quantity_last => data_src_lineitem__l_quantity_last,
    l_quantity_strb => data_src_lineitem__l_quantity_strb,
    l_extendedprice_valid => data_src_lineitem__l_extendedprice_valid,
    l_extendedprice_ready => data_src_lineitem__l_extendedprice_ready,
    l_extendedprice_data => data_src_lineitem__l_extendedprice_data,
    l_extendedprice_last => data_src_lineitem__l_extendedprice_last,
    l_extendedprice_strb => data_src_lineitem__l_extendedprice_strb,
    l_commitdate_valid => data_src_lineitem__l_commitdate_valid,
    l_commitdate_ready => data_src_lineitem__l_commitdate_ready,
    l_commitdate_data => data_src_lineitem__l_commitdate_data,
    l_commitdate_last => data_src_lineitem__l_commitdate_last,
    l_commitdate_strb => data_src_lineitem__l_commitdate_strb,
    l_receiptdate_valid => data_src_lineitem__l_receiptdate_valid,
    l_receiptdate_ready => data_src_lineitem__l_receiptdate_ready,
    l_receiptdate_data => data_src_lineitem__l_receiptdate_data,
    l_receiptdate_last => data_src_lineitem__l_receiptdate_last,
    l_receiptdate_strb => data_src_lineitem__l_receiptdate_strb,
    l_shipinstruct_valid => data_src_lineitem__l_shipinstruct_valid,
    l_shipinstruct_ready => data_src_lineitem__l_shipinstruct_ready,
    l_shipinstruct_data => data_src_lineitem__l_shipinstruct_data,
    l_shipinstruct_last => data_src_lineitem__l_shipinstruct_last,
    l_shipinstruct_strb => data_src_lineitem__l_shipinstruct_strb,
    l_discount_valid => data_src_lineitem__l_discount_valid,
    l_discount_ready => data_src_lineitem__l_discount_ready,
    l_discount_data => data_src_lineitem__l_discount_data,
    l_discount_last => data_src_lineitem__l_discount_last,
    l_discount_strb => data_src_lineitem__l_discount_strb,
    l_partkey_valid => data_src_lineitem__l_partkey_valid,
    l_partkey_ready => data_src_lineitem__l_partkey_ready,
    l_partkey_data => data_src_lineitem__l_partkey_data,
    l_partkey_last => data_src_lineitem__l_partkey_last,
    l_partkey_strb => data_src_lineitem__l_partkey_strb,
    l_shipmode_valid => data_src_lineitem__l_shipmode_valid,
    l_shipmode_ready => data_src_lineitem__l_shipmode_ready,
    l_shipmode_data => data_src_lineitem__l_shipmode_data,
    l_shipmode_last => data_src_lineitem__l_shipmode_last,
    l_shipmode_strb => data_src_lineitem__l_shipmode_strb
  );
  data_src_orders: test_project__tpch__orders_i_com port map(
    clk => clk,
    rst => rst,
    o_orderkey_valid => data_src_orders__o_orderkey_valid,
    o_orderkey_ready => data_src_orders__o_orderkey_ready,
    o_orderkey_data => data_src_orders__o_orderkey_data,
    o_orderkey_last => data_src_orders__o_orderkey_last,
    o_orderkey_strb => data_src_orders__o_orderkey_strb,
    o_custkey_valid => data_src_orders__o_custkey_valid,
    o_custkey_ready => data_src_orders__o_custkey_ready,
    o_custkey_data => data_src_orders__o_custkey_data,
    o_custkey_last => data_src_orders__o_custkey_last,
    o_custkey_strb => data_src_orders__o_custkey_strb,
    o_orderdate_valid => data_src_orders__o_orderdate_valid,
    o_orderdate_ready => data_src_orders__o_orderdate_ready,
    o_orderdate_data => data_src_orders__o_orderdate_data,
    o_orderdate_last => data_src_orders__o_orderdate_last,
    o_orderdate_strb => data_src_orders__o_orderdate_strb,
    o_totalprice_valid => data_src_orders__o_totalprice_valid,
    o_totalprice_ready => data_src_orders__o_totalprice_ready,
    o_totalprice_data => data_src_orders__o_totalprice_data,
    o_totalprice_last => data_src_orders__o_totalprice_last,
    o_totalprice_strb => data_src_orders__o_totalprice_strb,
    o_shippriority_valid => data_src_orders__o_shippriority_valid,
    o_shippriority_ready => data_src_orders__o_shippriority_ready,
    o_shippriority_data => data_src_orders__o_shippriority_data,
    o_shippriority_last => data_src_orders__o_shippriority_last,
    o_shippriority_strb => data_src_orders__o_shippriority_strb,
    o_comment_valid => data_src_orders__o_comment_valid,
    o_comment_ready => data_src_orders__o_comment_ready,
    o_comment_data => data_src_orders__o_comment_data,
    o_comment_last => data_src_orders__o_comment_last,
    o_comment_strb => data_src_orders__o_comment_strb,
    o_clerk_valid => data_src_orders__o_clerk_valid,
    o_clerk_ready => data_src_orders__o_clerk_ready,
    o_clerk_data => data_src_orders__o_clerk_data,
    o_clerk_last => data_src_orders__o_clerk_last,
    o_clerk_strb => data_src_orders__o_clerk_strb,
    o_orderstatus_valid => data_src_orders__o_orderstatus_valid,
    o_orderstatus_ready => data_src_orders__o_orderstatus_ready,
    o_orderstatus_data => data_src_orders__o_orderstatus_data,
    o_orderstatus_last => data_src_orders__o_orderstatus_last,
    o_orderstatus_strb => data_src_orders__o_orderstatus_strb,
    o_orderpriority_valid => data_src_orders__o_orderpriority_valid,
    o_orderpriority_ready => data_src_orders__o_orderpriority_ready,
    o_orderpriority_data => data_src_orders__o_orderpriority_data,
    o_orderpriority_last => data_src_orders__o_orderpriority_last,
    o_orderpriority_strb => data_src_orders__o_orderpriority_strb
  );
  duplicate_c_custkey: test_project__tpch__duplicator_iATStreamIWint_streamIMAT2_com port map(
    clk => clk,
    rst => rst,
    input_valid => duplicate_c_custkey__input_valid,
    input_ready => duplicate_c_custkey__input_ready,
    input_data => duplicate_c_custkey__input_data,
    input_last => duplicate_c_custkey__input_last,
    input_strb => duplicate_c_custkey__input_strb,
    outputAT1_valid => duplicate_c_custkey__outputAT1_valid,
    outputAT1_ready => duplicate_c_custkey__outputAT1_ready,
    outputAT1_data => duplicate_c_custkey__outputAT1_data,
    outputAT1_last => duplicate_c_custkey__outputAT1_last,
    outputAT1_strb => duplicate_c_custkey__outputAT1_strb,
    outputAT0_valid => duplicate_c_custkey__outputAT0_valid,
    outputAT0_ready => duplicate_c_custkey__outputAT0_ready,
    outputAT0_data => duplicate_c_custkey__outputAT0_data,
    outputAT0_last => duplicate_c_custkey__outputAT0_last,
    outputAT0_strb => duplicate_c_custkey__outputAT0_strb
  );
  duplicate_data_src_orders_o_orderdate: test_project__tpch__duplicator_iATStreamIWdate_streamIMAT2_com port map(
    clk => clk,
    rst => rst,
    input_valid => duplicate_data_src_orders_o_orderdate__input_valid,
    input_ready => duplicate_data_src_orders_o_orderdate__input_ready,
    input_data => duplicate_data_src_orders_o_orderdate__input_data,
    input_last => duplicate_data_src_orders_o_orderdate__input_last,
    input_strb => duplicate_data_src_orders_o_orderdate__input_strb,
    outputAT1_valid => duplicate_data_src_orders_o_orderdate__outputAT1_valid,
    outputAT1_ready => duplicate_data_src_orders_o_orderdate__outputAT1_ready,
    outputAT1_data => duplicate_data_src_orders_o_orderdate__outputAT1_data,
    outputAT1_last => duplicate_data_src_orders_o_orderdate__outputAT1_last,
    outputAT1_strb => duplicate_data_src_orders_o_orderdate__outputAT1_strb,
    outputAT0_valid => duplicate_data_src_orders_o_orderdate__outputAT0_valid,
    outputAT0_ready => duplicate_data_src_orders_o_orderdate__outputAT0_ready,
    outputAT0_data => duplicate_data_src_orders_o_orderdate__outputAT0_data,
    outputAT0_last => duplicate_data_src_orders_o_orderdate__outputAT0_last,
    outputAT0_strb => duplicate_data_src_orders_o_orderdate__outputAT0_strb
  );
  duplicate_l_orderkey: test_project__tpch__duplicator_iATStreamIWint_streamIMAT3_com port map(
    clk => clk,
    rst => rst,
    input_valid => duplicate_l_orderkey__input_valid,
    input_ready => duplicate_l_orderkey__input_ready,
    input_data => duplicate_l_orderkey__input_data,
    input_last => duplicate_l_orderkey__input_last,
    input_strb => duplicate_l_orderkey__input_strb,
    outputAT0_valid => duplicate_l_orderkey__outputAT0_valid,
    outputAT0_ready => duplicate_l_orderkey__outputAT0_ready,
    outputAT0_data => duplicate_l_orderkey__outputAT0_data,
    outputAT0_last => duplicate_l_orderkey__outputAT0_last,
    outputAT0_strb => duplicate_l_orderkey__outputAT0_strb,
    outputAT1_valid => duplicate_l_orderkey__outputAT1_valid,
    outputAT1_ready => duplicate_l_orderkey__outputAT1_ready,
    outputAT1_data => duplicate_l_orderkey__outputAT1_data,
    outputAT1_last => duplicate_l_orderkey__outputAT1_last,
    outputAT1_strb => duplicate_l_orderkey__outputAT1_strb,
    outputAT2_valid => duplicate_l_orderkey__outputAT2_valid,
    outputAT2_ready => duplicate_l_orderkey__outputAT2_ready,
    outputAT2_data => duplicate_l_orderkey__outputAT2_data,
    outputAT2_last => duplicate_l_orderkey__outputAT2_last,
    outputAT2_strb => duplicate_l_orderkey__outputAT2_strb
  );
  duplicate_o_orderkey: test_project__tpch__duplicator_iATStreamIWint_streamIMAT2_com port map(
    clk => clk,
    rst => rst,
    input_valid => duplicate_o_orderkey__input_valid,
    input_ready => duplicate_o_orderkey__input_ready,
    input_data => duplicate_o_orderkey__input_data,
    input_last => duplicate_o_orderkey__input_last,
    input_strb => duplicate_o_orderkey__input_strb,
    outputAT1_valid => duplicate_o_orderkey__outputAT1_valid,
    outputAT1_ready => duplicate_o_orderkey__outputAT1_ready,
    outputAT1_data => duplicate_o_orderkey__outputAT1_data,
    outputAT1_last => duplicate_o_orderkey__outputAT1_last,
    outputAT1_strb => duplicate_o_orderkey__outputAT1_strb,
    outputAT0_valid => duplicate_o_orderkey__outputAT0_valid,
    outputAT0_ready => duplicate_o_orderkey__outputAT0_ready,
    outputAT0_data => duplicate_o_orderkey__outputAT0_data,
    outputAT0_last => duplicate_o_orderkey__outputAT0_last,
    outputAT0_strb => duplicate_o_orderkey__outputAT0_strb
  );
  err_and: test_project__tpch__and_iATStreamIWerr_streamIMAT3_com port map(
    clk => clk,
    rst => rst,
    inputAT1_valid => err_and__inputAT1_valid,
    inputAT1_ready => err_and__inputAT1_ready,
    inputAT1_data => err_and__inputAT1_data,
    inputAT1_strb => err_and__inputAT1_strb,
    inputAT2_valid => err_and__inputAT2_valid,
    inputAT2_ready => err_and__inputAT2_ready,
    inputAT2_data => err_and__inputAT2_data,
    inputAT2_strb => err_and__inputAT2_strb,
    inputAT0_valid => err_and__inputAT0_valid,
    inputAT0_ready => err_and__inputAT0_ready,
    inputAT0_data => err_and__inputAT0_data,
    inputAT0_strb => err_and__inputAT0_strb,
    output_valid => err_and__output_valid,
    output_ready => err_and__output_ready,
    output_data => err_and__output_data,
    output_strb => err_and__output_strb
  );
  multiplier: test_project__tpch__multiplier_iATStreamIWSQL_decimal_15_2_streamIM_com port map(
    clk => clk,
    rst => rst,
    input1_valid => multiplier__input1_valid,
    input1_ready => multiplier__input1_ready,
    input1_data => multiplier__input1_data,
    input1_last => multiplier__input1_last,
    input1_strb => multiplier__input1_strb,
    input0_valid => multiplier__input0_valid,
    input0_ready => multiplier__input0_ready,
    input0_data => multiplier__input0_data,
    input0_last => multiplier__input0_last,
    input0_strb => multiplier__input0_strb,
    output_valid => multiplier__output_valid,
    output_ready => multiplier__output_ready,
    output_data => multiplier__output_data,
    output_last => multiplier__output_last,
    output_strb => multiplier__output_strb,
    overflow_valid => multiplier__overflow_valid,
    overflow_ready => multiplier__overflow_ready,
    overflow_data => multiplier__overflow_data,
    overflow_strb => multiplier__overflow_strb
  );
  to_neg: test_project__tpch__to_neg_iATStreamIWSQL_decimal_15_2_streamIM_com port map(
    clk => clk,
    rst => rst,
    input_valid => to_neg__input_valid,
    input_ready => to_neg__input_ready,
    input_data => to_neg__input_data,
    input_last => to_neg__input_last,
    input_strb => to_neg__input_strb,
    output_valid => to_neg__output_valid,
    output_ready => to_neg__output_ready,
    output_data => to_neg__output_data,
    output_last => to_neg__output_last,
    output_strb => to_neg__output_strb
  );
  void_accu_count: test_project__tpch__void_iATStreamIWcount_typeIM_com port map(
    clk => clk,
    rst => rst,
    input_valid => void_accu_count__input_valid,
    input_ready => void_accu_count__input_ready,
    input_data => void_accu_count__input_data,
    input_strb => void_accu_count__input_strb
  );
  void_data_src_customer_c_acctbal: test_project__tpch__void_iATStreamIWSQL_decimal_15_2_streamIM_com port map(
    clk => clk,
    rst => rst,
    input_valid => void_data_src_customer_c_acctbal__input_valid,
    input_ready => void_data_src_customer_c_acctbal__input_ready,
    input_data => void_data_src_customer_c_acctbal__input_data,
    input_last => void_data_src_customer_c_acctbal__input_last,
    input_strb => void_data_src_customer_c_acctbal__input_strb
  );
  void_data_src_customer_c_address: test_project__tpch__void_iATStreamIWvarchar_streamIM_com port map(
    clk => clk,
    rst => rst,
    input_valid => void_data_src_customer_c_address__input_valid,
    input_ready => void_data_src_customer_c_address__input_ready,
    input_data => void_data_src_customer_c_address__input_data,
    input_last => void_data_src_customer_c_address__input_last,
    input_strb => void_data_src_customer_c_address__input_strb
  );
  void_data_src_customer_c_comment: test_project__tpch__void_iATStreamIWvarchar_streamIM_com port map(
    clk => clk,
    rst => rst,
    input_valid => void_data_src_customer_c_comment__input_valid,
    input_ready => void_data_src_customer_c_comment__input_ready,
    input_data => void_data_src_customer_c_comment__input_data,
    input_last => void_data_src_customer_c_comment__input_last,
    input_strb => void_data_src_customer_c_comment__input_strb
  );
  void_data_src_customer_c_name: test_project__tpch__void_iATStreamIWvarchar_streamIM_com port map(
    clk => clk,
    rst => rst,
    input_valid => void_data_src_customer_c_name__input_valid,
    input_ready => void_data_src_customer_c_name__input_ready,
    input_data => void_data_src_customer_c_name__input_data,
    input_last => void_data_src_customer_c_name__input_last,
    input_strb => void_data_src_customer_c_name__input_strb
  );
  void_data_src_customer_c_nationkey: test_project__tpch__void_iATStreamIWint_streamIM_com port map(
    clk => clk,
    rst => rst,
    input_valid => void_data_src_customer_c_nationkey__input_valid,
    input_ready => void_data_src_customer_c_nationkey__input_ready,
    input_data => void_data_src_customer_c_nationkey__input_data,
    input_last => void_data_src_customer_c_nationkey__input_last,
    input_strb => void_data_src_customer_c_nationkey__input_strb
  );
  void_data_src_customer_c_phone: test_project__tpch__void_iATStreamIWSQL_char15_streamIM_com port map(
    clk => clk,
    rst => rst,
    input_valid => void_data_src_customer_c_phone__input_valid,
    input_ready => void_data_src_customer_c_phone__input_ready,
    input_data => void_data_src_customer_c_phone__input_data,
    input_last => void_data_src_customer_c_phone__input_last,
    input_strb => void_data_src_customer_c_phone__input_strb
  );
  void_data_src_lineitem_l_comment: test_project__tpch__void_iATStreamIWvarchar_streamIM_com port map(
    clk => clk,
    rst => rst,
    input_valid => void_data_src_lineitem_l_comment__input_valid,
    input_ready => void_data_src_lineitem_l_comment__input_ready,
    input_data => void_data_src_lineitem_l_comment__input_data,
    input_last => void_data_src_lineitem_l_comment__input_last,
    input_strb => void_data_src_lineitem_l_comment__input_strb
  );
  void_data_src_lineitem_l_commitdate: test_project__tpch__void_iATStreamIWdate_streamIM_com port map(
    clk => clk,
    rst => rst,
    input_valid => void_data_src_lineitem_l_commitdate__input_valid,
    input_ready => void_data_src_lineitem_l_commitdate__input_ready,
    input_data => void_data_src_lineitem_l_commitdate__input_data,
    input_last => void_data_src_lineitem_l_commitdate__input_last,
    input_strb => void_data_src_lineitem_l_commitdate__input_strb
  );
  void_data_src_lineitem_l_linestatus: test_project__tpch__void_iATStreamIWSQL_char1_streamIM_com port map(
    clk => clk,
    rst => rst,
    input_valid => void_data_src_lineitem_l_linestatus__input_valid,
    input_ready => void_data_src_lineitem_l_linestatus__input_ready,
    input_data => void_data_src_lineitem_l_linestatus__input_data,
    input_last => void_data_src_lineitem_l_linestatus__input_last,
    input_strb => void_data_src_lineitem_l_linestatus__input_strb
  );
  void_data_src_lineitem_l_partkey: test_project__tpch__void_iATStreamIWint_streamIM_com port map(
    clk => clk,
    rst => rst,
    input_valid => void_data_src_lineitem_l_partkey__input_valid,
    input_ready => void_data_src_lineitem_l_partkey__input_ready,
    input_data => void_data_src_lineitem_l_partkey__input_data,
    input_last => void_data_src_lineitem_l_partkey__input_last,
    input_strb => void_data_src_lineitem_l_partkey__input_strb
  );
  void_data_src_lineitem_l_quantity: test_project__tpch__void_iATStreamIWSQL_decimal_15_2_streamIM_com port map(
    clk => clk,
    rst => rst,
    input_valid => void_data_src_lineitem_l_quantity__input_valid,
    input_ready => void_data_src_lineitem_l_quantity__input_ready,
    input_data => void_data_src_lineitem_l_quantity__input_data,
    input_last => void_data_src_lineitem_l_quantity__input_last,
    input_strb => void_data_src_lineitem_l_quantity__input_strb
  );
  void_data_src_lineitem_l_receiptdate: test_project__tpch__void_iATStreamIWdate_streamIM_com port map(
    clk => clk,
    rst => rst,
    input_valid => void_data_src_lineitem_l_receiptdate__input_valid,
    input_ready => void_data_src_lineitem_l_receiptdate__input_ready,
    input_data => void_data_src_lineitem_l_receiptdate__input_data,
    input_last => void_data_src_lineitem_l_receiptdate__input_last,
    input_strb => void_data_src_lineitem_l_receiptdate__input_strb
  );
  void_data_src_lineitem_l_returnflag: test_project__tpch__void_iATStreamIWSQL_char1_streamIM_com port map(
    clk => clk,
    rst => rst,
    input_valid => void_data_src_lineitem_l_returnflag__input_valid,
    input_ready => void_data_src_lineitem_l_returnflag__input_ready,
    input_data => void_data_src_lineitem_l_returnflag__input_data,
    input_last => void_data_src_lineitem_l_returnflag__input_last,
    input_strb => void_data_src_lineitem_l_returnflag__input_strb
  );
  void_data_src_lineitem_l_shipinstruct: test_project__tpch__void_iATStreamIWSQL_char25_streamIM_com port map(
    clk => clk,
    rst => rst,
    input_valid => void_data_src_lineitem_l_shipinstruct__input_valid,
    input_ready => void_data_src_lineitem_l_shipinstruct__input_ready,
    input_data => void_data_src_lineitem_l_shipinstruct__input_data,
    input_last => void_data_src_lineitem_l_shipinstruct__input_last,
    input_strb => void_data_src_lineitem_l_shipinstruct__input_strb
  );
  void_data_src_lineitem_l_shipmode: test_project__tpch__void_iATStreamIWSQL_char10_streamIM_com port map(
    clk => clk,
    rst => rst,
    input_valid => void_data_src_lineitem_l_shipmode__input_valid,
    input_ready => void_data_src_lineitem_l_shipmode__input_ready,
    input_data => void_data_src_lineitem_l_shipmode__input_data,
    input_last => void_data_src_lineitem_l_shipmode__input_last,
    input_strb => void_data_src_lineitem_l_shipmode__input_strb
  );
  void_data_src_lineitem_l_suppkey: test_project__tpch__void_iATStreamIWint_streamIM_com port map(
    clk => clk,
    rst => rst,
    input_valid => void_data_src_lineitem_l_suppkey__input_valid,
    input_ready => void_data_src_lineitem_l_suppkey__input_ready,
    input_data => void_data_src_lineitem_l_suppkey__input_data,
    input_last => void_data_src_lineitem_l_suppkey__input_last,
    input_strb => void_data_src_lineitem_l_suppkey__input_strb
  );
  void_data_src_lineitem_l_tax: test_project__tpch__void_iATStreamIWSQL_decimal_15_2_streamIM_com port map(
    clk => clk,
    rst => rst,
    input_valid => void_data_src_lineitem_l_tax__input_valid,
    input_ready => void_data_src_lineitem_l_tax__input_ready,
    input_data => void_data_src_lineitem_l_tax__input_data,
    input_last => void_data_src_lineitem_l_tax__input_last,
    input_strb => void_data_src_lineitem_l_tax__input_strb
  );
  void_data_src_orders_o_clerk: test_project__tpch__void_iATStreamIWSQL_char15_streamIM_com port map(
    clk => clk,
    rst => rst,
    input_valid => void_data_src_orders_o_clerk__input_valid,
    input_ready => void_data_src_orders_o_clerk__input_ready,
    input_data => void_data_src_orders_o_clerk__input_data,
    input_last => void_data_src_orders_o_clerk__input_last,
    input_strb => void_data_src_orders_o_clerk__input_strb
  );
  void_data_src_orders_o_comment: test_project__tpch__void_iATStreamIWvarchar_streamIM_com port map(
    clk => clk,
    rst => rst,
    input_valid => void_data_src_orders_o_comment__input_valid,
    input_ready => void_data_src_orders_o_comment__input_ready,
    input_data => void_data_src_orders_o_comment__input_data,
    input_last => void_data_src_orders_o_comment__input_last,
    input_strb => void_data_src_orders_o_comment__input_strb
  );
  void_data_src_orders_o_orderpriority: test_project__tpch__void_iATStreamIWSQL_char15_streamIM_com port map(
    clk => clk,
    rst => rst,
    input_valid => void_data_src_orders_o_orderpriority__input_valid,
    input_ready => void_data_src_orders_o_orderpriority__input_ready,
    input_data => void_data_src_orders_o_orderpriority__input_data,
    input_last => void_data_src_orders_o_orderpriority__input_last,
    input_strb => void_data_src_orders_o_orderpriority__input_strb
  );
  void_data_src_orders_o_orderstatus: test_project__tpch__void_iATStreamIWSQL_char1_streamIM_com port map(
    clk => clk,
    rst => rst,
    input_valid => void_data_src_orders_o_orderstatus__input_valid,
    input_ready => void_data_src_orders_o_orderstatus__input_ready,
    input_data => void_data_src_orders_o_orderstatus__input_data,
    input_last => void_data_src_orders_o_orderstatus__input_last,
    input_strb => void_data_src_orders_o_orderstatus__input_strb
  );
  void_data_src_orders_o_totalprice: test_project__tpch__void_iATStreamIWSQL_decimal_15_2_streamIM_com port map(
    clk => clk,
    rst => rst,
    input_valid => void_data_src_orders_o_totalprice__input_valid,
    input_ready => void_data_src_orders_o_totalprice__input_ready,
    input_data => void_data_src_orders_o_totalprice__input_data,
    input_last => void_data_src_orders_o_totalprice__input_last,
    input_strb => void_data_src_orders_o_totalprice__input_strb
  );
  data_filter__o_shippriority_in_valid <= data_src_orders__o_shippriority_valid;
  data_src_orders__o_shippriority_ready <= data_filter__o_shippriority_in_ready;
  data_filter__o_shippriority_in_data <= data_src_orders__o_shippriority_data;
  data_filter__o_shippriority_in_last <= data_src_orders__o_shippriority_last;
  data_filter__o_shippriority_in_strb <= data_src_orders__o_shippriority_strb;
  void_data_src_lineitem_l_linestatus__input_valid <= data_src_lineitem__l_linestatus_valid;
  data_src_lineitem__l_linestatus_ready <= void_data_src_lineitem_l_linestatus__input_ready;
  void_data_src_lineitem_l_linestatus__input_data <= data_src_lineitem__l_linestatus_data;
  void_data_src_lineitem_l_linestatus__input_last <= data_src_lineitem__l_linestatus_last;
  void_data_src_lineitem_l_linestatus__input_strb <= data_src_lineitem__l_linestatus_strb;
  err_and__inputAT0_valid <= multiplier__overflow_valid;
  multiplier__overflow_ready <= err_and__inputAT0_ready;
  err_and__inputAT0_data <= multiplier__overflow_data;
  err_and__inputAT0_strb <= multiplier__overflow_strb;
  void_data_src_orders_o_comment__input_valid <= data_src_orders__o_comment_valid;
  data_src_orders__o_comment_ready <= void_data_src_orders_o_comment__input_ready;
  void_data_src_orders_o_comment__input_data <= data_src_orders__o_comment_data;
  void_data_src_orders_o_comment__input_last <= data_src_orders__o_comment_last;
  void_data_src_orders_o_comment__input_strb <= data_src_orders__o_comment_strb;
  data_filter__o_orderdate_in_valid <= duplicate_data_src_orders_o_orderdate__outputAT0_valid;
  duplicate_data_src_orders_o_orderdate__outputAT0_ready <= data_filter__o_orderdate_in_ready;
  data_filter__o_orderdate_in_data <= duplicate_data_src_orders_o_orderdate__outputAT0_data;
  data_filter__o_orderdate_in_last <= duplicate_data_src_orders_o_orderdate__outputAT0_last;
  data_filter__o_orderdate_in_strb <= duplicate_data_src_orders_o_orderdate__outputAT0_strb;
  adder__input1_valid <= to_neg__output_valid;
  to_neg__output_ready <= adder__input1_ready;
  adder__input1_data <= to_neg__output_data;
  adder__input1_last <= to_neg__output_last;
  adder__input1_strb <= to_neg__output_strb;
  data_src_lineitem__l_linenumber_valid <= l_linenumber_valid;
  l_linenumber_ready <= data_src_lineitem__l_linenumber_ready;
  data_src_lineitem__l_linenumber_data <= l_linenumber_data;
  data_src_lineitem__l_linenumber_last <= l_linenumber_last;
  data_src_lineitem__l_linenumber_strb <= l_linenumber_strb;
  o_orderdate_valid <= data_filter__o_orderdate_out_valid;
  data_filter__o_orderdate_out_ready <= o_orderdate_ready;
  o_orderdate_data <= data_filter__o_orderdate_out_data;
  o_orderdate_last <= data_filter__o_orderdate_out_last;
  o_orderdate_strb <= data_filter__o_orderdate_out_strb;
  void_data_src_customer_c_address__input_valid <= data_src_customer__c_address_valid;
  data_src_customer__c_address_ready <= void_data_src_customer_c_address__input_ready;
  void_data_src_customer_c_address__input_data <= data_src_customer__c_address_data;
  void_data_src_customer_c_address__input_last <= data_src_customer__c_address_last;
  void_data_src_customer_c_address__input_strb <= data_src_customer__c_address_strb;
  void_data_src_lineitem_l_partkey__input_valid <= data_src_lineitem__l_partkey_valid;
  data_src_lineitem__l_partkey_ready <= void_data_src_lineitem_l_partkey__input_ready;
  void_data_src_lineitem_l_partkey__input_data <= data_src_lineitem__l_partkey_data;
  void_data_src_lineitem_l_partkey__input_last <= data_src_lineitem__l_partkey_last;
  void_data_src_lineitem_l_partkey__input_strb <= data_src_lineitem__l_partkey_strb;
  duplicate_o_orderkey__input_valid <= o_orderkey_valid;
  o_orderkey_ready <= duplicate_o_orderkey__input_ready;
  duplicate_o_orderkey__input_data <= o_orderkey_data;
  duplicate_o_orderkey__input_last <= o_orderkey_last;
  duplicate_o_orderkey__input_strb <= o_orderkey_strb;
  void_data_src_lineitem_l_commitdate__input_valid <= data_src_lineitem__l_commitdate_valid;
  data_src_lineitem__l_commitdate_ready <= void_data_src_lineitem_l_commitdate__input_ready;
  void_data_src_lineitem_l_commitdate__input_data <= data_src_lineitem__l_commitdate_data;
  void_data_src_lineitem_l_commitdate__input_last <= data_src_lineitem__l_commitdate_last;
  void_data_src_lineitem_l_commitdate__input_strb <= data_src_lineitem__l_commitdate_strb;
  void_data_src_orders_o_clerk__input_valid <= data_src_orders__o_clerk_valid;
  data_src_orders__o_clerk_ready <= void_data_src_orders_o_clerk__input_ready;
  void_data_src_orders_o_clerk__input_data <= data_src_orders__o_clerk_data;
  void_data_src_orders_o_clerk__input_last <= data_src_orders__o_clerk_last;
  void_data_src_orders_o_clerk__input_strb <= data_src_orders__o_clerk_strb;
  void_data_src_lineitem_l_tax__input_valid <= data_src_lineitem__l_tax_valid;
  data_src_lineitem__l_tax_ready <= void_data_src_lineitem_l_tax__input_ready;
  void_data_src_lineitem_l_tax__input_data <= data_src_lineitem__l_tax_data;
  void_data_src_lineitem_l_tax__input_last <= data_src_lineitem__l_tax_last;
  void_data_src_lineitem_l_tax__input_strb <= data_src_lineitem__l_tax_strb;
  void_data_src_customer_c_name__input_valid <= data_src_customer__c_name_valid;
  data_src_customer__c_name_ready <= void_data_src_customer_c_name__input_ready;
  void_data_src_customer_c_name__input_data <= data_src_customer__c_name_data;
  void_data_src_customer_c_name__input_last <= data_src_customer__c_name_last;
  void_data_src_customer_c_name__input_strb <= data_src_customer__c_name_strb;
  data_filter__c_mktsegment_valid <= data_src_customer__c_mktsegment_valid;
  data_src_customer__c_mktsegment_ready <= data_filter__c_mktsegment_ready;
  data_filter__c_mktsegment_data <= data_src_customer__c_mktsegment_data;
  data_filter__c_mktsegment_last <= data_src_customer__c_mktsegment_last;
  data_filter__c_mktsegment_strb <= data_src_customer__c_mktsegment_strb;
  data_filter__o_orderkey_valid <= duplicate_o_orderkey__outputAT1_valid;
  duplicate_o_orderkey__outputAT1_ready <= data_filter__o_orderkey_ready;
  data_filter__o_orderkey_data <= duplicate_o_orderkey__outputAT1_data;
  data_filter__o_orderkey_last <= duplicate_o_orderkey__outputAT1_last;
  data_filter__o_orderkey_strb <= duplicate_o_orderkey__outputAT1_strb;
  void_data_src_lineitem_l_quantity__input_valid <= data_src_lineitem__l_quantity_valid;
  data_src_lineitem__l_quantity_ready <= void_data_src_lineitem_l_quantity__input_ready;
  void_data_src_lineitem_l_quantity__input_data <= data_src_lineitem__l_quantity_data;
  void_data_src_lineitem_l_quantity__input_last <= data_src_lineitem__l_quantity_last;
  void_data_src_lineitem_l_quantity__input_strb <= data_src_lineitem__l_quantity_strb;
  multiplier__input1_valid <= data_filter__l_extendedprice_out_valid;
  data_filter__l_extendedprice_out_ready <= multiplier__input1_ready;
  multiplier__input1_data <= data_filter__l_extendedprice_out_data;
  multiplier__input1_last <= data_filter__l_extendedprice_out_last;
  multiplier__input1_strb <= data_filter__l_extendedprice_out_strb;
  data_src_orders__o_orderkey_valid <= duplicate_o_orderkey__outputAT0_valid;
  duplicate_o_orderkey__outputAT0_ready <= data_src_orders__o_orderkey_ready;
  data_src_orders__o_orderkey_data <= duplicate_o_orderkey__outputAT0_data;
  data_src_orders__o_orderkey_last <= duplicate_o_orderkey__outputAT0_last;
  data_src_orders__o_orderkey_strb <= duplicate_o_orderkey__outputAT0_strb;
  void_data_src_lineitem_l_shipinstruct__input_valid <= data_src_lineitem__l_shipinstruct_valid;
  data_src_lineitem__l_shipinstruct_ready <= void_data_src_lineitem_l_shipinstruct__input_ready;
  void_data_src_lineitem_l_shipinstruct__input_data <= data_src_lineitem__l_shipinstruct_data;
  void_data_src_lineitem_l_shipinstruct__input_last <= data_src_lineitem__l_shipinstruct_last;
  void_data_src_lineitem_l_shipinstruct__input_strb <= data_src_lineitem__l_shipinstruct_strb;
  void_accu_count__input_valid <= accu__count_valid;
  accu__count_ready <= void_accu_count__input_ready;
  void_accu_count__input_data <= accu__count_data;
  void_accu_count__input_strb <= accu__count_strb;
  data_filter__o_custkey_valid <= data_src_orders__o_custkey_valid;
  data_src_orders__o_custkey_ready <= data_filter__o_custkey_ready;
  data_filter__o_custkey_data <= data_src_orders__o_custkey_data;
  data_filter__o_custkey_last <= data_src_orders__o_custkey_last;
  data_filter__o_custkey_strb <= data_src_orders__o_custkey_strb;
  to_neg__input_valid <= data_filter__l_discount_out_valid;
  data_filter__l_discount_out_ready <= to_neg__input_ready;
  to_neg__input_data <= data_filter__l_discount_out_data;
  to_neg__input_last <= data_filter__l_discount_out_last;
  to_neg__input_strb <= data_filter__l_discount_out_strb;
  data_src_customer__c_custkey_valid <= duplicate_c_custkey__outputAT1_valid;
  duplicate_c_custkey__outputAT1_ready <= data_src_customer__c_custkey_ready;
  data_src_customer__c_custkey_data <= duplicate_c_custkey__outputAT1_data;
  data_src_customer__c_custkey_last <= duplicate_c_custkey__outputAT1_last;
  data_src_customer__c_custkey_strb <= duplicate_c_custkey__outputAT1_strb;
  void_data_src_orders_o_totalprice__input_valid <= data_src_orders__o_totalprice_valid;
  data_src_orders__o_totalprice_ready <= void_data_src_orders_o_totalprice__input_ready;
  void_data_src_orders_o_totalprice__input_data <= data_src_orders__o_totalprice_data;
  void_data_src_orders_o_totalprice__input_last <= data_src_orders__o_totalprice_last;
  void_data_src_orders_o_totalprice__input_strb <= data_src_orders__o_totalprice_strb;
  void_data_src_orders_o_orderstatus__input_valid <= data_src_orders__o_orderstatus_valid;
  data_src_orders__o_orderstatus_ready <= void_data_src_orders_o_orderstatus__input_ready;
  void_data_src_orders_o_orderstatus__input_data <= data_src_orders__o_orderstatus_data;
  void_data_src_orders_o_orderstatus__input_last <= data_src_orders__o_orderstatus_last;
  void_data_src_orders_o_orderstatus__input_strb <= data_src_orders__o_orderstatus_strb;
  data_filter__l_orderkey_valid <= duplicate_l_orderkey__outputAT0_valid;
  duplicate_l_orderkey__outputAT0_ready <= data_filter__l_orderkey_ready;
  data_filter__l_orderkey_data <= duplicate_l_orderkey__outputAT0_data;
  data_filter__l_orderkey_last <= duplicate_l_orderkey__outputAT0_last;
  data_filter__l_orderkey_strb <= duplicate_l_orderkey__outputAT0_strb;
  data_filter__l_discount_in_valid <= data_src_lineitem__l_discount_valid;
  data_src_lineitem__l_discount_ready <= data_filter__l_discount_in_ready;
  data_filter__l_discount_in_data <= data_src_lineitem__l_discount_data;
  data_filter__l_discount_in_last <= data_src_lineitem__l_discount_last;
  data_filter__l_discount_in_strb <= data_src_lineitem__l_discount_strb;
  void_data_src_customer_c_comment__input_valid <= data_src_customer__c_comment_valid;
  data_src_customer__c_comment_ready <= void_data_src_customer_c_comment__input_ready;
  void_data_src_customer_c_comment__input_data <= data_src_customer__c_comment_data;
  void_data_src_customer_c_comment__input_last <= data_src_customer__c_comment_last;
  void_data_src_customer_c_comment__input_strb <= data_src_customer__c_comment_strb;
  data_filter__c_custkey_valid <= duplicate_c_custkey__outputAT0_valid;
  duplicate_c_custkey__outputAT0_ready <= data_filter__c_custkey_ready;
  data_filter__c_custkey_data <= duplicate_c_custkey__outputAT0_data;
  data_filter__c_custkey_last <= duplicate_c_custkey__outputAT0_last;
  data_filter__c_custkey_strb <= duplicate_c_custkey__outputAT0_strb;
  adder__input0_valid <= const_value_gen__output_valid;
  const_value_gen__output_ready <= adder__input0_ready;
  adder__input0_data <= const_value_gen__output_data;
  adder__input0_last <= const_value_gen__output_last;
  adder__input0_strb <= const_value_gen__output_strb;
  revenue_valid <= accu__output_valid;
  accu__output_ready <= revenue_ready;
  revenue_data <= accu__output_data;
  revenue_last <= accu__output_last;
  revenue_strb <= accu__output_strb;
  err_valid <= err_and__output_valid;
  err_and__output_ready <= err_ready;
  err_data <= err_and__output_data;
  err_strb <= err_and__output_strb;
  void_data_src_lineitem_l_receiptdate__input_valid <= data_src_lineitem__l_receiptdate_valid;
  data_src_lineitem__l_receiptdate_ready <= void_data_src_lineitem_l_receiptdate__input_ready;
  void_data_src_lineitem_l_receiptdate__input_data <= data_src_lineitem__l_receiptdate_data;
  void_data_src_lineitem_l_receiptdate__input_last <= data_src_lineitem__l_receiptdate_last;
  void_data_src_lineitem_l_receiptdate__input_strb <= data_src_lineitem__l_receiptdate_strb;
  duplicate_c_custkey__input_valid <= c_custkey_valid;
  c_custkey_ready <= duplicate_c_custkey__input_ready;
  duplicate_c_custkey__input_data <= c_custkey_data;
  duplicate_c_custkey__input_last <= c_custkey_last;
  duplicate_c_custkey__input_strb <= c_custkey_strb;
  void_data_src_lineitem_l_comment__input_valid <= data_src_lineitem__l_comment_valid;
  data_src_lineitem__l_comment_ready <= void_data_src_lineitem_l_comment__input_ready;
  void_data_src_lineitem_l_comment__input_data <= data_src_lineitem__l_comment_data;
  void_data_src_lineitem_l_comment__input_last <= data_src_lineitem__l_comment_last;
  void_data_src_lineitem_l_comment__input_strb <= data_src_lineitem__l_comment_strb;
  data_filter__o_orderdate_valid <= duplicate_data_src_orders_o_orderdate__outputAT1_valid;
  duplicate_data_src_orders_o_orderdate__outputAT1_ready <= data_filter__o_orderdate_ready;
  data_filter__o_orderdate_data <= duplicate_data_src_orders_o_orderdate__outputAT1_data;
  data_filter__o_orderdate_last <= duplicate_data_src_orders_o_orderdate__outputAT1_last;
  data_filter__o_orderdate_strb <= duplicate_data_src_orders_o_orderdate__outputAT1_strb;
  void_data_src_lineitem_l_shipmode__input_valid <= data_src_lineitem__l_shipmode_valid;
  data_src_lineitem__l_shipmode_ready <= void_data_src_lineitem_l_shipmode__input_ready;
  void_data_src_lineitem_l_shipmode__input_data <= data_src_lineitem__l_shipmode_data;
  void_data_src_lineitem_l_shipmode__input_last <= data_src_lineitem__l_shipmode_last;
  void_data_src_lineitem_l_shipmode__input_strb <= data_src_lineitem__l_shipmode_strb;
  err_and__inputAT1_valid <= accu__overflow_valid;
  accu__overflow_ready <= err_and__inputAT1_ready;
  err_and__inputAT1_data <= accu__overflow_data;
  err_and__inputAT1_strb <= accu__overflow_strb;
  data_src_lineitem__l_orderkey_valid <= duplicate_l_orderkey__outputAT1_valid;
  duplicate_l_orderkey__outputAT1_ready <= data_src_lineitem__l_orderkey_ready;
  data_src_lineitem__l_orderkey_data <= duplicate_l_orderkey__outputAT1_data;
  data_src_lineitem__l_orderkey_last <= duplicate_l_orderkey__outputAT1_last;
  data_src_lineitem__l_orderkey_strb <= duplicate_l_orderkey__outputAT1_strb;
  o_shippriority_valid <= data_filter__o_shippriority_out_valid;
  data_filter__o_shippriority_out_ready <= o_shippriority_ready;
  o_shippriority_data <= data_filter__o_shippriority_out_data;
  o_shippriority_last <= data_filter__o_shippriority_out_last;
  o_shippriority_strb <= data_filter__o_shippriority_out_strb;
  void_data_src_lineitem_l_suppkey__input_valid <= data_src_lineitem__l_suppkey_valid;
  data_src_lineitem__l_suppkey_ready <= void_data_src_lineitem_l_suppkey__input_ready;
  void_data_src_lineitem_l_suppkey__input_data <= data_src_lineitem__l_suppkey_data;
  void_data_src_lineitem_l_suppkey__input_last <= data_src_lineitem__l_suppkey_last;
  void_data_src_lineitem_l_suppkey__input_strb <= data_src_lineitem__l_suppkey_strb;
  void_data_src_customer_c_nationkey__input_valid <= data_src_customer__c_nationkey_valid;
  data_src_customer__c_nationkey_ready <= void_data_src_customer_c_nationkey__input_ready;
  void_data_src_customer_c_nationkey__input_data <= data_src_customer__c_nationkey_data;
  void_data_src_customer_c_nationkey__input_last <= data_src_customer__c_nationkey_last;
  void_data_src_customer_c_nationkey__input_strb <= data_src_customer__c_nationkey_strb;
  void_data_src_orders_o_orderpriority__input_valid <= data_src_orders__o_orderpriority_valid;
  data_src_orders__o_orderpriority_ready <= void_data_src_orders_o_orderpriority__input_ready;
  void_data_src_orders_o_orderpriority__input_data <= data_src_orders__o_orderpriority_data;
  void_data_src_orders_o_orderpriority__input_last <= data_src_orders__o_orderpriority_last;
  void_data_src_orders_o_orderpriority__input_strb <= data_src_orders__o_orderpriority_strb;
  data_filter__l_extendedprice_in_valid <= data_src_lineitem__l_extendedprice_valid;
  data_src_lineitem__l_extendedprice_ready <= data_filter__l_extendedprice_in_ready;
  data_filter__l_extendedprice_in_data <= data_src_lineitem__l_extendedprice_data;
  data_filter__l_extendedprice_in_last <= data_src_lineitem__l_extendedprice_last;
  data_filter__l_extendedprice_in_strb <= data_src_lineitem__l_extendedprice_strb;
  err_and__inputAT2_valid <= adder__overflow_valid;
  adder__overflow_ready <= err_and__inputAT2_ready;
  err_and__inputAT2_data <= adder__overflow_data;
  err_and__inputAT2_strb <= adder__overflow_strb;
  data_filter__l_shipdate_valid <= data_src_lineitem__l_shipdate_valid;
  data_src_lineitem__l_shipdate_ready <= data_filter__l_shipdate_ready;
  data_filter__l_shipdate_data <= data_src_lineitem__l_shipdate_data;
  data_filter__l_shipdate_last <= data_src_lineitem__l_shipdate_last;
  data_filter__l_shipdate_strb <= data_src_lineitem__l_shipdate_strb;
  l_orderkey_out_valid <= data_filter__l_orderkey_out_valid;
  data_filter__l_orderkey_out_ready <= l_orderkey_out_ready;
  l_orderkey_out_data <= data_filter__l_orderkey_out_data;
  l_orderkey_out_last <= data_filter__l_orderkey_out_last;
  l_orderkey_out_strb <= data_filter__l_orderkey_out_strb;
  duplicate_data_src_orders_o_orderdate__input_valid <= data_src_orders__o_orderdate_valid;
  data_src_orders__o_orderdate_ready <= duplicate_data_src_orders_o_orderdate__input_ready;
  duplicate_data_src_orders_o_orderdate__input_data <= data_src_orders__o_orderdate_data;
  duplicate_data_src_orders_o_orderdate__input_last <= data_src_orders__o_orderdate_last;
  duplicate_data_src_orders_o_orderdate__input_strb <= data_src_orders__o_orderdate_strb;
  duplicate_l_orderkey__input_valid <= l_orderkey_valid;
  l_orderkey_ready <= duplicate_l_orderkey__input_ready;
  duplicate_l_orderkey__input_data <= l_orderkey_data;
  duplicate_l_orderkey__input_last <= l_orderkey_last;
  duplicate_l_orderkey__input_strb <= l_orderkey_strb;
  accu__input_valid <= multiplier__output_valid;
  multiplier__output_ready <= accu__input_ready;
  accu__input_data <= multiplier__output_data;
  accu__input_last <= multiplier__output_last;
  accu__input_strb <= multiplier__output_strb;
  void_data_src_lineitem_l_returnflag__input_valid <= data_src_lineitem__l_returnflag_valid;
  data_src_lineitem__l_returnflag_ready <= void_data_src_lineitem_l_returnflag__input_ready;
  void_data_src_lineitem_l_returnflag__input_data <= data_src_lineitem__l_returnflag_data;
  void_data_src_lineitem_l_returnflag__input_last <= data_src_lineitem__l_returnflag_last;
  void_data_src_lineitem_l_returnflag__input_strb <= data_src_lineitem__l_returnflag_strb;
  data_filter__l_orderkey_in_valid <= duplicate_l_orderkey__outputAT2_valid;
  duplicate_l_orderkey__outputAT2_ready <= data_filter__l_orderkey_in_ready;
  data_filter__l_orderkey_in_data <= duplicate_l_orderkey__outputAT2_data;
  data_filter__l_orderkey_in_last <= duplicate_l_orderkey__outputAT2_last;
  data_filter__l_orderkey_in_strb <= duplicate_l_orderkey__outputAT2_strb;
  multiplier__input0_valid <= adder__output_valid;
  adder__output_ready <= multiplier__input0_ready;
  multiplier__input0_data <= adder__output_data;
  multiplier__input0_last <= adder__output_last;
  multiplier__input0_strb <= adder__output_strb;
  void_data_src_customer_c_phone__input_valid <= data_src_customer__c_phone_valid;
  data_src_customer__c_phone_ready <= void_data_src_customer_c_phone__input_ready;
  void_data_src_customer_c_phone__input_data <= data_src_customer__c_phone_data;
  void_data_src_customer_c_phone__input_last <= data_src_customer__c_phone_last;
  void_data_src_customer_c_phone__input_strb <= data_src_customer__c_phone_strb;
  void_data_src_customer_c_acctbal__input_valid <= data_src_customer__c_acctbal_valid;
  data_src_customer__c_acctbal_ready <= void_data_src_customer_c_acctbal__input_ready;
  void_data_src_customer_c_acctbal__input_data <= data_src_customer__c_acctbal_data;
  void_data_src_customer_c_acctbal__input_last <= data_src_customer__c_acctbal_last;
  void_data_src_customer_c_acctbal__input_strb <= data_src_customer__c_acctbal_strb;
end test_project__tpch__main_i;