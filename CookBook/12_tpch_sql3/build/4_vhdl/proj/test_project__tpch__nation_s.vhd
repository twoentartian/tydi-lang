library ieee;
use ieee.std_logic_1164.all;

library work;
use work.proj.all;

entity test_project__tpch__nation_s_com is
  port (
    clk : in std_logic;
    rst : in std_logic;
    n_nationkey_valid : in std_logic;
    n_nationkey_ready : out std_logic;
    n_nationkey_data : in std_logic_vector(31 downto 0);
    n_nationkey_last : in std_logic;
    n_nationkey_strb : in std_logic;
    n_name_valid : out std_logic;
    n_name_ready : in std_logic;
    n_name_data : out std_logic_vector(199 downto 0);
    n_name_last : out std_logic;
    n_name_strb : out std_logic;
    n_regionkey_valid : out std_logic;
    n_regionkey_ready : in std_logic;
    n_regionkey_data : out std_logic_vector(31 downto 0);
    n_regionkey_last : out std_logic;
    n_regionkey_strb : out std_logic;
    n_comment_valid : out std_logic;
    n_comment_ready : in std_logic;
    n_comment_data : out std_logic_vector(7 downto 0);
    n_comment_last : out std_logic_vector(1 downto 0);
    n_comment_strb : out std_logic
  );
end test_project__tpch__nation_s_com;

architecture Behavioral of test_project__tpch__nation_s_com is
begin
end Behavioral;