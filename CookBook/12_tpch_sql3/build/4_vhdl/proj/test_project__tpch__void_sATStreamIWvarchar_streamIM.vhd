library ieee;
use ieee.std_logic_1164.all;

library work;
use work.proj.all;

entity test_project__tpch__void_sATStreamIWvarchar_streamIM_com is
  port (
    clk : in std_logic;
    rst : in std_logic;
    input_valid : in std_logic;
    input_ready : out std_logic;
    input_data : in std_logic_vector(7 downto 0);
    input_last : in std_logic_vector(1 downto 0);
    input_strb : in std_logic
  );
end test_project__tpch__void_sATStreamIWvarchar_streamIM_com;

architecture Behavioral of test_project__tpch__void_sATStreamIWvarchar_streamIM_com is
begin
end Behavioral;