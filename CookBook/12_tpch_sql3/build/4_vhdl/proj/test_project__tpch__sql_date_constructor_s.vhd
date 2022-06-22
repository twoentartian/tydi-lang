library ieee;
use ieee.std_logic_1164.all;

library work;
use work.proj.all;

entity test_project__tpch__sql_date_constructor_s_com is
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
end test_project__tpch__sql_date_constructor_s_com;

architecture Behavioral of test_project__tpch__sql_date_constructor_s_com is
begin
end Behavioral;