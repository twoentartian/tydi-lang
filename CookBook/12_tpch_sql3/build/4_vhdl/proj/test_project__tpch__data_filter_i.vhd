library ieee;
use ieee.std_logic_1164.all;

library work;
use work.proj.all;

entity test_project__tpch__data_filter_i_com is
  port (
    clk : in std_logic;
    rst : in std_logic;
    l_orderkey_in_valid : in std_logic;
    l_orderkey_in_ready : out std_logic;
    l_orderkey_in_data : in std_logic_vector(31 downto 0);
    l_orderkey_in_last : in std_logic;
    l_orderkey_in_strb : in std_logic;
    o_orderdate_in_valid : in std_logic;
    o_orderdate_in_ready : out std_logic;
    o_orderdate_in_data : in std_logic_vector(25 downto 0);
    o_orderdate_in_last : in std_logic;
    o_orderdate_in_strb : in std_logic;
    l_orderkey_valid : in std_logic;
    l_orderkey_ready : out std_logic;
    l_orderkey_data : in std_logic_vector(31 downto 0);
    l_orderkey_last : in std_logic;
    l_orderkey_strb : in std_logic;
    o_orderkey_valid : in std_logic;
    o_orderkey_ready : out std_logic;
    o_orderkey_data : in std_logic_vector(31 downto 0);
    o_orderkey_last : in std_logic;
    o_orderkey_strb : in std_logic;
    o_custkey_valid : in std_logic;
    o_custkey_ready : out std_logic;
    o_custkey_data : in std_logic_vector(31 downto 0);
    o_custkey_last : in std_logic;
    o_custkey_strb : in std_logic;
    c_custkey_valid : in std_logic;
    c_custkey_ready : out std_logic;
    c_custkey_data : in std_logic_vector(31 downto 0);
    c_custkey_last : in std_logic;
    c_custkey_strb : in std_logic;
    l_extendedprice_in_valid : in std_logic;
    l_extendedprice_in_ready : out std_logic;
    l_extendedprice_in_data : in std_logic_vector(49 downto 0);
    l_extendedprice_in_last : in std_logic;
    l_extendedprice_in_strb : in std_logic;
    l_shipdate_valid : in std_logic;
    l_shipdate_ready : out std_logic;
    l_shipdate_data : in std_logic_vector(25 downto 0);
    l_shipdate_last : in std_logic;
    l_shipdate_strb : in std_logic;
    o_shippriority_in_valid : in std_logic;
    o_shippriority_in_ready : out std_logic;
    o_shippriority_in_data : in std_logic_vector(31 downto 0);
    o_shippriority_in_last : in std_logic;
    o_shippriority_in_strb : in std_logic;
    c_mktsegment_valid : in std_logic;
    c_mktsegment_ready : out std_logic;
    c_mktsegment_data : in std_logic_vector(79 downto 0);
    c_mktsegment_last : in std_logic;
    c_mktsegment_strb : in std_logic;
    l_discount_in_valid : in std_logic;
    l_discount_in_ready : out std_logic;
    l_discount_in_data : in std_logic_vector(49 downto 0);
    l_discount_in_last : in std_logic;
    l_discount_in_strb : in std_logic;
    o_orderdate_valid : in std_logic;
    o_orderdate_ready : out std_logic;
    o_orderdate_data : in std_logic_vector(25 downto 0);
    o_orderdate_last : in std_logic;
    o_orderdate_strb : in std_logic;
    o_orderdate_out_valid : out std_logic;
    o_orderdate_out_ready : in std_logic;
    o_orderdate_out_data : out std_logic_vector(25 downto 0);
    o_orderdate_out_last : out std_logic;
    o_orderdate_out_strb : out std_logic;
    l_extendedprice_out_valid : out std_logic;
    l_extendedprice_out_ready : in std_logic;
    l_extendedprice_out_data : out std_logic_vector(49 downto 0);
    l_extendedprice_out_last : out std_logic;
    l_extendedprice_out_strb : out std_logic;
    l_discount_out_valid : out std_logic;
    l_discount_out_ready : in std_logic;
    l_discount_out_data : out std_logic_vector(49 downto 0);
    l_discount_out_last : out std_logic;
    l_discount_out_strb : out std_logic;
    l_orderkey_out_valid : out std_logic;
    l_orderkey_out_ready : in std_logic;
    l_orderkey_out_data : out std_logic_vector(31 downto 0);
    l_orderkey_out_last : out std_logic;
    l_orderkey_out_strb : out std_logic;
    o_shippriority_out_valid : out std_logic;
    o_shippriority_out_ready : in std_logic;
    o_shippriority_out_data : out std_logic_vector(31 downto 0);
    o_shippriority_out_last : out std_logic;
    o_shippriority_out_strb : out std_logic
  );
end test_project__tpch__data_filter_i_com;

architecture test_project__tpch__data_filter_i of test_project__tpch__data_filter_i_com is
  signal duplicate_selection_select__input_valid : std_logic;
  signal duplicate_selection_select__input_ready : std_logic;
  signal duplicate_selection_select__input_data : std_logic;
  signal duplicate_selection_select__input_strb : std_logic;
  signal duplicate_selection_select__outputAT0_valid : std_logic;
  signal duplicate_selection_select__outputAT0_ready : std_logic;
  signal duplicate_selection_select__outputAT0_data : std_logic;
  signal duplicate_selection_select__outputAT0_strb : std_logic;
  signal duplicate_selection_select__outputAT1_valid : std_logic;
  signal duplicate_selection_select__outputAT1_ready : std_logic;
  signal duplicate_selection_select__outputAT1_data : std_logic;
  signal duplicate_selection_select__outputAT1_strb : std_logic;
  signal duplicate_selection_select__outputAT2_valid : std_logic;
  signal duplicate_selection_select__outputAT2_ready : std_logic;
  signal duplicate_selection_select__outputAT2_data : std_logic;
  signal duplicate_selection_select__outputAT2_strb : std_logic;
  signal duplicate_selection_select__outputAT3_valid : std_logic;
  signal duplicate_selection_select__outputAT3_ready : std_logic;
  signal duplicate_selection_select__outputAT3_data : std_logic;
  signal duplicate_selection_select__outputAT3_strb : std_logic;
  signal duplicate_selection_select__outputAT4_valid : std_logic;
  signal duplicate_selection_select__outputAT4_ready : std_logic;
  signal duplicate_selection_select__outputAT4_data : std_logic;
  signal duplicate_selection_select__outputAT4_strb : std_logic;
  signal l_discount_filter__input_valid : std_logic;
  signal l_discount_filter__input_ready : std_logic;
  signal l_discount_filter__input_data : std_logic_vector(49 downto 0);
  signal l_discount_filter__input_last : std_logic;
  signal l_discount_filter__input_strb : std_logic;
  signal l_discount_filter__output_valid : std_logic;
  signal l_discount_filter__output_ready : std_logic;
  signal l_discount_filter__output_data : std_logic_vector(49 downto 0);
  signal l_discount_filter__output_last : std_logic;
  signal l_discount_filter__output_strb : std_logic;
  signal l_discount_filter__select_valid : std_logic;
  signal l_discount_filter__select_ready : std_logic;
  signal l_discount_filter__select_data : std_logic;
  signal l_discount_filter__select_strb : std_logic;
  signal l_extendedprice_filter__input_valid : std_logic;
  signal l_extendedprice_filter__input_ready : std_logic;
  signal l_extendedprice_filter__input_data : std_logic_vector(49 downto 0);
  signal l_extendedprice_filter__input_last : std_logic;
  signal l_extendedprice_filter__input_strb : std_logic;
  signal l_extendedprice_filter__output_valid : std_logic;
  signal l_extendedprice_filter__output_ready : std_logic;
  signal l_extendedprice_filter__output_data : std_logic_vector(49 downto 0);
  signal l_extendedprice_filter__output_last : std_logic;
  signal l_extendedprice_filter__output_strb : std_logic;
  signal l_extendedprice_filter__select_valid : std_logic;
  signal l_extendedprice_filter__select_ready : std_logic;
  signal l_extendedprice_filter__select_data : std_logic;
  signal l_extendedprice_filter__select_strb : std_logic;
  signal l_orderkey_filter__input_valid : std_logic;
  signal l_orderkey_filter__input_ready : std_logic;
  signal l_orderkey_filter__input_data : std_logic_vector(31 downto 0);
  signal l_orderkey_filter__input_last : std_logic;
  signal l_orderkey_filter__input_strb : std_logic;
  signal l_orderkey_filter__output_valid : std_logic;
  signal l_orderkey_filter__output_ready : std_logic;
  signal l_orderkey_filter__output_data : std_logic_vector(31 downto 0);
  signal l_orderkey_filter__output_last : std_logic;
  signal l_orderkey_filter__output_strb : std_logic;
  signal l_orderkey_filter__select_valid : std_logic;
  signal l_orderkey_filter__select_ready : std_logic;
  signal l_orderkey_filter__select_data : std_logic;
  signal l_orderkey_filter__select_strb : std_logic;
  signal o_orderdate_filter__input_valid : std_logic;
  signal o_orderdate_filter__input_ready : std_logic;
  signal o_orderdate_filter__input_data : std_logic_vector(25 downto 0);
  signal o_orderdate_filter__input_last : std_logic;
  signal o_orderdate_filter__input_strb : std_logic;
  signal o_orderdate_filter__output_valid : std_logic;
  signal o_orderdate_filter__output_ready : std_logic;
  signal o_orderdate_filter__output_data : std_logic_vector(25 downto 0);
  signal o_orderdate_filter__output_last : std_logic;
  signal o_orderdate_filter__output_strb : std_logic;
  signal o_orderdate_filter__select_valid : std_logic;
  signal o_orderdate_filter__select_ready : std_logic;
  signal o_orderdate_filter__select_data : std_logic;
  signal o_orderdate_filter__select_strb : std_logic;
  signal o_shippriority_filter__input_valid : std_logic;
  signal o_shippriority_filter__input_ready : std_logic;
  signal o_shippriority_filter__input_data : std_logic_vector(31 downto 0);
  signal o_shippriority_filter__input_last : std_logic;
  signal o_shippriority_filter__input_strb : std_logic;
  signal o_shippriority_filter__output_valid : std_logic;
  signal o_shippriority_filter__output_ready : std_logic;
  signal o_shippriority_filter__output_data : std_logic_vector(31 downto 0);
  signal o_shippriority_filter__output_last : std_logic;
  signal o_shippriority_filter__output_strb : std_logic;
  signal o_shippriority_filter__select_valid : std_logic;
  signal o_shippriority_filter__select_ready : std_logic;
  signal o_shippriority_filter__select_data : std_logic;
  signal o_shippriority_filter__select_strb : std_logic;
  signal selection__c_custkey_valid : std_logic;
  signal selection__c_custkey_ready : std_logic;
  signal selection__c_custkey_data : std_logic_vector(31 downto 0);
  signal selection__c_custkey_last : std_logic;
  signal selection__c_custkey_strb : std_logic;
  signal selection__c_mktsegment_valid : std_logic;
  signal selection__c_mktsegment_ready : std_logic;
  signal selection__c_mktsegment_data : std_logic_vector(79 downto 0);
  signal selection__c_mktsegment_last : std_logic;
  signal selection__c_mktsegment_strb : std_logic;
  signal selection__l_orderkey_valid : std_logic;
  signal selection__l_orderkey_ready : std_logic;
  signal selection__l_orderkey_data : std_logic_vector(31 downto 0);
  signal selection__l_orderkey_last : std_logic;
  signal selection__l_orderkey_strb : std_logic;
  signal selection__l_shipdate_valid : std_logic;
  signal selection__l_shipdate_ready : std_logic;
  signal selection__l_shipdate_data : std_logic_vector(25 downto 0);
  signal selection__l_shipdate_last : std_logic;
  signal selection__l_shipdate_strb : std_logic;
  signal selection__o_custkey_valid : std_logic;
  signal selection__o_custkey_ready : std_logic;
  signal selection__o_custkey_data : std_logic_vector(31 downto 0);
  signal selection__o_custkey_last : std_logic;
  signal selection__o_custkey_strb : std_logic;
  signal selection__o_orderdate_valid : std_logic;
  signal selection__o_orderdate_ready : std_logic;
  signal selection__o_orderdate_data : std_logic_vector(25 downto 0);
  signal selection__o_orderdate_last : std_logic;
  signal selection__o_orderdate_strb : std_logic;
  signal selection__o_orderkey_valid : std_logic;
  signal selection__o_orderkey_ready : std_logic;
  signal selection__o_orderkey_data : std_logic_vector(31 downto 0);
  signal selection__o_orderkey_last : std_logic;
  signal selection__o_orderkey_strb : std_logic;
  signal selection__select_valid : std_logic;
  signal selection__select_ready : std_logic;
  signal selection__select_data : std_logic;
  signal selection__select_strb : std_logic;
begin
  duplicate_selection_select: test_project__tpch__duplicator_iATStreamIWselect_streamIMAT5_com port map(
    clk => clk,
    rst => rst,
    input_valid => duplicate_selection_select__input_valid,
    input_ready => duplicate_selection_select__input_ready,
    input_data => duplicate_selection_select__input_data,
    input_strb => duplicate_selection_select__input_strb,
    outputAT1_valid => duplicate_selection_select__outputAT1_valid,
    outputAT1_ready => duplicate_selection_select__outputAT1_ready,
    outputAT1_data => duplicate_selection_select__outputAT1_data,
    outputAT1_strb => duplicate_selection_select__outputAT1_strb,
    outputAT3_valid => duplicate_selection_select__outputAT3_valid,
    outputAT3_ready => duplicate_selection_select__outputAT3_ready,
    outputAT3_data => duplicate_selection_select__outputAT3_data,
    outputAT3_strb => duplicate_selection_select__outputAT3_strb,
    outputAT4_valid => duplicate_selection_select__outputAT4_valid,
    outputAT4_ready => duplicate_selection_select__outputAT4_ready,
    outputAT4_data => duplicate_selection_select__outputAT4_data,
    outputAT4_strb => duplicate_selection_select__outputAT4_strb,
    outputAT2_valid => duplicate_selection_select__outputAT2_valid,
    outputAT2_ready => duplicate_selection_select__outputAT2_ready,
    outputAT2_data => duplicate_selection_select__outputAT2_data,
    outputAT2_strb => duplicate_selection_select__outputAT2_strb,
    outputAT0_valid => duplicate_selection_select__outputAT0_valid,
    outputAT0_ready => duplicate_selection_select__outputAT0_ready,
    outputAT0_data => duplicate_selection_select__outputAT0_data,
    outputAT0_strb => duplicate_selection_select__outputAT0_strb
  );
  l_discount_filter: test_project__tpch__stream_filter_1bit_iATStreamIWSQL_decimal_15_2_streamIM_com port map(
    clk => clk,
    rst => rst,
    input_valid => l_discount_filter__input_valid,
    input_ready => l_discount_filter__input_ready,
    input_data => l_discount_filter__input_data,
    input_last => l_discount_filter__input_last,
    input_strb => l_discount_filter__input_strb,
    select_valid => l_discount_filter__select_valid,
    select_ready => l_discount_filter__select_ready,
    select_data => l_discount_filter__select_data,
    select_strb => l_discount_filter__select_strb,
    output_valid => l_discount_filter__output_valid,
    output_ready => l_discount_filter__output_ready,
    output_data => l_discount_filter__output_data,
    output_last => l_discount_filter__output_last,
    output_strb => l_discount_filter__output_strb
  );
  l_extendedprice_filter: test_project__tpch__stream_filter_1bit_iATStreamIWSQL_decimal_15_2_streamIM_com port map(
    clk => clk,
    rst => rst,
    input_valid => l_extendedprice_filter__input_valid,
    input_ready => l_extendedprice_filter__input_ready,
    input_data => l_extendedprice_filter__input_data,
    input_last => l_extendedprice_filter__input_last,
    input_strb => l_extendedprice_filter__input_strb,
    select_valid => l_extendedprice_filter__select_valid,
    select_ready => l_extendedprice_filter__select_ready,
    select_data => l_extendedprice_filter__select_data,
    select_strb => l_extendedprice_filter__select_strb,
    output_valid => l_extendedprice_filter__output_valid,
    output_ready => l_extendedprice_filter__output_ready,
    output_data => l_extendedprice_filter__output_data,
    output_last => l_extendedprice_filter__output_last,
    output_strb => l_extendedprice_filter__output_strb
  );
  l_orderkey_filter: test_project__tpch__stream_filter_1bit_iATStreamIWint_streamIM_com port map(
    clk => clk,
    rst => rst,
    input_valid => l_orderkey_filter__input_valid,
    input_ready => l_orderkey_filter__input_ready,
    input_data => l_orderkey_filter__input_data,
    input_last => l_orderkey_filter__input_last,
    input_strb => l_orderkey_filter__input_strb,
    select_valid => l_orderkey_filter__select_valid,
    select_ready => l_orderkey_filter__select_ready,
    select_data => l_orderkey_filter__select_data,
    select_strb => l_orderkey_filter__select_strb,
    output_valid => l_orderkey_filter__output_valid,
    output_ready => l_orderkey_filter__output_ready,
    output_data => l_orderkey_filter__output_data,
    output_last => l_orderkey_filter__output_last,
    output_strb => l_orderkey_filter__output_strb
  );
  o_orderdate_filter: test_project__tpch__stream_filter_1bit_iATStreamIWdate_streamIM_com port map(
    clk => clk,
    rst => rst,
    select_valid => o_orderdate_filter__select_valid,
    select_ready => o_orderdate_filter__select_ready,
    select_data => o_orderdate_filter__select_data,
    select_strb => o_orderdate_filter__select_strb,
    input_valid => o_orderdate_filter__input_valid,
    input_ready => o_orderdate_filter__input_ready,
    input_data => o_orderdate_filter__input_data,
    input_last => o_orderdate_filter__input_last,
    input_strb => o_orderdate_filter__input_strb,
    output_valid => o_orderdate_filter__output_valid,
    output_ready => o_orderdate_filter__output_ready,
    output_data => o_orderdate_filter__output_data,
    output_last => o_orderdate_filter__output_last,
    output_strb => o_orderdate_filter__output_strb
  );
  o_shippriority_filter: test_project__tpch__stream_filter_1bit_iATStreamIWint_streamIM_com port map(
    clk => clk,
    rst => rst,
    input_valid => o_shippriority_filter__input_valid,
    input_ready => o_shippriority_filter__input_ready,
    input_data => o_shippriority_filter__input_data,
    input_last => o_shippriority_filter__input_last,
    input_strb => o_shippriority_filter__input_strb,
    select_valid => o_shippriority_filter__select_valid,
    select_ready => o_shippriority_filter__select_ready,
    select_data => o_shippriority_filter__select_data,
    select_strb => o_shippriority_filter__select_strb,
    output_valid => o_shippriority_filter__output_valid,
    output_ready => o_shippriority_filter__output_ready,
    output_data => o_shippriority_filter__output_data,
    output_last => o_shippriority_filter__output_last,
    output_strb => o_shippriority_filter__output_strb
  );
  selection: test_project__tpch__where_claus_i_com port map(
    clk => clk,
    rst => rst,
    o_custkey_valid => selection__o_custkey_valid,
    o_custkey_ready => selection__o_custkey_ready,
    o_custkey_data => selection__o_custkey_data,
    o_custkey_last => selection__o_custkey_last,
    o_custkey_strb => selection__o_custkey_strb,
    c_custkey_valid => selection__c_custkey_valid,
    c_custkey_ready => selection__c_custkey_ready,
    c_custkey_data => selection__c_custkey_data,
    c_custkey_last => selection__c_custkey_last,
    c_custkey_strb => selection__c_custkey_strb,
    l_shipdate_valid => selection__l_shipdate_valid,
    l_shipdate_ready => selection__l_shipdate_ready,
    l_shipdate_data => selection__l_shipdate_data,
    l_shipdate_last => selection__l_shipdate_last,
    l_shipdate_strb => selection__l_shipdate_strb,
    l_orderkey_valid => selection__l_orderkey_valid,
    l_orderkey_ready => selection__l_orderkey_ready,
    l_orderkey_data => selection__l_orderkey_data,
    l_orderkey_last => selection__l_orderkey_last,
    l_orderkey_strb => selection__l_orderkey_strb,
    o_orderdate_valid => selection__o_orderdate_valid,
    o_orderdate_ready => selection__o_orderdate_ready,
    o_orderdate_data => selection__o_orderdate_data,
    o_orderdate_last => selection__o_orderdate_last,
    o_orderdate_strb => selection__o_orderdate_strb,
    o_orderkey_valid => selection__o_orderkey_valid,
    o_orderkey_ready => selection__o_orderkey_ready,
    o_orderkey_data => selection__o_orderkey_data,
    o_orderkey_last => selection__o_orderkey_last,
    o_orderkey_strb => selection__o_orderkey_strb,
    c_mktsegment_valid => selection__c_mktsegment_valid,
    c_mktsegment_ready => selection__c_mktsegment_ready,
    c_mktsegment_data => selection__c_mktsegment_data,
    c_mktsegment_last => selection__c_mktsegment_last,
    c_mktsegment_strb => selection__c_mktsegment_strb,
    select_valid => selection__select_valid,
    select_ready => selection__select_ready,
    select_data => selection__select_data,
    select_strb => selection__select_strb
  );
  duplicate_selection_select__input_valid <= selection__select_valid;
  selection__select_ready <= duplicate_selection_select__input_ready;
  duplicate_selection_select__input_data <= selection__select_data;
  duplicate_selection_select__input_strb <= selection__select_strb;
  o_shippriority_filter__select_valid <= duplicate_selection_select__outputAT0_valid;
  duplicate_selection_select__outputAT0_ready <= o_shippriority_filter__select_ready;
  o_shippriority_filter__select_data <= duplicate_selection_select__outputAT0_data;
  o_shippriority_filter__select_strb <= duplicate_selection_select__outputAT0_strb;
  o_shippriority_filter__input_valid <= o_shippriority_in_valid;
  o_shippriority_in_ready <= o_shippriority_filter__input_ready;
  o_shippriority_filter__input_data <= o_shippriority_in_data;
  o_shippriority_filter__input_last <= o_shippriority_in_last;
  o_shippriority_filter__input_strb <= o_shippriority_in_strb;
  l_extendedprice_out_valid <= l_extendedprice_filter__output_valid;
  l_extendedprice_filter__output_ready <= l_extendedprice_out_ready;
  l_extendedprice_out_data <= l_extendedprice_filter__output_data;
  l_extendedprice_out_last <= l_extendedprice_filter__output_last;
  l_extendedprice_out_strb <= l_extendedprice_filter__output_strb;
  selection__o_orderkey_valid <= o_orderkey_valid;
  o_orderkey_ready <= selection__o_orderkey_ready;
  selection__o_orderkey_data <= o_orderkey_data;
  selection__o_orderkey_last <= o_orderkey_last;
  selection__o_orderkey_strb <= o_orderkey_strb;
  selection__l_shipdate_valid <= l_shipdate_valid;
  l_shipdate_ready <= selection__l_shipdate_ready;
  selection__l_shipdate_data <= l_shipdate_data;
  selection__l_shipdate_last <= l_shipdate_last;
  selection__l_shipdate_strb <= l_shipdate_strb;
  l_extendedprice_filter__input_valid <= l_extendedprice_in_valid;
  l_extendedprice_in_ready <= l_extendedprice_filter__input_ready;
  l_extendedprice_filter__input_data <= l_extendedprice_in_data;
  l_extendedprice_filter__input_last <= l_extendedprice_in_last;
  l_extendedprice_filter__input_strb <= l_extendedprice_in_strb;
  l_extendedprice_filter__select_valid <= duplicate_selection_select__outputAT2_valid;
  duplicate_selection_select__outputAT2_ready <= l_extendedprice_filter__select_ready;
  l_extendedprice_filter__select_data <= duplicate_selection_select__outputAT2_data;
  l_extendedprice_filter__select_strb <= duplicate_selection_select__outputAT2_strb;
  selection__c_mktsegment_valid <= c_mktsegment_valid;
  c_mktsegment_ready <= selection__c_mktsegment_ready;
  selection__c_mktsegment_data <= c_mktsegment_data;
  selection__c_mktsegment_last <= c_mktsegment_last;
  selection__c_mktsegment_strb <= c_mktsegment_strb;
  selection__l_orderkey_valid <= l_orderkey_valid;
  l_orderkey_ready <= selection__l_orderkey_ready;
  selection__l_orderkey_data <= l_orderkey_data;
  selection__l_orderkey_last <= l_orderkey_last;
  selection__l_orderkey_strb <= l_orderkey_strb;
  o_orderdate_filter__select_valid <= duplicate_selection_select__outputAT3_valid;
  duplicate_selection_select__outputAT3_ready <= o_orderdate_filter__select_ready;
  o_orderdate_filter__select_data <= duplicate_selection_select__outputAT3_data;
  o_orderdate_filter__select_strb <= duplicate_selection_select__outputAT3_strb;
  l_orderkey_out_valid <= l_orderkey_filter__output_valid;
  l_orderkey_filter__output_ready <= l_orderkey_out_ready;
  l_orderkey_out_data <= l_orderkey_filter__output_data;
  l_orderkey_out_last <= l_orderkey_filter__output_last;
  l_orderkey_out_strb <= l_orderkey_filter__output_strb;
  l_orderkey_filter__input_valid <= l_orderkey_in_valid;
  l_orderkey_in_ready <= l_orderkey_filter__input_ready;
  l_orderkey_filter__input_data <= l_orderkey_in_data;
  l_orderkey_filter__input_last <= l_orderkey_in_last;
  l_orderkey_filter__input_strb <= l_orderkey_in_strb;
  l_discount_out_valid <= l_discount_filter__output_valid;
  l_discount_filter__output_ready <= l_discount_out_ready;
  l_discount_out_data <= l_discount_filter__output_data;
  l_discount_out_last <= l_discount_filter__output_last;
  l_discount_out_strb <= l_discount_filter__output_strb;
  o_orderdate_filter__input_valid <= o_orderdate_in_valid;
  o_orderdate_in_ready <= o_orderdate_filter__input_ready;
  o_orderdate_filter__input_data <= o_orderdate_in_data;
  o_orderdate_filter__input_last <= o_orderdate_in_last;
  o_orderdate_filter__input_strb <= o_orderdate_in_strb;
  l_discount_filter__select_valid <= duplicate_selection_select__outputAT4_valid;
  duplicate_selection_select__outputAT4_ready <= l_discount_filter__select_ready;
  l_discount_filter__select_data <= duplicate_selection_select__outputAT4_data;
  l_discount_filter__select_strb <= duplicate_selection_select__outputAT4_strb;
  o_orderdate_out_valid <= o_orderdate_filter__output_valid;
  o_orderdate_filter__output_ready <= o_orderdate_out_ready;
  o_orderdate_out_data <= o_orderdate_filter__output_data;
  o_orderdate_out_last <= o_orderdate_filter__output_last;
  o_orderdate_out_strb <= o_orderdate_filter__output_strb;
  o_shippriority_out_valid <= o_shippriority_filter__output_valid;
  o_shippriority_filter__output_ready <= o_shippriority_out_ready;
  o_shippriority_out_data <= o_shippriority_filter__output_data;
  o_shippriority_out_last <= o_shippriority_filter__output_last;
  o_shippriority_out_strb <= o_shippriority_filter__output_strb;
  selection__o_orderdate_valid <= o_orderdate_valid;
  o_orderdate_ready <= selection__o_orderdate_ready;
  selection__o_orderdate_data <= o_orderdate_data;
  selection__o_orderdate_last <= o_orderdate_last;
  selection__o_orderdate_strb <= o_orderdate_strb;
  l_orderkey_filter__select_valid <= duplicate_selection_select__outputAT1_valid;
  duplicate_selection_select__outputAT1_ready <= l_orderkey_filter__select_ready;
  l_orderkey_filter__select_data <= duplicate_selection_select__outputAT1_data;
  l_orderkey_filter__select_strb <= duplicate_selection_select__outputAT1_strb;
  l_discount_filter__input_valid <= l_discount_in_valid;
  l_discount_in_ready <= l_discount_filter__input_ready;
  l_discount_filter__input_data <= l_discount_in_data;
  l_discount_filter__input_last <= l_discount_in_last;
  l_discount_filter__input_strb <= l_discount_in_strb;
  selection__c_custkey_valid <= c_custkey_valid;
  c_custkey_ready <= selection__c_custkey_ready;
  selection__c_custkey_data <= c_custkey_data;
  selection__c_custkey_last <= c_custkey_last;
  selection__c_custkey_strb <= c_custkey_strb;
  selection__o_custkey_valid <= o_custkey_valid;
  o_custkey_ready <= selection__o_custkey_ready;
  selection__o_custkey_data <= o_custkey_data;
  selection__o_custkey_last <= o_custkey_last;
  selection__o_custkey_strb <= o_custkey_strb;
end test_project__tpch__data_filter_i;