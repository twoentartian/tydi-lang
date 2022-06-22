library ieee;
use ieee.std_logic_1164.all;

library work;
use work.proj.all;

entity test_project__tpch__and_iATStreamIWselect_streamIMAT5_com is
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
end test_project__tpch__and_iATStreamIWselect_streamIMAT5_com;

architecture Behavioral of test_project__tpch__and_iATStreamIWselect_streamIMAT5_com is
begin
end Behavioral;