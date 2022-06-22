library ieee;
use ieee.std_logic_1164.all;

library work;
use work.proj.all;

entity test_project__tpch__where_claus_i_com is
  port (
    clk : in std_logic;
    rst : in std_logic;
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
    l_shipdate_valid : in std_logic;
    l_shipdate_ready : out std_logic;
    l_shipdate_data : in std_logic_vector(25 downto 0);
    l_shipdate_last : in std_logic;
    l_shipdate_strb : in std_logic;
    l_orderkey_valid : in std_logic;
    l_orderkey_ready : out std_logic;
    l_orderkey_data : in std_logic_vector(31 downto 0);
    l_orderkey_last : in std_logic;
    l_orderkey_strb : in std_logic;
    o_orderdate_valid : in std_logic;
    o_orderdate_ready : out std_logic;
    o_orderdate_data : in std_logic_vector(25 downto 0);
    o_orderdate_last : in std_logic;
    o_orderdate_strb : in std_logic;
    o_orderkey_valid : in std_logic;
    o_orderkey_ready : out std_logic;
    o_orderkey_data : in std_logic_vector(31 downto 0);
    o_orderkey_last : in std_logic;
    o_orderkey_strb : in std_logic;
    c_mktsegment_valid : in std_logic;
    c_mktsegment_ready : out std_logic;
    c_mktsegment_data : in std_logic_vector(79 downto 0);
    c_mktsegment_last : in std_logic;
    c_mktsegment_strb : in std_logic;
    select_valid : out std_logic;
    select_ready : in std_logic;
    select_data : out std_logic;
    select_strb : out std_logic
  );
end test_project__tpch__where_claus_i_com;

architecture test_project__tpch__where_claus_i of test_project__tpch__where_claus_i_com is
  signal and_0__inputAT0_valid : std_logic;
  signal and_0__inputAT0_ready : std_logic;
  signal and_0__inputAT0_data : std_logic;
  signal and_0__inputAT0_strb : std_logic;
  signal and_0__inputAT1_valid : std_logic;
  signal and_0__inputAT1_ready : std_logic;
  signal and_0__inputAT1_data : std_logic;
  signal and_0__inputAT1_strb : std_logic;
  signal and_0__inputAT2_valid : std_logic;
  signal and_0__inputAT2_ready : std_logic;
  signal and_0__inputAT2_data : std_logic;
  signal and_0__inputAT2_strb : std_logic;
  signal and_0__inputAT3_valid : std_logic;
  signal and_0__inputAT3_ready : std_logic;
  signal and_0__inputAT3_data : std_logic;
  signal and_0__inputAT3_strb : std_logic;
  signal and_0__inputAT4_valid : std_logic;
  signal and_0__inputAT4_ready : std_logic;
  signal and_0__inputAT4_data : std_logic;
  signal and_0__inputAT4_strb : std_logic;
  signal and_0__output_valid : std_logic;
  signal and_0__output_ready : std_logic;
  signal and_0__output_data : std_logic;
  signal and_0__output_strb : std_logic;
  signal c_mktsegment_compare__input0_valid : std_logic;
  signal c_mktsegment_compare__input0_ready : std_logic;
  signal c_mktsegment_compare__input0_data : std_logic_vector(79 downto 0);
  signal c_mktsegment_compare__input0_last : std_logic;
  signal c_mktsegment_compare__input0_strb : std_logic;
  signal c_mktsegment_compare__input1_valid : std_logic;
  signal c_mktsegment_compare__input1_ready : std_logic;
  signal c_mktsegment_compare__input1_data : std_logic_vector(79 downto 0);
  signal c_mktsegment_compare__input1_last : std_logic;
  signal c_mktsegment_compare__input1_strb : std_logic;
  signal c_mktsegment_compare__output_valid : std_logic;
  signal c_mktsegment_compare__output_ready : std_logic;
  signal c_mktsegment_compare__output_data : std_logic;
  signal c_mktsegment_compare__output_strb : std_logic;
  signal c_mktsegment_standard_gen__output_valid : std_logic;
  signal c_mktsegment_standard_gen__output_ready : std_logic;
  signal c_mktsegment_standard_gen__output_data : std_logic_vector(79 downto 0);
  signal c_mktsegment_standard_gen__output_last : std_logic;
  signal c_mktsegment_standard_gen__output_strb : std_logic;
  signal compare0__input0_valid : std_logic;
  signal compare0__input0_ready : std_logic;
  signal compare0__input0_data : std_logic_vector(31 downto 0);
  signal compare0__input0_last : std_logic;
  signal compare0__input0_strb : std_logic;
  signal compare0__input1_valid : std_logic;
  signal compare0__input1_ready : std_logic;
  signal compare0__input1_data : std_logic_vector(31 downto 0);
  signal compare0__input1_last : std_logic;
  signal compare0__input1_strb : std_logic;
  signal compare0__output_valid : std_logic;
  signal compare0__output_ready : std_logic;
  signal compare0__output_data : std_logic;
  signal compare0__output_strb : std_logic;
  signal compare1__input0_valid : std_logic;
  signal compare1__input0_ready : std_logic;
  signal compare1__input0_data : std_logic_vector(31 downto 0);
  signal compare1__input0_last : std_logic;
  signal compare1__input0_strb : std_logic;
  signal compare1__input1_valid : std_logic;
  signal compare1__input1_ready : std_logic;
  signal compare1__input1_data : std_logic_vector(31 downto 0);
  signal compare1__input1_last : std_logic;
  signal compare1__input1_strb : std_logic;
  signal compare1__output_valid : std_logic;
  signal compare1__output_ready : std_logic;
  signal compare1__output_data : std_logic;
  signal compare1__output_strb : std_logic;
  signal compare2__input0_valid : std_logic;
  signal compare2__input0_ready : std_logic;
  signal compare2__input0_data : std_logic_vector(25 downto 0);
  signal compare2__input0_last : std_logic;
  signal compare2__input0_strb : std_logic;
  signal compare2__input1_valid : std_logic;
  signal compare2__input1_ready : std_logic;
  signal compare2__input1_data : std_logic_vector(25 downto 0);
  signal compare2__input1_last : std_logic;
  signal compare2__input1_strb : std_logic;
  signal compare2__output_valid : std_logic;
  signal compare2__output_ready : std_logic;
  signal compare2__output_data : std_logic;
  signal compare2__output_strb : std_logic;
  signal compare3__input0_valid : std_logic;
  signal compare3__input0_ready : std_logic;
  signal compare3__input0_data : std_logic_vector(25 downto 0);
  signal compare3__input0_last : std_logic;
  signal compare3__input0_strb : std_logic;
  signal compare3__input1_valid : std_logic;
  signal compare3__input1_ready : std_logic;
  signal compare3__input1_data : std_logic_vector(25 downto 0);
  signal compare3__input1_last : std_logic;
  signal compare3__input1_strb : std_logic;
  signal compare3__output_valid : std_logic;
  signal compare3__output_ready : std_logic;
  signal compare3__output_data : std_logic;
  signal compare3__output_strb : std_logic;
  signal l_shipdate_standard_gen__date_output_valid : std_logic;
  signal l_shipdate_standard_gen__date_output_ready : std_logic;
  signal l_shipdate_standard_gen__date_output_data : std_logic_vector(25 downto 0);
  signal l_shipdate_standard_gen__date_output_last : std_logic;
  signal l_shipdate_standard_gen__date_output_strb : std_logic;
  signal o_orderdate_standard_gen__date_output_valid : std_logic;
  signal o_orderdate_standard_gen__date_output_ready : std_logic;
  signal o_orderdate_standard_gen__date_output_data : std_logic_vector(25 downto 0);
  signal o_orderdate_standard_gen__date_output_last : std_logic;
  signal o_orderdate_standard_gen__date_output_strb : std_logic;
begin
  and_0: test_project__tpch__and_iATStreamIWselect_streamIMAT5_com port map(
    clk => clk,
    rst => rst,
    inputAT2_valid => and_0__inputAT2_valid,
    inputAT2_ready => and_0__inputAT2_ready,
    inputAT2_data => and_0__inputAT2_data,
    inputAT2_strb => and_0__inputAT2_strb,
    inputAT0_valid => and_0__inputAT0_valid,
    inputAT0_ready => and_0__inputAT0_ready,
    inputAT0_data => and_0__inputAT0_data,
    inputAT0_strb => and_0__inputAT0_strb,
    inputAT3_valid => and_0__inputAT3_valid,
    inputAT3_ready => and_0__inputAT3_ready,
    inputAT3_data => and_0__inputAT3_data,
    inputAT3_strb => and_0__inputAT3_strb,
    inputAT4_valid => and_0__inputAT4_valid,
    inputAT4_ready => and_0__inputAT4_ready,
    inputAT4_data => and_0__inputAT4_data,
    inputAT4_strb => and_0__inputAT4_strb,
    inputAT1_valid => and_0__inputAT1_valid,
    inputAT1_ready => and_0__inputAT1_ready,
    inputAT1_data => and_0__inputAT1_data,
    inputAT1_strb => and_0__inputAT1_strb,
    output_valid => and_0__output_valid,
    output_ready => and_0__output_ready,
    output_data => and_0__output_data,
    output_strb => and_0__output_strb
  );
  c_mktsegment_compare: test_project__tpch__comparator_is_equal_iATStreamIWSQL_char10_streamIM_com port map(
    clk => clk,
    rst => rst,
    input1_valid => c_mktsegment_compare__input1_valid,
    input1_ready => c_mktsegment_compare__input1_ready,
    input1_data => c_mktsegment_compare__input1_data,
    input1_last => c_mktsegment_compare__input1_last,
    input1_strb => c_mktsegment_compare__input1_strb,
    input0_valid => c_mktsegment_compare__input0_valid,
    input0_ready => c_mktsegment_compare__input0_ready,
    input0_data => c_mktsegment_compare__input0_data,
    input0_last => c_mktsegment_compare__input0_last,
    input0_strb => c_mktsegment_compare__input0_strb,
    output_valid => c_mktsegment_compare__output_valid,
    output_ready => c_mktsegment_compare__output_ready,
    output_data => c_mktsegment_compare__output_data,
    output_strb => c_mktsegment_compare__output_strb
  );
  c_mktsegment_standard_gen: test_project__tpch__const_value_generator_iATStreamIWSQL_char10_streamIMAT10_com port map(
    clk => clk,
    rst => rst,
    output_valid => c_mktsegment_standard_gen__output_valid,
    output_ready => c_mktsegment_standard_gen__output_ready,
    output_data => c_mktsegment_standard_gen__output_data,
    output_last => c_mktsegment_standard_gen__output_last,
    output_strb => c_mktsegment_standard_gen__output_strb
  );
  compare0: test_project__tpch__comparator_is_equal_iATStreamIWint_streamIM_com port map(
    clk => clk,
    rst => rst,
    input1_valid => compare0__input1_valid,
    input1_ready => compare0__input1_ready,
    input1_data => compare0__input1_data,
    input1_last => compare0__input1_last,
    input1_strb => compare0__input1_strb,
    input0_valid => compare0__input0_valid,
    input0_ready => compare0__input0_ready,
    input0_data => compare0__input0_data,
    input0_last => compare0__input0_last,
    input0_strb => compare0__input0_strb,
    output_valid => compare0__output_valid,
    output_ready => compare0__output_ready,
    output_data => compare0__output_data,
    output_strb => compare0__output_strb
  );
  compare1: test_project__tpch__comparator_is_equal_iATStreamIWint_streamIM_com port map(
    clk => clk,
    rst => rst,
    input1_valid => compare1__input1_valid,
    input1_ready => compare1__input1_ready,
    input1_data => compare1__input1_data,
    input1_last => compare1__input1_last,
    input1_strb => compare1__input1_strb,
    input0_valid => compare1__input0_valid,
    input0_ready => compare1__input0_ready,
    input0_data => compare1__input0_data,
    input0_last => compare1__input0_last,
    input0_strb => compare1__input0_strb,
    output_valid => compare1__output_valid,
    output_ready => compare1__output_ready,
    output_data => compare1__output_data,
    output_strb => compare1__output_strb
  );
  compare2: test_project__tpch__comparator_is_smaller_iATStreamIWdate_streamIM_com port map(
    clk => clk,
    rst => rst,
    input1_valid => compare2__input1_valid,
    input1_ready => compare2__input1_ready,
    input1_data => compare2__input1_data,
    input1_last => compare2__input1_last,
    input1_strb => compare2__input1_strb,
    input0_valid => compare2__input0_valid,
    input0_ready => compare2__input0_ready,
    input0_data => compare2__input0_data,
    input0_last => compare2__input0_last,
    input0_strb => compare2__input0_strb,
    output_valid => compare2__output_valid,
    output_ready => compare2__output_ready,
    output_data => compare2__output_data,
    output_strb => compare2__output_strb
  );
  compare3: test_project__tpch__comparator_is_larger_iATStreamIWdate_streamIM_com port map(
    clk => clk,
    rst => rst,
    input0_valid => compare3__input0_valid,
    input0_ready => compare3__input0_ready,
    input0_data => compare3__input0_data,
    input0_last => compare3__input0_last,
    input0_strb => compare3__input0_strb,
    input1_valid => compare3__input1_valid,
    input1_ready => compare3__input1_ready,
    input1_data => compare3__input1_data,
    input1_last => compare3__input1_last,
    input1_strb => compare3__input1_strb,
    output_valid => compare3__output_valid,
    output_ready => compare3__output_ready,
    output_data => compare3__output_data,
    output_strb => compare3__output_strb
  );
  l_shipdate_standard_gen: test_project__tpch__const_date_generator_iAT1AT1AT1998_com port map(
    clk => clk,
    rst => rst,
    date_output_valid => l_shipdate_standard_gen__date_output_valid,
    date_output_ready => l_shipdate_standard_gen__date_output_ready,
    date_output_data => l_shipdate_standard_gen__date_output_data,
    date_output_last => l_shipdate_standard_gen__date_output_last,
    date_output_strb => l_shipdate_standard_gen__date_output_strb
  );
  o_orderdate_standard_gen: test_project__tpch__const_date_generator_iAT1AT1AT1997_com port map(
    clk => clk,
    rst => rst,
    date_output_valid => o_orderdate_standard_gen__date_output_valid,
    date_output_ready => o_orderdate_standard_gen__date_output_ready,
    date_output_data => o_orderdate_standard_gen__date_output_data,
    date_output_last => o_orderdate_standard_gen__date_output_last,
    date_output_strb => o_orderdate_standard_gen__date_output_strb
  );
  compare3__input0_valid <= l_shipdate_valid;
  l_shipdate_ready <= compare3__input0_ready;
  compare3__input0_data <= l_shipdate_data;
  compare3__input0_last <= l_shipdate_last;
  compare3__input0_strb <= l_shipdate_strb;
  c_mktsegment_compare__input0_valid <= c_mktsegment_standard_gen__output_valid;
  c_mktsegment_standard_gen__output_ready <= c_mktsegment_compare__input0_ready;
  c_mktsegment_compare__input0_data <= c_mktsegment_standard_gen__output_data;
  c_mktsegment_compare__input0_last <= c_mktsegment_standard_gen__output_last;
  c_mktsegment_compare__input0_strb <= c_mktsegment_standard_gen__output_strb;
  and_0__inputAT2_valid <= compare1__output_valid;
  compare1__output_ready <= and_0__inputAT2_ready;
  and_0__inputAT2_data <= compare1__output_data;
  and_0__inputAT2_strb <= compare1__output_strb;
  compare1__input0_valid <= l_orderkey_valid;
  l_orderkey_ready <= compare1__input0_ready;
  compare1__input0_data <= l_orderkey_data;
  compare1__input0_last <= l_orderkey_last;
  compare1__input0_strb <= l_orderkey_strb;
  select_valid <= and_0__output_valid;
  and_0__output_ready <= select_ready;
  select_data <= and_0__output_data;
  select_strb <= and_0__output_strb;
  compare0__input1_valid <= o_custkey_valid;
  o_custkey_ready <= compare0__input1_ready;
  compare0__input1_data <= o_custkey_data;
  compare0__input1_last <= o_custkey_last;
  compare0__input1_strb <= o_custkey_strb;
  and_0__inputAT4_valid <= compare3__output_valid;
  compare3__output_ready <= and_0__inputAT4_ready;
  and_0__inputAT4_data <= compare3__output_data;
  and_0__inputAT4_strb <= compare3__output_strb;
  compare2__input0_valid <= o_orderdate_valid;
  o_orderdate_ready <= compare2__input0_ready;
  compare2__input0_data <= o_orderdate_data;
  compare2__input0_last <= o_orderdate_last;
  compare2__input0_strb <= o_orderdate_strb;
  c_mktsegment_compare__input1_valid <= c_mktsegment_valid;
  c_mktsegment_ready <= c_mktsegment_compare__input1_ready;
  c_mktsegment_compare__input1_data <= c_mktsegment_data;
  c_mktsegment_compare__input1_last <= c_mktsegment_last;
  c_mktsegment_compare__input1_strb <= c_mktsegment_strb;
  compare2__input1_valid <= o_orderdate_standard_gen__date_output_valid;
  o_orderdate_standard_gen__date_output_ready <= compare2__input1_ready;
  compare2__input1_data <= o_orderdate_standard_gen__date_output_data;
  compare2__input1_last <= o_orderdate_standard_gen__date_output_last;
  compare2__input1_strb <= o_orderdate_standard_gen__date_output_strb;
  compare3__input1_valid <= l_shipdate_standard_gen__date_output_valid;
  l_shipdate_standard_gen__date_output_ready <= compare3__input1_ready;
  compare3__input1_data <= l_shipdate_standard_gen__date_output_data;
  compare3__input1_last <= l_shipdate_standard_gen__date_output_last;
  compare3__input1_strb <= l_shipdate_standard_gen__date_output_strb;
  and_0__inputAT0_valid <= c_mktsegment_compare__output_valid;
  c_mktsegment_compare__output_ready <= and_0__inputAT0_ready;
  and_0__inputAT0_data <= c_mktsegment_compare__output_data;
  and_0__inputAT0_strb <= c_mktsegment_compare__output_strb;
  compare0__input0_valid <= c_custkey_valid;
  c_custkey_ready <= compare0__input0_ready;
  compare0__input0_data <= c_custkey_data;
  compare0__input0_last <= c_custkey_last;
  compare0__input0_strb <= c_custkey_strb;
  and_0__inputAT1_valid <= compare0__output_valid;
  compare0__output_ready <= and_0__inputAT1_ready;
  and_0__inputAT1_data <= compare0__output_data;
  and_0__inputAT1_strb <= compare0__output_strb;
  and_0__inputAT3_valid <= compare2__output_valid;
  compare2__output_ready <= and_0__inputAT3_ready;
  and_0__inputAT3_data <= compare2__output_data;
  and_0__inputAT3_strb <= compare2__output_strb;
  compare1__input1_valid <= o_orderkey_valid;
  o_orderkey_ready <= compare1__input1_ready;
  compare1__input1_data <= o_orderkey_data;
  compare1__input1_last <= o_orderkey_last;
  compare1__input1_strb <= o_orderkey_strb;
end test_project__tpch__where_claus_i;