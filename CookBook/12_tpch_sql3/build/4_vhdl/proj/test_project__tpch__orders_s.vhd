library ieee;
use ieee.std_logic_1164.all;

library work;
use work.proj.all;

entity test_project__tpch__orders_s_com is
  port (
    clk : in std_logic;
    rst : in std_logic;
    o_orderkey_valid : in std_logic;
    o_orderkey_ready : out std_logic;
    o_orderkey_data : in std_logic_vector(31 downto 0);
    o_orderkey_last : in std_logic;
    o_orderkey_strb : in std_logic;
    o_custkey_valid : out std_logic;
    o_custkey_ready : in std_logic;
    o_custkey_data : out std_logic_vector(31 downto 0);
    o_custkey_last : out std_logic;
    o_custkey_strb : out std_logic;
    o_orderdate_valid : out std_logic;
    o_orderdate_ready : in std_logic;
    o_orderdate_data : out std_logic_vector(25 downto 0);
    o_orderdate_last : out std_logic;
    o_orderdate_strb : out std_logic;
    o_totalprice_valid : out std_logic;
    o_totalprice_ready : in std_logic;
    o_totalprice_data : out std_logic_vector(49 downto 0);
    o_totalprice_last : out std_logic;
    o_totalprice_strb : out std_logic;
    o_shippriority_valid : out std_logic;
    o_shippriority_ready : in std_logic;
    o_shippriority_data : out std_logic_vector(31 downto 0);
    o_shippriority_last : out std_logic;
    o_shippriority_strb : out std_logic;
    o_comment_valid : out std_logic;
    o_comment_ready : in std_logic;
    o_comment_data : out std_logic_vector(7 downto 0);
    o_comment_last : out std_logic_vector(1 downto 0);
    o_comment_strb : out std_logic;
    o_clerk_valid : out std_logic;
    o_clerk_ready : in std_logic;
    o_clerk_data : out std_logic_vector(119 downto 0);
    o_clerk_last : out std_logic;
    o_clerk_strb : out std_logic;
    o_orderstatus_valid : out std_logic;
    o_orderstatus_ready : in std_logic;
    o_orderstatus_data : out std_logic_vector(7 downto 0);
    o_orderstatus_last : out std_logic;
    o_orderstatus_strb : out std_logic;
    o_orderpriority_valid : out std_logic;
    o_orderpriority_ready : in std_logic;
    o_orderpriority_data : out std_logic_vector(119 downto 0);
    o_orderpriority_last : out std_logic;
    o_orderpriority_strb : out std_logic
  );
end test_project__tpch__orders_s_com;

architecture Behavioral of test_project__tpch__orders_s_com is
begin
end Behavioral;