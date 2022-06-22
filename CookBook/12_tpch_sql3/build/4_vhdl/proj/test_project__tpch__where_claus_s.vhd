library ieee;
use ieee.std_logic_1164.all;

library work;
use work.proj.all;

entity test_project__tpch__where_claus_s_com is
  port (
    clk : in std_logic;
    rst : in std_logic;
    o_custkey_valid : in std_logic;
    o_custkey_ready : out std_logic;
    o_custkey_data : in std_logic_vector(31 downto 0);
    o_custkey_last : in std_logic;
    o_custkey_strb : in std_logic;
    c_custkey_valid : in std_logic;
    c_custkey_ready : out std_logic;
    c_custkey_data : in std_logic_vector(31 downto 0);
    c_custkey_last : in std_logic;
    c_custkey_strb : in std_logic;
    l_shipdate_valid : in std_logic;
    l_shipdate_ready : out std_logic;
    l_shipdate_data : in std_logic_vector(25 downto 0);
    l_shipdate_last : in std_logic;
    l_shipdate_strb : in std_logic;
    l_orderkey_valid : in std_logic;
    l_orderkey_ready : out std_logic;
    l_orderkey_data : in std_logic_vector(31 downto 0);
    l_orderkey_last : in std_logic;
    l_orderkey_strb : in std_logic;
    o_orderdate_valid : in std_logic;
    o_orderdate_ready : out std_logic;
    o_orderdate_data : in std_logic_vector(25 downto 0);
    o_orderdate_last : in std_logic;
    o_orderdate_strb : in std_logic;
    o_orderkey_valid : in std_logic;
    o_orderkey_ready : out std_logic;
    o_orderkey_data : in std_logic_vector(31 downto 0);
    o_orderkey_last : in std_logic;
    o_orderkey_strb : in std_logic;
    c_mktsegment_valid : in std_logic;
    c_mktsegment_ready : out std_logic;
    c_mktsegment_data : in std_logic_vector(79 downto 0);
    c_mktsegment_last : in std_logic;
    c_mktsegment_strb : in std_logic;
    select_valid : out std_logic;
    select_ready : in std_logic;
    select_data : out std_logic;
    select_strb : out std_logic
  );
end test_project__tpch__where_claus_s_com;

architecture Behavioral of test_project__tpch__where_claus_s_com is
begin
end Behavioral;