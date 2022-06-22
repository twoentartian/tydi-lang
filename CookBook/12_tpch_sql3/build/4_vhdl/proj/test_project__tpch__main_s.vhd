library ieee;
use ieee.std_logic_1164.all;

library work;
use work.proj.all;

entity test_project__tpch__main_s_com is
  port (
    clk : in std_logic;
    rst : in std_logic;
    l_linenumber_valid : in std_logic;
    l_linenumber_ready : out std_logic;
    l_linenumber_data : in std_logic_vector(31 downto 0);
    l_linenumber_last : in std_logic;
    l_linenumber_strb : in std_logic;
    o_orderkey_valid : in std_logic;
    o_orderkey_ready : out std_logic;
    o_orderkey_data : in std_logic_vector(31 downto 0);
    o_orderkey_last : in std_logic;
    o_orderkey_strb : in std_logic;
    l_orderkey_valid : in std_logic;
    l_orderkey_ready : out std_logic;
    l_orderkey_data : in std_logic_vector(31 downto 0);
    l_orderkey_last : in std_logic;
    l_orderkey_strb : in std_logic;
    c_custkey_valid : in std_logic;
    c_custkey_ready : out std_logic;
    c_custkey_data : in std_logic_vector(31 downto 0);
    c_custkey_last : in std_logic;
    c_custkey_strb : in std_logic;
    o_shippriority_valid : out std_logic;
    o_shippriority_ready : in std_logic;
    o_shippriority_data : out std_logic_vector(31 downto 0);
    o_shippriority_last : out std_logic;
    o_shippriority_strb : out std_logic;
    revenue_valid : out std_logic;
    revenue_ready : in std_logic;
    revenue_data : out std_logic_vector(49 downto 0);
    revenue_last : out std_logic;
    revenue_strb : out std_logic;
    err_valid : out std_logic;
    err_ready : in std_logic;
    err_data : out std_logic;
    err_strb : out std_logic;
    o_orderdate_valid : out std_logic;
    o_orderdate_ready : in std_logic;
    o_orderdate_data : out std_logic_vector(25 downto 0);
    o_orderdate_last : out std_logic;
    o_orderdate_strb : out std_logic;
    l_orderkey_out_valid : out std_logic;
    l_orderkey_out_ready : in std_logic;
    l_orderkey_out_data : out std_logic_vector(31 downto 0);
    l_orderkey_out_last : out std_logic;
    l_orderkey_out_strb : out std_logic
  );
end test_project__tpch__main_s_com;

architecture Behavioral of test_project__tpch__main_s_com is
begin
end Behavioral;