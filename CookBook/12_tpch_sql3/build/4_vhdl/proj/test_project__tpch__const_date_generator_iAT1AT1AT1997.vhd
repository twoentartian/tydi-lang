library ieee;
use ieee.std_logic_1164.all;

library work;
use work.proj.all;

entity test_project__tpch__const_date_generator_iAT1AT1AT1997_com is
  port (
    clk : in std_logic;
    rst : in std_logic;
    date_output_valid : out std_logic;
    date_output_ready : in std_logic;
    date_output_data : out std_logic_vector(25 downto 0);
    date_output_last : out std_logic;
    date_output_strb : out std_logic
  );
end test_project__tpch__const_date_generator_iAT1AT1AT1997_com;

architecture test_project__tpch__const_date_generator_iAT1AT1AT1997 of test_project__tpch__const_date_generator_iAT1AT1AT1997_com is
  signal compositor__date_output_valid : std_logic;
  signal compositor__date_output_ready : std_logic;
  signal compositor__date_output_data : std_logic_vector(25 downto 0);
  signal compositor__date_output_last : std_logic;
  signal compositor__date_output_strb : std_logic;
  signal compositor__day_input_valid : std_logic;
  signal compositor__day_input_ready : std_logic;
  signal compositor__day_input_data : std_logic_vector(4 downto 0);
  signal compositor__day_input_strb : std_logic;
  signal compositor__month_input_valid : std_logic;
  signal compositor__month_input_ready : std_logic;
  signal compositor__month_input_data : std_logic_vector(3 downto 0);
  signal compositor__month_input_strb : std_logic;
  signal compositor__year_input_valid : std_logic;
  signal compositor__year_input_ready : std_logic;
  signal compositor__year_input_data : std_logic_vector(16 downto 0);
  signal compositor__year_input_strb : std_logic;
  signal day_gen__output_valid : std_logic;
  signal day_gen__output_ready : std_logic;
  signal day_gen__output_data : std_logic_vector(4 downto 0);
  signal day_gen__output_strb : std_logic;
  signal month_gen__output_valid : std_logic;
  signal month_gen__output_ready : std_logic;
  signal month_gen__output_data : std_logic_vector(3 downto 0);
  signal month_gen__output_strb : std_logic;
  signal year_gen__output_valid : std_logic;
  signal year_gen__output_ready : std_logic;
  signal year_gen__output_data : std_logic_vector(16 downto 0);
  signal year_gen__output_strb : std_logic;
begin
  compositor: test_project__tpch__sql_date_constructor_i_com port map(
    clk => clk,
    rst => rst,
    year_input_valid => compositor__year_input_valid,
    year_input_ready => compositor__year_input_ready,
    year_input_data => compositor__year_input_data,
    year_input_strb => compositor__year_input_strb,
    month_input_valid => compositor__month_input_valid,
    month_input_ready => compositor__month_input_ready,
    month_input_data => compositor__month_input_data,
    month_input_strb => compositor__month_input_strb,
    day_input_valid => compositor__day_input_valid,
    day_input_ready => compositor__day_input_ready,
    day_input_data => compositor__day_input_data,
    day_input_strb => compositor__day_input_strb,
    date_output_valid => compositor__date_output_valid,
    date_output_ready => compositor__date_output_ready,
    date_output_data => compositor__date_output_data,
    date_output_last => compositor__date_output_last,
    date_output_strb => compositor__date_output_strb
  );
  day_gen: test_project__tpch__const_value_generator_iATStreamIWday_streamIMAT1_com port map(
    clk => clk,
    rst => rst,
    output_valid => day_gen__output_valid,
    output_ready => day_gen__output_ready,
    output_data => day_gen__output_data,
    output_strb => day_gen__output_strb
  );
  month_gen: test_project__tpch__const_value_generator_iATStreamIWmonth_streamIMAT1_com port map(
    clk => clk,
    rst => rst,
    output_valid => month_gen__output_valid,
    output_ready => month_gen__output_ready,
    output_data => month_gen__output_data,
    output_strb => month_gen__output_strb
  );
  year_gen: test_project__tpch__const_value_generator_iATStreamIWyear_streamIMAT1997_com port map(
    clk => clk,
    rst => rst,
    output_valid => year_gen__output_valid,
    output_ready => year_gen__output_ready,
    output_data => year_gen__output_data,
    output_strb => year_gen__output_strb
  );
  compositor__month_input_valid <= month_gen__output_valid;
  month_gen__output_ready <= compositor__month_input_ready;
  compositor__month_input_data <= month_gen__output_data;
  compositor__month_input_strb <= month_gen__output_strb;
  compositor__day_input_valid <= day_gen__output_valid;
  day_gen__output_ready <= compositor__day_input_ready;
  compositor__day_input_data <= day_gen__output_data;
  compositor__day_input_strb <= day_gen__output_strb;
  date_output_valid <= compositor__date_output_valid;
  compositor__date_output_ready <= date_output_ready;
  date_output_data <= compositor__date_output_data;
  date_output_last <= compositor__date_output_last;
  date_output_strb <= compositor__date_output_strb;
  compositor__year_input_valid <= year_gen__output_valid;
  year_gen__output_ready <= compositor__year_input_ready;
  compositor__year_input_data <= year_gen__output_data;
  compositor__year_input_strb <= year_gen__output_strb;
end test_project__tpch__const_date_generator_iAT1AT1AT1997;