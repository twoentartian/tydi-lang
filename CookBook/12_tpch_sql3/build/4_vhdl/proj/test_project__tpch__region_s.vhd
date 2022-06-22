library ieee;
use ieee.std_logic_1164.all;

library work;
use work.proj.all;

entity test_project__tpch__region_s_com is
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
end test_project__tpch__region_s_com;

architecture Behavioral of test_project__tpch__region_s_com is
begin
end Behavioral;