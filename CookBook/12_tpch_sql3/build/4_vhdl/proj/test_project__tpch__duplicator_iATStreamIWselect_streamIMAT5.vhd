library ieee;
use ieee.std_logic_1164.all;

library work;
use work.proj.all;

entity test_project__tpch__duplicator_iATStreamIWselect_streamIMAT5_com is
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
end test_project__tpch__duplicator_iATStreamIWselect_streamIMAT5_com;

architecture Behavioral of test_project__tpch__duplicator_iATStreamIWselect_streamIMAT5_com is
begin
end Behavioral;