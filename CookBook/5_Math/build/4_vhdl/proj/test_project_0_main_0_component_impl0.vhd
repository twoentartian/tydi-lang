library ieee;
use ieee.std_logic_1164.all;

library work;
use work.proj.all;

entity test_project_0_main_0_component_impl0_com is
  port (
    clk : in std_logic;
    rst : in std_logic;
    input_valid : in std_logic;
    input_ready : out std_logic;
    input_data : in std_logic_vector(159 downto 0);
    input_strb : in std_logic_vector(0 downto 0);
    output_valid : out std_logic;
    output_ready : in std_logic;
    output_data : out std_logic_vector(159 downto 0);
    output_strb : out std_logic_vector(0 downto 0)
  );
end test_project_0_main_0_component_impl0_com;

architecture test_project_0_main_0_component_impl0 of test_project_0_main_0_component_impl0_com is
begin
  output_valid <= input_valid;
  input_ready <= output_ready;
  output_data <= input_data;
  output_strb <= input_strb;
end test_project_0_main_0_component_impl0;