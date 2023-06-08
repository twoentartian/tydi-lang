library ieee;
use ieee.std_logic_1164.all;

library work;
use work.proj.all;

-- streamlet documentation
entity test_project_0_main_0_impl_rgb_bypass_com is
  port (
    clk : in std_logic;
    rst : in std_logic;
    input_valid : in std_logic;
    input_ready : out std_logic;
    input_data : in std_logic_vector(23 downto 0);
    input_user : in std_logic_vector(23 downto 0);
    output_valid : out std_logic;
    output_ready : in std_logic;
    output_data : out std_logic_vector(23 downto 0);
    output_user : out std_logic_vector(23 downto 0);
    output2_valid : out std_logic;
    output2_ready : in std_logic;
    output2_data : out std_logic_vector(23 downto 0);
    output2_strb : out std_logic_vector(0 downto 0);
    input2_valid : in std_logic;
    input2_ready : out std_logic;
    input2_data : in std_logic_vector(23 downto 0);
    input2_strb : in std_logic_vector(0 downto 0)
  );
end test_project_0_main_0_impl_rgb_bypass_com;

-- implement documentation
architecture test_project_0_main_0_impl_rgb_bypass of test_project_0_main_0_impl_rgb_bypass_com is
begin
  output2_valid <= input2_valid;
  input2_ready <= output2_ready;
  output2_data <= input2_data;
  output2_strb <= input2_strb;
  output_valid <= input_valid;
  input_ready <= output_ready;
  output_data <= input_data;
  output_user <= input_user;
end test_project_0_main_0_impl_rgb_bypass;