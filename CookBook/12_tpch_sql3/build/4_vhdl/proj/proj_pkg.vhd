library ieee;
use ieee.std_logic_1164.all;

package proj is

  component test_project__tpch__accumulator_iATStreamIWSQL_decimal_15_2_streamIM_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      input_valid : in std_logic;
      input_ready : out std_logic;
      input_data : in std_logic_vector(49 downto 0);
      input_last : in std_logic;
      input_strb : in std_logic;
      count_valid : out std_logic;
      count_ready : in std_logic;
      count_data : out std_logic_vector(31 downto 0);
      count_strb : out std_logic;
      output_valid : out std_logic;
      output_ready : in std_logic;
      output_data : out std_logic_vector(49 downto 0);
      output_last : out std_logic;
      output_strb : out std_logic;
      overflow_valid : out std_logic;
      overflow_ready : in std_logic;
      overflow_data : out std_logic;
      overflow_strb : out std_logic
    );
  end component;

  component test_project__tpch__accumulator_sATStreamIWSQL_decimal_15_2_streamIM_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      input_valid : in std_logic;
      input_ready : out std_logic;
      input_data : in std_logic_vector(49 downto 0);
      input_last : in std_logic;
      input_strb : in std_logic;
      count_valid : out std_logic;
      count_ready : in std_logic;
      count_data : out std_logic_vector(31 downto 0);
      count_strb : out std_logic;
      output_valid : out std_logic;
      output_ready : in std_logic;
      output_data : out std_logic_vector(49 downto 0);
      output_last : out std_logic;
      output_strb : out std_logic;
      overflow_valid : out std_logic;
      overflow_ready : in std_logic;
      overflow_data : out std_logic;
      overflow_strb : out std_logic
    );
  end component;

  component test_project__tpch__adder_iATStreamIWSQL_decimal_15_2_streamIM_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      input1_valid : in std_logic;
      input1_ready : out std_logic;
      input1_data : in std_logic_vector(49 downto 0);
      input1_last : in std_logic;
      input1_strb : in std_logic;
      input0_valid : in std_logic;
      input0_ready : out std_logic;
      input0_data : in std_logic_vector(49 downto 0);
      input0_last : in std_logic;
      input0_strb : in std_logic;
      overflow_valid : out std_logic;
      overflow_ready : in std_logic;
      overflow_data : out std_logic;
      overflow_strb : out std_logic;
      output_valid : out std_logic;
      output_ready : in std_logic;
      output_data : out std_logic_vector(49 downto 0);
      output_last : out std_logic;
      output_strb : out std_logic
    );
  end component;

  component test_project__tpch__adder_sATStreamIWSQL_decimal_15_2_streamIM_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      input1_valid : in std_logic;
      input1_ready : out std_logic;
      input1_data : in std_logic_vector(49 downto 0);
      input1_last : in std_logic;
      input1_strb : in std_logic;
      input0_valid : in std_logic;
      input0_ready : out std_logic;
      input0_data : in std_logic_vector(49 downto 0);
      input0_last : in std_logic;
      input0_strb : in std_logic;
      overflow_valid : out std_logic;
      overflow_ready : in std_logic;
      overflow_data : out std_logic;
      overflow_strb : out std_logic;
      output_valid : out std_logic;
      output_ready : in std_logic;
      output_data : out std_logic_vector(49 downto 0);
      output_last : out std_logic;
      output_strb : out std_logic
    );
  end component;

  component test_project__tpch__and_iATStreamIWerr_streamIMAT3_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      inputAT1_valid : in std_logic;
      inputAT1_ready : out std_logic;
      inputAT1_data : in std_logic;
      inputAT1_strb : in std_logic;
      inputAT2_valid : in std_logic;
      inputAT2_ready : out std_logic;
      inputAT2_data : in std_logic;
      inputAT2_strb : in std_logic;
      inputAT0_valid : in std_logic;
      inputAT0_ready : out std_logic;
      inputAT0_data : in std_logic;
      inputAT0_strb : in std_logic;
      output_valid : out std_logic;
      output_ready : in std_logic;
      output_data : out std_logic;
      output_strb : out std_logic
    );
  end component;

  component test_project__tpch__and_iATStreamIWselect_streamIMAT5_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      inputAT2_valid : in std_logic;
      inputAT2_ready : out std_logic;
      inputAT2_data : in std_logic;
      inputAT2_strb : in std_logic;
      inputAT0_valid : in std_logic;
      inputAT0_ready : out std_logic;
      inputAT0_data : in std_logic;
      inputAT0_strb : in std_logic;
      inputAT3_valid : in std_logic;
      inputAT3_ready : out std_logic;
      inputAT3_data : in std_logic;
      inputAT3_strb : in std_logic;
      inputAT4_valid : in std_logic;
      inputAT4_ready : out std_logic;
      inputAT4_data : in std_logic;
      inputAT4_strb : in std_logic;
      inputAT1_valid : in std_logic;
      inputAT1_ready : out std_logic;
      inputAT1_data : in std_logic;
      inputAT1_strb : in std_logic;
      output_valid : out std_logic;
      output_ready : in std_logic;
      output_data : out std_logic;
      output_strb : out std_logic
    );
  end component;

  component test_project__tpch__and_sATStreamIWerr_streamIMAT3_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      inputAT1_valid : in std_logic;
      inputAT1_ready : out std_logic;
      inputAT1_data : in std_logic;
      inputAT1_strb : in std_logic;
      inputAT2_valid : in std_logic;
      inputAT2_ready : out std_logic;
      inputAT2_data : in std_logic;
      inputAT2_strb : in std_logic;
      inputAT0_valid : in std_logic;
      inputAT0_ready : out std_logic;
      inputAT0_data : in std_logic;
      inputAT0_strb : in std_logic;
      output_valid : out std_logic;
      output_ready : in std_logic;
      output_data : out std_logic;
      output_strb : out std_logic
    );
  end component;

  component test_project__tpch__and_sATStreamIWselect_streamIMAT5_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      inputAT2_valid : in std_logic;
      inputAT2_ready : out std_logic;
      inputAT2_data : in std_logic;
      inputAT2_strb : in std_logic;
      inputAT0_valid : in std_logic;
      inputAT0_ready : out std_logic;
      inputAT0_data : in std_logic;
      inputAT0_strb : in std_logic;
      inputAT3_valid : in std_logic;
      inputAT3_ready : out std_logic;
      inputAT3_data : in std_logic;
      inputAT3_strb : in std_logic;
      inputAT4_valid : in std_logic;
      inputAT4_ready : out std_logic;
      inputAT4_data : in std_logic;
      inputAT4_strb : in std_logic;
      inputAT1_valid : in std_logic;
      inputAT1_ready : out std_logic;
      inputAT1_data : in std_logic;
      inputAT1_strb : in std_logic;
      output_valid : out std_logic;
      output_ready : in std_logic;
      output_data : out std_logic;
      output_strb : out std_logic
    );
  end component;

  component test_project__tpch__comparator_is_equal_iATStreamIWSQL_char10_streamIM_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      input1_valid : in std_logic;
      input1_ready : out std_logic;
      input1_data : in std_logic_vector(79 downto 0);
      input1_last : in std_logic;
      input1_strb : in std_logic;
      input0_valid : in std_logic;
      input0_ready : out std_logic;
      input0_data : in std_logic_vector(79 downto 0);
      input0_last : in std_logic;
      input0_strb : in std_logic;
      output_valid : out std_logic;
      output_ready : in std_logic;
      output_data : out std_logic;
      output_strb : out std_logic
    );
  end component;

  component test_project__tpch__comparator_is_equal_iATStreamIWint_streamIM_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      input1_valid : in std_logic;
      input1_ready : out std_logic;
      input1_data : in std_logic_vector(31 downto 0);
      input1_last : in std_logic;
      input1_strb : in std_logic;
      input0_valid : in std_logic;
      input0_ready : out std_logic;
      input0_data : in std_logic_vector(31 downto 0);
      input0_last : in std_logic;
      input0_strb : in std_logic;
      output_valid : out std_logic;
      output_ready : in std_logic;
      output_data : out std_logic;
      output_strb : out std_logic
    );
  end component;

  component test_project__tpch__comparator_is_equal_sATStreamIWSQL_char10_streamIM_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      input1_valid : in std_logic;
      input1_ready : out std_logic;
      input1_data : in std_logic_vector(79 downto 0);
      input1_last : in std_logic;
      input1_strb : in std_logic;
      input0_valid : in std_logic;
      input0_ready : out std_logic;
      input0_data : in std_logic_vector(79 downto 0);
      input0_last : in std_logic;
      input0_strb : in std_logic;
      output_valid : out std_logic;
      output_ready : in std_logic;
      output_data : out std_logic;
      output_strb : out std_logic
    );
  end component;

  component test_project__tpch__comparator_is_equal_sATStreamIWint_streamIM_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      input1_valid : in std_logic;
      input1_ready : out std_logic;
      input1_data : in std_logic_vector(31 downto 0);
      input1_last : in std_logic;
      input1_strb : in std_logic;
      input0_valid : in std_logic;
      input0_ready : out std_logic;
      input0_data : in std_logic_vector(31 downto 0);
      input0_last : in std_logic;
      input0_strb : in std_logic;
      output_valid : out std_logic;
      output_ready : in std_logic;
      output_data : out std_logic;
      output_strb : out std_logic
    );
  end component;

  component test_project__tpch__comparator_is_larger_iATStreamIWdate_streamIM_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      input0_valid : in std_logic;
      input0_ready : out std_logic;
      input0_data : in std_logic_vector(25 downto 0);
      input0_last : in std_logic;
      input0_strb : in std_logic;
      input1_valid : in std_logic;
      input1_ready : out std_logic;
      input1_data : in std_logic_vector(25 downto 0);
      input1_last : in std_logic;
      input1_strb : in std_logic;
      output_valid : out std_logic;
      output_ready : in std_logic;
      output_data : out std_logic;
      output_strb : out std_logic
    );
  end component;

  component test_project__tpch__comparator_is_larger_sATStreamIWdate_streamIM_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      input0_valid : in std_logic;
      input0_ready : out std_logic;
      input0_data : in std_logic_vector(25 downto 0);
      input0_last : in std_logic;
      input0_strb : in std_logic;
      input1_valid : in std_logic;
      input1_ready : out std_logic;
      input1_data : in std_logic_vector(25 downto 0);
      input1_last : in std_logic;
      input1_strb : in std_logic;
      output_valid : out std_logic;
      output_ready : in std_logic;
      output_data : out std_logic;
      output_strb : out std_logic
    );
  end component;

  component test_project__tpch__comparator_is_smaller_iATStreamIWdate_streamIM_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      input1_valid : in std_logic;
      input1_ready : out std_logic;
      input1_data : in std_logic_vector(25 downto 0);
      input1_last : in std_logic;
      input1_strb : in std_logic;
      input0_valid : in std_logic;
      input0_ready : out std_logic;
      input0_data : in std_logic_vector(25 downto 0);
      input0_last : in std_logic;
      input0_strb : in std_logic;
      output_valid : out std_logic;
      output_ready : in std_logic;
      output_data : out std_logic;
      output_strb : out std_logic
    );
  end component;

  component test_project__tpch__comparator_is_smaller_sATStreamIWdate_streamIM_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      input1_valid : in std_logic;
      input1_ready : out std_logic;
      input1_data : in std_logic_vector(25 downto 0);
      input1_last : in std_logic;
      input1_strb : in std_logic;
      input0_valid : in std_logic;
      input0_ready : out std_logic;
      input0_data : in std_logic_vector(25 downto 0);
      input0_last : in std_logic;
      input0_strb : in std_logic;
      output_valid : out std_logic;
      output_ready : in std_logic;
      output_data : out std_logic;
      output_strb : out std_logic
    );
  end component;

  component test_project__tpch__const_date_generator_iAT1AT1AT1997_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      date_output_valid : out std_logic;
      date_output_ready : in std_logic;
      date_output_data : out std_logic_vector(25 downto 0);
      date_output_last : out std_logic;
      date_output_strb : out std_logic
    );
  end component;

  component test_project__tpch__const_date_generator_iAT1AT1AT1998_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      date_output_valid : out std_logic;
      date_output_ready : in std_logic;
      date_output_data : out std_logic_vector(25 downto 0);
      date_output_last : out std_logic;
      date_output_strb : out std_logic
    );
  end component;

  component test_project__tpch__const_date_generator_s_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      date_output_valid : out std_logic;
      date_output_ready : in std_logic;
      date_output_data : out std_logic_vector(25 downto 0);
      date_output_last : out std_logic;
      date_output_strb : out std_logic
    );
  end component;

  component test_project__tpch__const_value_generator_iATStreamIWSQL_char10_streamIMAT10_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      output_valid : out std_logic;
      output_ready : in std_logic;
      output_data : out std_logic_vector(79 downto 0);
      output_last : out std_logic;
      output_strb : out std_logic
    );
  end component;

  component test_project__tpch__const_value_generator_iATStreamIWSQL_decimal_15_2_streamIMAT1_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      output_valid : out std_logic;
      output_ready : in std_logic;
      output_data : out std_logic_vector(49 downto 0);
      output_last : out std_logic;
      output_strb : out std_logic
    );
  end component;

  component test_project__tpch__const_value_generator_iATStreamIWday_streamIMAT1_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      output_valid : out std_logic;
      output_ready : in std_logic;
      output_data : out std_logic_vector(4 downto 0);
      output_strb : out std_logic
    );
  end component;

  component test_project__tpch__const_value_generator_iATStreamIWmonth_streamIMAT1_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      output_valid : out std_logic;
      output_ready : in std_logic;
      output_data : out std_logic_vector(3 downto 0);
      output_strb : out std_logic
    );
  end component;

  component test_project__tpch__const_value_generator_iATStreamIWyear_streamIMAT1997_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      output_valid : out std_logic;
      output_ready : in std_logic;
      output_data : out std_logic_vector(16 downto 0);
      output_strb : out std_logic
    );
  end component;

  component test_project__tpch__const_value_generator_iATStreamIWyear_streamIMAT1998_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      output_valid : out std_logic;
      output_ready : in std_logic;
      output_data : out std_logic_vector(16 downto 0);
      output_strb : out std_logic
    );
  end component;

  component test_project__tpch__const_value_generator_sATStreamIWSQL_char10_streamIMAT10_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      output_valid : out std_logic;
      output_ready : in std_logic;
      output_data : out std_logic_vector(79 downto 0);
      output_last : out std_logic;
      output_strb : out std_logic
    );
  end component;

  component test_project__tpch__const_value_generator_sATStreamIWSQL_decimal_15_2_streamIMAT1_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      output_valid : out std_logic;
      output_ready : in std_logic;
      output_data : out std_logic_vector(49 downto 0);
      output_last : out std_logic;
      output_strb : out std_logic
    );
  end component;

  component test_project__tpch__const_value_generator_sATStreamIWday_streamIMAT1_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      output_valid : out std_logic;
      output_ready : in std_logic;
      output_data : out std_logic_vector(4 downto 0);
      output_strb : out std_logic
    );
  end component;

  component test_project__tpch__const_value_generator_sATStreamIWmonth_streamIMAT1_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      output_valid : out std_logic;
      output_ready : in std_logic;
      output_data : out std_logic_vector(3 downto 0);
      output_strb : out std_logic
    );
  end component;

  component test_project__tpch__const_value_generator_sATStreamIWyear_streamIMAT1997_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      output_valid : out std_logic;
      output_ready : in std_logic;
      output_data : out std_logic_vector(16 downto 0);
      output_strb : out std_logic
    );
  end component;

  component test_project__tpch__const_value_generator_sATStreamIWyear_streamIMAT1998_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      output_valid : out std_logic;
      output_ready : in std_logic;
      output_data : out std_logic_vector(16 downto 0);
      output_strb : out std_logic
    );
  end component;

  component test_project__tpch__customer_i_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      c_custkey_valid : in std_logic;
      c_custkey_ready : out std_logic;
      c_custkey_data : in std_logic_vector(31 downto 0);
      c_custkey_last : in std_logic;
      c_custkey_strb : in std_logic;
      c_nationkey_valid : out std_logic;
      c_nationkey_ready : in std_logic;
      c_nationkey_data : out std_logic_vector(31 downto 0);
      c_nationkey_last : out std_logic;
      c_nationkey_strb : out std_logic;
      c_comment_valid : out std_logic;
      c_comment_ready : in std_logic;
      c_comment_data : out std_logic_vector(7 downto 0);
      c_comment_last : out std_logic_vector(1 downto 0);
      c_comment_strb : out std_logic;
      c_mktsegment_valid : out std_logic;
      c_mktsegment_ready : in std_logic;
      c_mktsegment_data : out std_logic_vector(79 downto 0);
      c_mktsegment_last : out std_logic;
      c_mktsegment_strb : out std_logic;
      c_address_valid : out std_logic;
      c_address_ready : in std_logic;
      c_address_data : out std_logic_vector(7 downto 0);
      c_address_last : out std_logic_vector(1 downto 0);
      c_address_strb : out std_logic;
      c_phone_valid : out std_logic;
      c_phone_ready : in std_logic;
      c_phone_data : out std_logic_vector(119 downto 0);
      c_phone_last : out std_logic;
      c_phone_strb : out std_logic;
      c_name_valid : out std_logic;
      c_name_ready : in std_logic;
      c_name_data : out std_logic_vector(7 downto 0);
      c_name_last : out std_logic_vector(1 downto 0);
      c_name_strb : out std_logic;
      c_acctbal_valid : out std_logic;
      c_acctbal_ready : in std_logic;
      c_acctbal_data : out std_logic_vector(49 downto 0);
      c_acctbal_last : out std_logic;
      c_acctbal_strb : out std_logic
    );
  end component;

  component test_project__tpch__customer_s_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      c_custkey_valid : in std_logic;
      c_custkey_ready : out std_logic;
      c_custkey_data : in std_logic_vector(31 downto 0);
      c_custkey_last : in std_logic;
      c_custkey_strb : in std_logic;
      c_nationkey_valid : out std_logic;
      c_nationkey_ready : in std_logic;
      c_nationkey_data : out std_logic_vector(31 downto 0);
      c_nationkey_last : out std_logic;
      c_nationkey_strb : out std_logic;
      c_comment_valid : out std_logic;
      c_comment_ready : in std_logic;
      c_comment_data : out std_logic_vector(7 downto 0);
      c_comment_last : out std_logic_vector(1 downto 0);
      c_comment_strb : out std_logic;
      c_mktsegment_valid : out std_logic;
      c_mktsegment_ready : in std_logic;
      c_mktsegment_data : out std_logic_vector(79 downto 0);
      c_mktsegment_last : out std_logic;
      c_mktsegment_strb : out std_logic;
      c_address_valid : out std_logic;
      c_address_ready : in std_logic;
      c_address_data : out std_logic_vector(7 downto 0);
      c_address_last : out std_logic_vector(1 downto 0);
      c_address_strb : out std_logic;
      c_phone_valid : out std_logic;
      c_phone_ready : in std_logic;
      c_phone_data : out std_logic_vector(119 downto 0);
      c_phone_last : out std_logic;
      c_phone_strb : out std_logic;
      c_name_valid : out std_logic;
      c_name_ready : in std_logic;
      c_name_data : out std_logic_vector(7 downto 0);
      c_name_last : out std_logic_vector(1 downto 0);
      c_name_strb : out std_logic;
      c_acctbal_valid : out std_logic;
      c_acctbal_ready : in std_logic;
      c_acctbal_data : out std_logic_vector(49 downto 0);
      c_acctbal_last : out std_logic;
      c_acctbal_strb : out std_logic
    );
  end component;

  component test_project__tpch__data_filter_i_com
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
  end component;

  component test_project__tpch__data_filter_s_com
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
  end component;

  component test_project__tpch__duplicator_iATStreamIWdate_streamIMAT2_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      input_valid : in std_logic;
      input_ready : out std_logic;
      input_data : in std_logic_vector(25 downto 0);
      input_last : in std_logic;
      input_strb : in std_logic;
      outputAT1_valid : out std_logic;
      outputAT1_ready : in std_logic;
      outputAT1_data : out std_logic_vector(25 downto 0);
      outputAT1_last : out std_logic;
      outputAT1_strb : out std_logic;
      outputAT0_valid : out std_logic;
      outputAT0_ready : in std_logic;
      outputAT0_data : out std_logic_vector(25 downto 0);
      outputAT0_last : out std_logic;
      outputAT0_strb : out std_logic
    );
  end component;

  component test_project__tpch__duplicator_iATStreamIWint_streamIMAT2_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      input_valid : in std_logic;
      input_ready : out std_logic;
      input_data : in std_logic_vector(31 downto 0);
      input_last : in std_logic;
      input_strb : in std_logic;
      outputAT1_valid : out std_logic;
      outputAT1_ready : in std_logic;
      outputAT1_data : out std_logic_vector(31 downto 0);
      outputAT1_last : out std_logic;
      outputAT1_strb : out std_logic;
      outputAT0_valid : out std_logic;
      outputAT0_ready : in std_logic;
      outputAT0_data : out std_logic_vector(31 downto 0);
      outputAT0_last : out std_logic;
      outputAT0_strb : out std_logic
    );
  end component;

  component test_project__tpch__duplicator_iATStreamIWint_streamIMAT3_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      input_valid : in std_logic;
      input_ready : out std_logic;
      input_data : in std_logic_vector(31 downto 0);
      input_last : in std_logic;
      input_strb : in std_logic;
      outputAT0_valid : out std_logic;
      outputAT0_ready : in std_logic;
      outputAT0_data : out std_logic_vector(31 downto 0);
      outputAT0_last : out std_logic;
      outputAT0_strb : out std_logic;
      outputAT1_valid : out std_logic;
      outputAT1_ready : in std_logic;
      outputAT1_data : out std_logic_vector(31 downto 0);
      outputAT1_last : out std_logic;
      outputAT1_strb : out std_logic;
      outputAT2_valid : out std_logic;
      outputAT2_ready : in std_logic;
      outputAT2_data : out std_logic_vector(31 downto 0);
      outputAT2_last : out std_logic;
      outputAT2_strb : out std_logic
    );
  end component;

  component test_project__tpch__duplicator_iATStreamIWselect_streamIMAT5_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      input_valid : in std_logic;
      input_ready : out std_logic;
      input_data : in std_logic;
      input_strb : in std_logic;
      outputAT1_valid : out std_logic;
      outputAT1_ready : in std_logic;
      outputAT1_data : out std_logic;
      outputAT1_strb : out std_logic;
      outputAT3_valid : out std_logic;
      outputAT3_ready : in std_logic;
      outputAT3_data : out std_logic;
      outputAT3_strb : out std_logic;
      outputAT4_valid : out std_logic;
      outputAT4_ready : in std_logic;
      outputAT4_data : out std_logic;
      outputAT4_strb : out std_logic;
      outputAT2_valid : out std_logic;
      outputAT2_ready : in std_logic;
      outputAT2_data : out std_logic;
      outputAT2_strb : out std_logic;
      outputAT0_valid : out std_logic;
      outputAT0_ready : in std_logic;
      outputAT0_data : out std_logic;
      outputAT0_strb : out std_logic
    );
  end component;

  component test_project__tpch__duplicator_sATStreamIWdate_streamIMAT2_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      input_valid : in std_logic;
      input_ready : out std_logic;
      input_data : in std_logic_vector(25 downto 0);
      input_last : in std_logic;
      input_strb : in std_logic;
      outputAT1_valid : out std_logic;
      outputAT1_ready : in std_logic;
      outputAT1_data : out std_logic_vector(25 downto 0);
      outputAT1_last : out std_logic;
      outputAT1_strb : out std_logic;
      outputAT0_valid : out std_logic;
      outputAT0_ready : in std_logic;
      outputAT0_data : out std_logic_vector(25 downto 0);
      outputAT0_last : out std_logic;
      outputAT0_strb : out std_logic
    );
  end component;

  component test_project__tpch__duplicator_sATStreamIWint_streamIMAT2_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      input_valid : in std_logic;
      input_ready : out std_logic;
      input_data : in std_logic_vector(31 downto 0);
      input_last : in std_logic;
      input_strb : in std_logic;
      outputAT1_valid : out std_logic;
      outputAT1_ready : in std_logic;
      outputAT1_data : out std_logic_vector(31 downto 0);
      outputAT1_last : out std_logic;
      outputAT1_strb : out std_logic;
      outputAT0_valid : out std_logic;
      outputAT0_ready : in std_logic;
      outputAT0_data : out std_logic_vector(31 downto 0);
      outputAT0_last : out std_logic;
      outputAT0_strb : out std_logic
    );
  end component;

  component test_project__tpch__duplicator_sATStreamIWint_streamIMAT3_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      input_valid : in std_logic;
      input_ready : out std_logic;
      input_data : in std_logic_vector(31 downto 0);
      input_last : in std_logic;
      input_strb : in std_logic;
      outputAT0_valid : out std_logic;
      outputAT0_ready : in std_logic;
      outputAT0_data : out std_logic_vector(31 downto 0);
      outputAT0_last : out std_logic;
      outputAT0_strb : out std_logic;
      outputAT1_valid : out std_logic;
      outputAT1_ready : in std_logic;
      outputAT1_data : out std_logic_vector(31 downto 0);
      outputAT1_last : out std_logic;
      outputAT1_strb : out std_logic;
      outputAT2_valid : out std_logic;
      outputAT2_ready : in std_logic;
      outputAT2_data : out std_logic_vector(31 downto 0);
      outputAT2_last : out std_logic;
      outputAT2_strb : out std_logic
    );
  end component;

  component test_project__tpch__duplicator_sATStreamIWselect_streamIMAT5_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      input_valid : in std_logic;
      input_ready : out std_logic;
      input_data : in std_logic;
      input_strb : in std_logic;
      outputAT1_valid : out std_logic;
      outputAT1_ready : in std_logic;
      outputAT1_data : out std_logic;
      outputAT1_strb : out std_logic;
      outputAT3_valid : out std_logic;
      outputAT3_ready : in std_logic;
      outputAT3_data : out std_logic;
      outputAT3_strb : out std_logic;
      outputAT4_valid : out std_logic;
      outputAT4_ready : in std_logic;
      outputAT4_data : out std_logic;
      outputAT4_strb : out std_logic;
      outputAT2_valid : out std_logic;
      outputAT2_ready : in std_logic;
      outputAT2_data : out std_logic;
      outputAT2_strb : out std_logic;
      outputAT0_valid : out std_logic;
      outputAT0_ready : in std_logic;
      outputAT0_data : out std_logic;
      outputAT0_strb : out std_logic
    );
  end component;

  component test_project__tpch__lineitem_i_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      l_orderkey_valid : in std_logic;
      l_orderkey_ready : out std_logic;
      l_orderkey_data : in std_logic_vector(31 downto 0);
      l_orderkey_last : in std_logic;
      l_orderkey_strb : in std_logic;
      l_linenumber_valid : in std_logic;
      l_linenumber_ready : out std_logic;
      l_linenumber_data : in std_logic_vector(31 downto 0);
      l_linenumber_last : in std_logic;
      l_linenumber_strb : in std_logic;
      l_returnflag_valid : out std_logic;
      l_returnflag_ready : in std_logic;
      l_returnflag_data : out std_logic_vector(7 downto 0);
      l_returnflag_last : out std_logic;
      l_returnflag_strb : out std_logic;
      l_suppkey_valid : out std_logic;
      l_suppkey_ready : in std_logic;
      l_suppkey_data : out std_logic_vector(31 downto 0);
      l_suppkey_last : out std_logic;
      l_suppkey_strb : out std_logic;
      l_shipdate_valid : out std_logic;
      l_shipdate_ready : in std_logic;
      l_shipdate_data : out std_logic_vector(25 downto 0);
      l_shipdate_last : out std_logic;
      l_shipdate_strb : out std_logic;
      l_tax_valid : out std_logic;
      l_tax_ready : in std_logic;
      l_tax_data : out std_logic_vector(49 downto 0);
      l_tax_last : out std_logic;
      l_tax_strb : out std_logic;
      l_comment_valid : out std_logic;
      l_comment_ready : in std_logic;
      l_comment_data : out std_logic_vector(7 downto 0);
      l_comment_last : out std_logic_vector(1 downto 0);
      l_comment_strb : out std_logic;
      l_linestatus_valid : out std_logic;
      l_linestatus_ready : in std_logic;
      l_linestatus_data : out std_logic_vector(7 downto 0);
      l_linestatus_last : out std_logic;
      l_linestatus_strb : out std_logic;
      l_quantity_valid : out std_logic;
      l_quantity_ready : in std_logic;
      l_quantity_data : out std_logic_vector(49 downto 0);
      l_quantity_last : out std_logic;
      l_quantity_strb : out std_logic;
      l_extendedprice_valid : out std_logic;
      l_extendedprice_ready : in std_logic;
      l_extendedprice_data : out std_logic_vector(49 downto 0);
      l_extendedprice_last : out std_logic;
      l_extendedprice_strb : out std_logic;
      l_commitdate_valid : out std_logic;
      l_commitdate_ready : in std_logic;
      l_commitdate_data : out std_logic_vector(25 downto 0);
      l_commitdate_last : out std_logic;
      l_commitdate_strb : out std_logic;
      l_receiptdate_valid : out std_logic;
      l_receiptdate_ready : in std_logic;
      l_receiptdate_data : out std_logic_vector(25 downto 0);
      l_receiptdate_last : out std_logic;
      l_receiptdate_strb : out std_logic;
      l_shipinstruct_valid : out std_logic;
      l_shipinstruct_ready : in std_logic;
      l_shipinstruct_data : out std_logic_vector(199 downto 0);
      l_shipinstruct_last : out std_logic;
      l_shipinstruct_strb : out std_logic;
      l_discount_valid : out std_logic;
      l_discount_ready : in std_logic;
      l_discount_data : out std_logic_vector(49 downto 0);
      l_discount_last : out std_logic;
      l_discount_strb : out std_logic;
      l_partkey_valid : out std_logic;
      l_partkey_ready : in std_logic;
      l_partkey_data : out std_logic_vector(31 downto 0);
      l_partkey_last : out std_logic;
      l_partkey_strb : out std_logic;
      l_shipmode_valid : out std_logic;
      l_shipmode_ready : in std_logic;
      l_shipmode_data : out std_logic_vector(79 downto 0);
      l_shipmode_last : out std_logic;
      l_shipmode_strb : out std_logic
    );
  end component;

  component test_project__tpch__lineitem_s_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      l_orderkey_valid : in std_logic;
      l_orderkey_ready : out std_logic;
      l_orderkey_data : in std_logic_vector(31 downto 0);
      l_orderkey_last : in std_logic;
      l_orderkey_strb : in std_logic;
      l_linenumber_valid : in std_logic;
      l_linenumber_ready : out std_logic;
      l_linenumber_data : in std_logic_vector(31 downto 0);
      l_linenumber_last : in std_logic;
      l_linenumber_strb : in std_logic;
      l_returnflag_valid : out std_logic;
      l_returnflag_ready : in std_logic;
      l_returnflag_data : out std_logic_vector(7 downto 0);
      l_returnflag_last : out std_logic;
      l_returnflag_strb : out std_logic;
      l_suppkey_valid : out std_logic;
      l_suppkey_ready : in std_logic;
      l_suppkey_data : out std_logic_vector(31 downto 0);
      l_suppkey_last : out std_logic;
      l_suppkey_strb : out std_logic;
      l_shipdate_valid : out std_logic;
      l_shipdate_ready : in std_logic;
      l_shipdate_data : out std_logic_vector(25 downto 0);
      l_shipdate_last : out std_logic;
      l_shipdate_strb : out std_logic;
      l_tax_valid : out std_logic;
      l_tax_ready : in std_logic;
      l_tax_data : out std_logic_vector(49 downto 0);
      l_tax_last : out std_logic;
      l_tax_strb : out std_logic;
      l_comment_valid : out std_logic;
      l_comment_ready : in std_logic;
      l_comment_data : out std_logic_vector(7 downto 0);
      l_comment_last : out std_logic_vector(1 downto 0);
      l_comment_strb : out std_logic;
      l_linestatus_valid : out std_logic;
      l_linestatus_ready : in std_logic;
      l_linestatus_data : out std_logic_vector(7 downto 0);
      l_linestatus_last : out std_logic;
      l_linestatus_strb : out std_logic;
      l_quantity_valid : out std_logic;
      l_quantity_ready : in std_logic;
      l_quantity_data : out std_logic_vector(49 downto 0);
      l_quantity_last : out std_logic;
      l_quantity_strb : out std_logic;
      l_extendedprice_valid : out std_logic;
      l_extendedprice_ready : in std_logic;
      l_extendedprice_data : out std_logic_vector(49 downto 0);
      l_extendedprice_last : out std_logic;
      l_extendedprice_strb : out std_logic;
      l_commitdate_valid : out std_logic;
      l_commitdate_ready : in std_logic;
      l_commitdate_data : out std_logic_vector(25 downto 0);
      l_commitdate_last : out std_logic;
      l_commitdate_strb : out std_logic;
      l_receiptdate_valid : out std_logic;
      l_receiptdate_ready : in std_logic;
      l_receiptdate_data : out std_logic_vector(25 downto 0);
      l_receiptdate_last : out std_logic;
      l_receiptdate_strb : out std_logic;
      l_shipinstruct_valid : out std_logic;
      l_shipinstruct_ready : in std_logic;
      l_shipinstruct_data : out std_logic_vector(199 downto 0);
      l_shipinstruct_last : out std_logic;
      l_shipinstruct_strb : out std_logic;
      l_discount_valid : out std_logic;
      l_discount_ready : in std_logic;
      l_discount_data : out std_logic_vector(49 downto 0);
      l_discount_last : out std_logic;
      l_discount_strb : out std_logic;
      l_partkey_valid : out std_logic;
      l_partkey_ready : in std_logic;
      l_partkey_data : out std_logic_vector(31 downto 0);
      l_partkey_last : out std_logic;
      l_partkey_strb : out std_logic;
      l_shipmode_valid : out std_logic;
      l_shipmode_ready : in std_logic;
      l_shipmode_data : out std_logic_vector(79 downto 0);
      l_shipmode_last : out std_logic;
      l_shipmode_strb : out std_logic
    );
  end component;

  component test_project__tpch__main_i_com
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
  end component;

  component test_project__tpch__main_s_com
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
  end component;

  component test_project__tpch__multiplier_iATStreamIWSQL_decimal_15_2_streamIM_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      input1_valid : in std_logic;
      input1_ready : out std_logic;
      input1_data : in std_logic_vector(49 downto 0);
      input1_last : in std_logic;
      input1_strb : in std_logic;
      input0_valid : in std_logic;
      input0_ready : out std_logic;
      input0_data : in std_logic_vector(49 downto 0);
      input0_last : in std_logic;
      input0_strb : in std_logic;
      output_valid : out std_logic;
      output_ready : in std_logic;
      output_data : out std_logic_vector(49 downto 0);
      output_last : out std_logic;
      output_strb : out std_logic;
      overflow_valid : out std_logic;
      overflow_ready : in std_logic;
      overflow_data : out std_logic;
      overflow_strb : out std_logic
    );
  end component;

  component test_project__tpch__multiplier_sATStreamIWSQL_decimal_15_2_streamIM_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      input1_valid : in std_logic;
      input1_ready : out std_logic;
      input1_data : in std_logic_vector(49 downto 0);
      input1_last : in std_logic;
      input1_strb : in std_logic;
      input0_valid : in std_logic;
      input0_ready : out std_logic;
      input0_data : in std_logic_vector(49 downto 0);
      input0_last : in std_logic;
      input0_strb : in std_logic;
      output_valid : out std_logic;
      output_ready : in std_logic;
      output_data : out std_logic_vector(49 downto 0);
      output_last : out std_logic;
      output_strb : out std_logic;
      overflow_valid : out std_logic;
      overflow_ready : in std_logic;
      overflow_data : out std_logic;
      overflow_strb : out std_logic
    );
  end component;

  component test_project__tpch__nation_i_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      n_nationkey_valid : in std_logic;
      n_nationkey_ready : out std_logic;
      n_nationkey_data : in std_logic_vector(31 downto 0);
      n_nationkey_last : in std_logic;
      n_nationkey_strb : in std_logic;
      n_name_valid : out std_logic;
      n_name_ready : in std_logic;
      n_name_data : out std_logic_vector(199 downto 0);
      n_name_last : out std_logic;
      n_name_strb : out std_logic;
      n_regionkey_valid : out std_logic;
      n_regionkey_ready : in std_logic;
      n_regionkey_data : out std_logic_vector(31 downto 0);
      n_regionkey_last : out std_logic;
      n_regionkey_strb : out std_logic;
      n_comment_valid : out std_logic;
      n_comment_ready : in std_logic;
      n_comment_data : out std_logic_vector(7 downto 0);
      n_comment_last : out std_logic_vector(1 downto 0);
      n_comment_strb : out std_logic
    );
  end component;

  component test_project__tpch__nation_s_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      n_nationkey_valid : in std_logic;
      n_nationkey_ready : out std_logic;
      n_nationkey_data : in std_logic_vector(31 downto 0);
      n_nationkey_last : in std_logic;
      n_nationkey_strb : in std_logic;
      n_name_valid : out std_logic;
      n_name_ready : in std_logic;
      n_name_data : out std_logic_vector(199 downto 0);
      n_name_last : out std_logic;
      n_name_strb : out std_logic;
      n_regionkey_valid : out std_logic;
      n_regionkey_ready : in std_logic;
      n_regionkey_data : out std_logic_vector(31 downto 0);
      n_regionkey_last : out std_logic;
      n_regionkey_strb : out std_logic;
      n_comment_valid : out std_logic;
      n_comment_ready : in std_logic;
      n_comment_data : out std_logic_vector(7 downto 0);
      n_comment_last : out std_logic_vector(1 downto 0);
      n_comment_strb : out std_logic
    );
  end component;

  component test_project__tpch__orders_i_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      o_orderkey_valid : in std_logic;
      o_orderkey_ready : out std_logic;
      o_orderkey_data : in std_logic_vector(31 downto 0);
      o_orderkey_last : in std_logic;
      o_orderkey_strb : in std_logic;
      o_custkey_valid : out std_logic;
      o_custkey_ready : in std_logic;
      o_custkey_data : out std_logic_vector(31 downto 0);
      o_custkey_last : out std_logic;
      o_custkey_strb : out std_logic;
      o_orderdate_valid : out std_logic;
      o_orderdate_ready : in std_logic;
      o_orderdate_data : out std_logic_vector(25 downto 0);
      o_orderdate_last : out std_logic;
      o_orderdate_strb : out std_logic;
      o_totalprice_valid : out std_logic;
      o_totalprice_ready : in std_logic;
      o_totalprice_data : out std_logic_vector(49 downto 0);
      o_totalprice_last : out std_logic;
      o_totalprice_strb : out std_logic;
      o_shippriority_valid : out std_logic;
      o_shippriority_ready : in std_logic;
      o_shippriority_data : out std_logic_vector(31 downto 0);
      o_shippriority_last : out std_logic;
      o_shippriority_strb : out std_logic;
      o_comment_valid : out std_logic;
      o_comment_ready : in std_logic;
      o_comment_data : out std_logic_vector(7 downto 0);
      o_comment_last : out std_logic_vector(1 downto 0);
      o_comment_strb : out std_logic;
      o_clerk_valid : out std_logic;
      o_clerk_ready : in std_logic;
      o_clerk_data : out std_logic_vector(119 downto 0);
      o_clerk_last : out std_logic;
      o_clerk_strb : out std_logic;
      o_orderstatus_valid : out std_logic;
      o_orderstatus_ready : in std_logic;
      o_orderstatus_data : out std_logic_vector(7 downto 0);
      o_orderstatus_last : out std_logic;
      o_orderstatus_strb : out std_logic;
      o_orderpriority_valid : out std_logic;
      o_orderpriority_ready : in std_logic;
      o_orderpriority_data : out std_logic_vector(119 downto 0);
      o_orderpriority_last : out std_logic;
      o_orderpriority_strb : out std_logic
    );
  end component;

  component test_project__tpch__orders_s_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      o_orderkey_valid : in std_logic;
      o_orderkey_ready : out std_logic;
      o_orderkey_data : in std_logic_vector(31 downto 0);
      o_orderkey_last : in std_logic;
      o_orderkey_strb : in std_logic;
      o_custkey_valid : out std_logic;
      o_custkey_ready : in std_logic;
      o_custkey_data : out std_logic_vector(31 downto 0);
      o_custkey_last : out std_logic;
      o_custkey_strb : out std_logic;
      o_orderdate_valid : out std_logic;
      o_orderdate_ready : in std_logic;
      o_orderdate_data : out std_logic_vector(25 downto 0);
      o_orderdate_last : out std_logic;
      o_orderdate_strb : out std_logic;
      o_totalprice_valid : out std_logic;
      o_totalprice_ready : in std_logic;
      o_totalprice_data : out std_logic_vector(49 downto 0);
      o_totalprice_last : out std_logic;
      o_totalprice_strb : out std_logic;
      o_shippriority_valid : out std_logic;
      o_shippriority_ready : in std_logic;
      o_shippriority_data : out std_logic_vector(31 downto 0);
      o_shippriority_last : out std_logic;
      o_shippriority_strb : out std_logic;
      o_comment_valid : out std_logic;
      o_comment_ready : in std_logic;
      o_comment_data : out std_logic_vector(7 downto 0);
      o_comment_last : out std_logic_vector(1 downto 0);
      o_comment_strb : out std_logic;
      o_clerk_valid : out std_logic;
      o_clerk_ready : in std_logic;
      o_clerk_data : out std_logic_vector(119 downto 0);
      o_clerk_last : out std_logic;
      o_clerk_strb : out std_logic;
      o_orderstatus_valid : out std_logic;
      o_orderstatus_ready : in std_logic;
      o_orderstatus_data : out std_logic_vector(7 downto 0);
      o_orderstatus_last : out std_logic;
      o_orderstatus_strb : out std_logic;
      o_orderpriority_valid : out std_logic;
      o_orderpriority_ready : in std_logic;
      o_orderpriority_data : out std_logic_vector(119 downto 0);
      o_orderpriority_last : out std_logic;
      o_orderpriority_strb : out std_logic
    );
  end component;

  component test_project__tpch__part_i_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      p_partkey_valid : in std_logic;
      p_partkey_ready : out std_logic;
      p_partkey_data : in std_logic_vector(31 downto 0);
      p_partkey_last : in std_logic;
      p_partkey_strb : in std_logic;
      p_name_valid : out std_logic;
      p_name_ready : in std_logic;
      p_name_data : out std_logic_vector(7 downto 0);
      p_name_last : out std_logic_vector(1 downto 0);
      p_name_strb : out std_logic;
      p_comment_valid : out std_logic;
      p_comment_ready : in std_logic;
      p_comment_data : out std_logic_vector(7 downto 0);
      p_comment_last : out std_logic_vector(1 downto 0);
      p_comment_strb : out std_logic;
      p_mfgr_valid : out std_logic;
      p_mfgr_ready : in std_logic;
      p_mfgr_data : out std_logic_vector(199 downto 0);
      p_mfgr_last : out std_logic;
      p_mfgr_strb : out std_logic;
      p_type_valid : out std_logic;
      p_type_ready : in std_logic;
      p_type_data : out std_logic_vector(7 downto 0);
      p_type_last : out std_logic_vector(1 downto 0);
      p_type_strb : out std_logic;
      p_container_valid : out std_logic;
      p_container_ready : in std_logic;
      p_container_data : out std_logic_vector(79 downto 0);
      p_container_last : out std_logic;
      p_container_strb : out std_logic;
      p_brand_valid : out std_logic;
      p_brand_ready : in std_logic;
      p_brand_data : out std_logic_vector(79 downto 0);
      p_brand_last : out std_logic;
      p_brand_strb : out std_logic;
      p_retailprice_valid : out std_logic;
      p_retailprice_ready : in std_logic;
      p_retailprice_data : out std_logic_vector(49 downto 0);
      p_retailprice_last : out std_logic;
      p_retailprice_strb : out std_logic;
      p_size_valid : out std_logic;
      p_size_ready : in std_logic;
      p_size_data : out std_logic_vector(31 downto 0);
      p_size_last : out std_logic;
      p_size_strb : out std_logic
    );
  end component;

  component test_project__tpch__part_s_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      p_partkey_valid : in std_logic;
      p_partkey_ready : out std_logic;
      p_partkey_data : in std_logic_vector(31 downto 0);
      p_partkey_last : in std_logic;
      p_partkey_strb : in std_logic;
      p_name_valid : out std_logic;
      p_name_ready : in std_logic;
      p_name_data : out std_logic_vector(7 downto 0);
      p_name_last : out std_logic_vector(1 downto 0);
      p_name_strb : out std_logic;
      p_comment_valid : out std_logic;
      p_comment_ready : in std_logic;
      p_comment_data : out std_logic_vector(7 downto 0);
      p_comment_last : out std_logic_vector(1 downto 0);
      p_comment_strb : out std_logic;
      p_mfgr_valid : out std_logic;
      p_mfgr_ready : in std_logic;
      p_mfgr_data : out std_logic_vector(199 downto 0);
      p_mfgr_last : out std_logic;
      p_mfgr_strb : out std_logic;
      p_type_valid : out std_logic;
      p_type_ready : in std_logic;
      p_type_data : out std_logic_vector(7 downto 0);
      p_type_last : out std_logic_vector(1 downto 0);
      p_type_strb : out std_logic;
      p_container_valid : out std_logic;
      p_container_ready : in std_logic;
      p_container_data : out std_logic_vector(79 downto 0);
      p_container_last : out std_logic;
      p_container_strb : out std_logic;
      p_brand_valid : out std_logic;
      p_brand_ready : in std_logic;
      p_brand_data : out std_logic_vector(79 downto 0);
      p_brand_last : out std_logic;
      p_brand_strb : out std_logic;
      p_retailprice_valid : out std_logic;
      p_retailprice_ready : in std_logic;
      p_retailprice_data : out std_logic_vector(49 downto 0);
      p_retailprice_last : out std_logic;
      p_retailprice_strb : out std_logic;
      p_size_valid : out std_logic;
      p_size_ready : in std_logic;
      p_size_data : out std_logic_vector(31 downto 0);
      p_size_last : out std_logic;
      p_size_strb : out std_logic
    );
  end component;

  component test_project__tpch__partsupp_i_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      ps_suppkey_valid : in std_logic;
      ps_suppkey_ready : out std_logic;
      ps_suppkey_data : in std_logic_vector(31 downto 0);
      ps_suppkey_last : in std_logic;
      ps_suppkey_strb : in std_logic;
      ps_partkey_valid : in std_logic;
      ps_partkey_ready : out std_logic;
      ps_partkey_data : in std_logic_vector(31 downto 0);
      ps_partkey_last : in std_logic;
      ps_partkey_strb : in std_logic;
      ps_supplycost_valid : out std_logic;
      ps_supplycost_ready : in std_logic;
      ps_supplycost_data : out std_logic_vector(49 downto 0);
      ps_supplycost_last : out std_logic;
      ps_supplycost_strb : out std_logic;
      ps_availqty_valid : out std_logic;
      ps_availqty_ready : in std_logic;
      ps_availqty_data : out std_logic_vector(31 downto 0);
      ps_availqty_last : out std_logic;
      ps_availqty_strb : out std_logic;
      ps_comment_valid : out std_logic;
      ps_comment_ready : in std_logic;
      ps_comment_data : out std_logic_vector(7 downto 0);
      ps_comment_last : out std_logic_vector(1 downto 0);
      ps_comment_strb : out std_logic
    );
  end component;

  component test_project__tpch__partsupp_s_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      ps_suppkey_valid : in std_logic;
      ps_suppkey_ready : out std_logic;
      ps_suppkey_data : in std_logic_vector(31 downto 0);
      ps_suppkey_last : in std_logic;
      ps_suppkey_strb : in std_logic;
      ps_partkey_valid : in std_logic;
      ps_partkey_ready : out std_logic;
      ps_partkey_data : in std_logic_vector(31 downto 0);
      ps_partkey_last : in std_logic;
      ps_partkey_strb : in std_logic;
      ps_supplycost_valid : out std_logic;
      ps_supplycost_ready : in std_logic;
      ps_supplycost_data : out std_logic_vector(49 downto 0);
      ps_supplycost_last : out std_logic;
      ps_supplycost_strb : out std_logic;
      ps_availqty_valid : out std_logic;
      ps_availqty_ready : in std_logic;
      ps_availqty_data : out std_logic_vector(31 downto 0);
      ps_availqty_last : out std_logic;
      ps_availqty_strb : out std_logic;
      ps_comment_valid : out std_logic;
      ps_comment_ready : in std_logic;
      ps_comment_data : out std_logic_vector(7 downto 0);
      ps_comment_last : out std_logic_vector(1 downto 0);
      ps_comment_strb : out std_logic
    );
  end component;

  component test_project__tpch__region_i_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      r_regionkey_valid : in std_logic;
      r_regionkey_ready : out std_logic;
      r_regionkey_data : in std_logic_vector(31 downto 0);
      r_regionkey_last : in std_logic;
      r_regionkey_strb : in std_logic;
      r_comment_valid : out std_logic;
      r_comment_ready : in std_logic;
      r_comment_data : out std_logic_vector(7 downto 0);
      r_comment_last : out std_logic_vector(1 downto 0);
      r_comment_strb : out std_logic;
      r_name_valid : out std_logic;
      r_name_ready : in std_logic;
      r_name_data : out std_logic_vector(199 downto 0);
      r_name_last : out std_logic;
      r_name_strb : out std_logic
    );
  end component;

  component test_project__tpch__region_s_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      r_regionkey_valid : in std_logic;
      r_regionkey_ready : out std_logic;
      r_regionkey_data : in std_logic_vector(31 downto 0);
      r_regionkey_last : in std_logic;
      r_regionkey_strb : in std_logic;
      r_comment_valid : out std_logic;
      r_comment_ready : in std_logic;
      r_comment_data : out std_logic_vector(7 downto 0);
      r_comment_last : out std_logic_vector(1 downto 0);
      r_comment_strb : out std_logic;
      r_name_valid : out std_logic;
      r_name_ready : in std_logic;
      r_name_data : out std_logic_vector(199 downto 0);
      r_name_last : out std_logic;
      r_name_strb : out std_logic
    );
  end component;

  component test_project__tpch__sql_date_constructor_i_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      year_input_valid : in std_logic;
      year_input_ready : out std_logic;
      year_input_data : in std_logic_vector(16 downto 0);
      year_input_strb : in std_logic;
      month_input_valid : in std_logic;
      month_input_ready : out std_logic;
      month_input_data : in std_logic_vector(3 downto 0);
      month_input_strb : in std_logic;
      day_input_valid : in std_logic;
      day_input_ready : out std_logic;
      day_input_data : in std_logic_vector(4 downto 0);
      day_input_strb : in std_logic;
      date_output_valid : out std_logic;
      date_output_ready : in std_logic;
      date_output_data : out std_logic_vector(25 downto 0);
      date_output_last : out std_logic;
      date_output_strb : out std_logic
    );
  end component;

  component test_project__tpch__sql_date_constructor_s_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      year_input_valid : in std_logic;
      year_input_ready : out std_logic;
      year_input_data : in std_logic_vector(16 downto 0);
      year_input_strb : in std_logic;
      month_input_valid : in std_logic;
      month_input_ready : out std_logic;
      month_input_data : in std_logic_vector(3 downto 0);
      month_input_strb : in std_logic;
      day_input_valid : in std_logic;
      day_input_ready : out std_logic;
      day_input_data : in std_logic_vector(4 downto 0);
      day_input_strb : in std_logic;
      date_output_valid : out std_logic;
      date_output_ready : in std_logic;
      date_output_data : out std_logic_vector(25 downto 0);
      date_output_last : out std_logic;
      date_output_strb : out std_logic
    );
  end component;

  component test_project__tpch__stream_filter_1bit_iATStreamIWSQL_decimal_15_2_streamIM_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      input_valid : in std_logic;
      input_ready : out std_logic;
      input_data : in std_logic_vector(49 downto 0);
      input_last : in std_logic;
      input_strb : in std_logic;
      select_valid : in std_logic;
      select_ready : out std_logic;
      select_data : in std_logic;
      select_strb : in std_logic;
      output_valid : out std_logic;
      output_ready : in std_logic;
      output_data : out std_logic_vector(49 downto 0);
      output_last : out std_logic;
      output_strb : out std_logic
    );
  end component;

  component test_project__tpch__stream_filter_1bit_iATStreamIWdate_streamIM_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      select_valid : in std_logic;
      select_ready : out std_logic;
      select_data : in std_logic;
      select_strb : in std_logic;
      input_valid : in std_logic;
      input_ready : out std_logic;
      input_data : in std_logic_vector(25 downto 0);
      input_last : in std_logic;
      input_strb : in std_logic;
      output_valid : out std_logic;
      output_ready : in std_logic;
      output_data : out std_logic_vector(25 downto 0);
      output_last : out std_logic;
      output_strb : out std_logic
    );
  end component;

  component test_project__tpch__stream_filter_1bit_iATStreamIWint_streamIM_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      input_valid : in std_logic;
      input_ready : out std_logic;
      input_data : in std_logic_vector(31 downto 0);
      input_last : in std_logic;
      input_strb : in std_logic;
      select_valid : in std_logic;
      select_ready : out std_logic;
      select_data : in std_logic;
      select_strb : in std_logic;
      output_valid : out std_logic;
      output_ready : in std_logic;
      output_data : out std_logic_vector(31 downto 0);
      output_last : out std_logic;
      output_strb : out std_logic
    );
  end component;

  component test_project__tpch__stream_filter_1bit_sATStreamIWSQL_decimal_15_2_streamIM_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      input_valid : in std_logic;
      input_ready : out std_logic;
      input_data : in std_logic_vector(49 downto 0);
      input_last : in std_logic;
      input_strb : in std_logic;
      select_valid : in std_logic;
      select_ready : out std_logic;
      select_data : in std_logic;
      select_strb : in std_logic;
      output_valid : out std_logic;
      output_ready : in std_logic;
      output_data : out std_logic_vector(49 downto 0);
      output_last : out std_logic;
      output_strb : out std_logic
    );
  end component;

  component test_project__tpch__stream_filter_1bit_sATStreamIWdate_streamIM_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      select_valid : in std_logic;
      select_ready : out std_logic;
      select_data : in std_logic;
      select_strb : in std_logic;
      input_valid : in std_logic;
      input_ready : out std_logic;
      input_data : in std_logic_vector(25 downto 0);
      input_last : in std_logic;
      input_strb : in std_logic;
      output_valid : out std_logic;
      output_ready : in std_logic;
      output_data : out std_logic_vector(25 downto 0);
      output_last : out std_logic;
      output_strb : out std_logic
    );
  end component;

  component test_project__tpch__stream_filter_1bit_sATStreamIWint_streamIM_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      input_valid : in std_logic;
      input_ready : out std_logic;
      input_data : in std_logic_vector(31 downto 0);
      input_last : in std_logic;
      input_strb : in std_logic;
      select_valid : in std_logic;
      select_ready : out std_logic;
      select_data : in std_logic;
      select_strb : in std_logic;
      output_valid : out std_logic;
      output_ready : in std_logic;
      output_data : out std_logic_vector(31 downto 0);
      output_last : out std_logic;
      output_strb : out std_logic
    );
  end component;

  component test_project__tpch__supplier_i_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      s_suppkey_valid : in std_logic;
      s_suppkey_ready : out std_logic;
      s_suppkey_data : in std_logic_vector(31 downto 0);
      s_suppkey_last : in std_logic;
      s_suppkey_strb : in std_logic;
      s_address_valid : out std_logic;
      s_address_ready : in std_logic;
      s_address_data : out std_logic_vector(7 downto 0);
      s_address_last : out std_logic_vector(1 downto 0);
      s_address_strb : out std_logic;
      s_nationkey_valid : out std_logic;
      s_nationkey_ready : in std_logic;
      s_nationkey_data : out std_logic_vector(31 downto 0);
      s_nationkey_last : out std_logic;
      s_nationkey_strb : out std_logic;
      s_acctbal_valid : out std_logic;
      s_acctbal_ready : in std_logic;
      s_acctbal_data : out std_logic_vector(49 downto 0);
      s_acctbal_last : out std_logic;
      s_acctbal_strb : out std_logic;
      s_name_valid : out std_logic;
      s_name_ready : in std_logic;
      s_name_data : out std_logic_vector(199 downto 0);
      s_name_last : out std_logic;
      s_name_strb : out std_logic;
      s_phone_valid : out std_logic;
      s_phone_ready : in std_logic;
      s_phone_data : out std_logic_vector(119 downto 0);
      s_phone_last : out std_logic;
      s_phone_strb : out std_logic;
      s_comment_valid : out std_logic;
      s_comment_ready : in std_logic;
      s_comment_data : out std_logic_vector(7 downto 0);
      s_comment_last : out std_logic_vector(1 downto 0);
      s_comment_strb : out std_logic
    );
  end component;

  component test_project__tpch__supplier_s_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      s_suppkey_valid : in std_logic;
      s_suppkey_ready : out std_logic;
      s_suppkey_data : in std_logic_vector(31 downto 0);
      s_suppkey_last : in std_logic;
      s_suppkey_strb : in std_logic;
      s_address_valid : out std_logic;
      s_address_ready : in std_logic;
      s_address_data : out std_logic_vector(7 downto 0);
      s_address_last : out std_logic_vector(1 downto 0);
      s_address_strb : out std_logic;
      s_nationkey_valid : out std_logic;
      s_nationkey_ready : in std_logic;
      s_nationkey_data : out std_logic_vector(31 downto 0);
      s_nationkey_last : out std_logic;
      s_nationkey_strb : out std_logic;
      s_acctbal_valid : out std_logic;
      s_acctbal_ready : in std_logic;
      s_acctbal_data : out std_logic_vector(49 downto 0);
      s_acctbal_last : out std_logic;
      s_acctbal_strb : out std_logic;
      s_name_valid : out std_logic;
      s_name_ready : in std_logic;
      s_name_data : out std_logic_vector(199 downto 0);
      s_name_last : out std_logic;
      s_name_strb : out std_logic;
      s_phone_valid : out std_logic;
      s_phone_ready : in std_logic;
      s_phone_data : out std_logic_vector(119 downto 0);
      s_phone_last : out std_logic;
      s_phone_strb : out std_logic;
      s_comment_valid : out std_logic;
      s_comment_ready : in std_logic;
      s_comment_data : out std_logic_vector(7 downto 0);
      s_comment_last : out std_logic_vector(1 downto 0);
      s_comment_strb : out std_logic
    );
  end component;

  component test_project__tpch__to_neg_iATStreamIWSQL_decimal_15_2_streamIM_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      input_valid : in std_logic;
      input_ready : out std_logic;
      input_data : in std_logic_vector(49 downto 0);
      input_last : in std_logic;
      input_strb : in std_logic;
      output_valid : out std_logic;
      output_ready : in std_logic;
      output_data : out std_logic_vector(49 downto 0);
      output_last : out std_logic;
      output_strb : out std_logic
    );
  end component;

  component test_project__tpch__to_neg_sATStreamIWSQL_decimal_15_2_streamIM_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      input_valid : in std_logic;
      input_ready : out std_logic;
      input_data : in std_logic_vector(49 downto 0);
      input_last : in std_logic;
      input_strb : in std_logic;
      output_valid : out std_logic;
      output_ready : in std_logic;
      output_data : out std_logic_vector(49 downto 0);
      output_last : out std_logic;
      output_strb : out std_logic
    );
  end component;

  component test_project__tpch__void_iATStreamIWSQL_char10_streamIM_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      input_valid : in std_logic;
      input_ready : out std_logic;
      input_data : in std_logic_vector(79 downto 0);
      input_last : in std_logic;
      input_strb : in std_logic
    );
  end component;

  component test_project__tpch__void_iATStreamIWSQL_char15_streamIM_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      input_valid : in std_logic;
      input_ready : out std_logic;
      input_data : in std_logic_vector(119 downto 0);
      input_last : in std_logic;
      input_strb : in std_logic
    );
  end component;

  component test_project__tpch__void_iATStreamIWSQL_char1_streamIM_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      input_valid : in std_logic;
      input_ready : out std_logic;
      input_data : in std_logic_vector(7 downto 0);
      input_last : in std_logic;
      input_strb : in std_logic
    );
  end component;

  component test_project__tpch__void_iATStreamIWSQL_char25_streamIM_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      input_valid : in std_logic;
      input_ready : out std_logic;
      input_data : in std_logic_vector(199 downto 0);
      input_last : in std_logic;
      input_strb : in std_logic
    );
  end component;

  component test_project__tpch__void_iATStreamIWSQL_decimal_15_2_streamIM_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      input_valid : in std_logic;
      input_ready : out std_logic;
      input_data : in std_logic_vector(49 downto 0);
      input_last : in std_logic;
      input_strb : in std_logic
    );
  end component;

  component test_project__tpch__void_iATStreamIWcount_typeIM_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      input_valid : in std_logic;
      input_ready : out std_logic;
      input_data : in std_logic_vector(31 downto 0);
      input_strb : in std_logic
    );
  end component;

  component test_project__tpch__void_iATStreamIWdate_streamIM_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      input_valid : in std_logic;
      input_ready : out std_logic;
      input_data : in std_logic_vector(25 downto 0);
      input_last : in std_logic;
      input_strb : in std_logic
    );
  end component;

  component test_project__tpch__void_iATStreamIWint_streamIM_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      input_valid : in std_logic;
      input_ready : out std_logic;
      input_data : in std_logic_vector(31 downto 0);
      input_last : in std_logic;
      input_strb : in std_logic
    );
  end component;

  component test_project__tpch__void_iATStreamIWvarchar_streamIM_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      input_valid : in std_logic;
      input_ready : out std_logic;
      input_data : in std_logic_vector(7 downto 0);
      input_last : in std_logic_vector(1 downto 0);
      input_strb : in std_logic
    );
  end component;

  component test_project__tpch__void_sATStreamIWSQL_char10_streamIM_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      input_valid : in std_logic;
      input_ready : out std_logic;
      input_data : in std_logic_vector(79 downto 0);
      input_last : in std_logic;
      input_strb : in std_logic
    );
  end component;

  component test_project__tpch__void_sATStreamIWSQL_char15_streamIM_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      input_valid : in std_logic;
      input_ready : out std_logic;
      input_data : in std_logic_vector(119 downto 0);
      input_last : in std_logic;
      input_strb : in std_logic
    );
  end component;

  component test_project__tpch__void_sATStreamIWSQL_char1_streamIM_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      input_valid : in std_logic;
      input_ready : out std_logic;
      input_data : in std_logic_vector(7 downto 0);
      input_last : in std_logic;
      input_strb : in std_logic
    );
  end component;

  component test_project__tpch__void_sATStreamIWSQL_char25_streamIM_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      input_valid : in std_logic;
      input_ready : out std_logic;
      input_data : in std_logic_vector(199 downto 0);
      input_last : in std_logic;
      input_strb : in std_logic
    );
  end component;

  component test_project__tpch__void_sATStreamIWSQL_decimal_15_2_streamIM_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      input_valid : in std_logic;
      input_ready : out std_logic;
      input_data : in std_logic_vector(49 downto 0);
      input_last : in std_logic;
      input_strb : in std_logic
    );
  end component;

  component test_project__tpch__void_sATStreamIWcount_typeIM_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      input_valid : in std_logic;
      input_ready : out std_logic;
      input_data : in std_logic_vector(31 downto 0);
      input_strb : in std_logic
    );
  end component;

  component test_project__tpch__void_sATStreamIWdate_streamIM_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      input_valid : in std_logic;
      input_ready : out std_logic;
      input_data : in std_logic_vector(25 downto 0);
      input_last : in std_logic;
      input_strb : in std_logic
    );
  end component;

  component test_project__tpch__void_sATStreamIWint_streamIM_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      input_valid : in std_logic;
      input_ready : out std_logic;
      input_data : in std_logic_vector(31 downto 0);
      input_last : in std_logic;
      input_strb : in std_logic
    );
  end component;

  component test_project__tpch__void_sATStreamIWvarchar_streamIM_com
    port (
      clk : in std_logic;
      rst : in std_logic;
      input_valid : in std_logic;
      input_ready : out std_logic;
      input_data : in std_logic_vector(7 downto 0);
      input_last : in std_logic_vector(1 downto 0);
      input_strb : in std_logic
    );
  end component;

  component test_project__tpch__where_claus_i_com
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
  end component;

  component test_project__tpch__where_claus_s_com
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
  end component;

end proj;