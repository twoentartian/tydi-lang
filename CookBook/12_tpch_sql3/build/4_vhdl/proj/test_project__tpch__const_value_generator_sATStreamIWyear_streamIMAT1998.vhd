library ieee;
use ieee.std_logic_1164.all;

library work;
use work.proj.all;

entity test_project__tpch__const_value_generator_sATStreamIWyear_streamIMAT1998_com is
  port (
    clk : in std_logic;
    rst : in std_logic;
    output_valid : out std_logic;
    output_ready : in std_logic;
    output_data : out std_logic_vector(16 downto 0);
    output_strb : out std_logic
  );
end test_project__tpch__const_value_generator_sATStreamIWyear_streamIMAT1998_com;

architecture Behavioral of test_project__tpch__const_value_generator_sATStreamIWyear_streamIMAT1998_com is
begin
end Behavioral;