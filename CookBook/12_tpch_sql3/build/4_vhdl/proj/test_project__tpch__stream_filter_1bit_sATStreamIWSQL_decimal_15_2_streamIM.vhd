library ieee;
use ieee.std_logic_1164.all;

library work;
use work.proj.all;

entity test_project__tpch__stream_filter_1bit_sATStreamIWSQL_decimal_15_2_streamIM_com is
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
end test_project__tpch__stream_filter_1bit_sATStreamIWSQL_decimal_15_2_streamIM_com;

architecture Behavioral of test_project__tpch__stream_filter_1bit_sATStreamIWSQL_decimal_15_2_streamIM_com is
begin
end Behavioral;