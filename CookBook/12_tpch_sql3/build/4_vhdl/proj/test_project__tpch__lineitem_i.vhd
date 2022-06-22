library ieee;
use ieee.std_logic_1164.all;

library work;
use work.proj.all;

entity test_project__tpch__lineitem_i_com is
  port (
    clk : in std_logic;
    rst : in std_logic;
    l_orderkey_valid : in std_logic;
    l_orderkey_ready : out std_logic;
    l_orderkey_data : in std_logic_vector(31 downto 0);
    l_orderkey_last : in std_logic;
    l_orderkey_strb : in std_logic;
    l_linenumber_valid : in std_logic;
    l_linenumber_ready : out std_logic;
    l_linenumber_data : in std_logic_vector(31 downto 0);
    l_linenumber_last : in std_logic;
    l_linenumber_strb : in std_logic;
    l_returnflag_valid : out std_logic;
    l_returnflag_ready : in std_logic;
    l_returnflag_data : out std_logic_vector(7 downto 0);
    l_returnflag_last : out std_logic;
    l_returnflag_strb : out std_logic;
    l_suppkey_valid : out std_logic;
    l_suppkey_ready : in std_logic;
    l_suppkey_data : out std_logic_vector(31 downto 0);
    l_suppkey_last : out std_logic;
    l_suppkey_strb : out std_logic;
    l_shipdate_valid : out std_logic;
    l_shipdate_ready : in std_logic;
    l_shipdate_data : out std_logic_vector(25 downto 0);
    l_shipdate_last : out std_logic;
    l_shipdate_strb : out std_logic;
    l_tax_valid : out std_logic;
    l_tax_ready : in std_logic;
    l_tax_data : out std_logic_vector(49 downto 0);
    l_tax_last : out std_logic;
    l_tax_strb : out std_logic;
    l_comment_valid : out std_logic;
    l_comment_ready : in std_logic;
    l_comment_data : out std_logic_vector(7 downto 0);
    l_comment_last : out std_logic_vector(1 downto 0);
    l_comment_strb : out std_logic;
    l_linestatus_valid : out std_logic;
    l_linestatus_ready : in std_logic;
    l_linestatus_data : out std_logic_vector(7 downto 0);
    l_linestatus_last : out std_logic;
    l_linestatus_strb : out std_logic;
    l_quantity_valid : out std_logic;
    l_quantity_ready : in std_logic;
    l_quantity_data : out std_logic_vector(49 downto 0);
    l_quantity_last : out std_logic;
    l_quantity_strb : out std_logic;
    l_extendedprice_valid : out std_logic;
    l_extendedprice_ready : in std_logic;
    l_extendedprice_data : out std_logic_vector(49 downto 0);
    l_extendedprice_last : out std_logic;
    l_extendedprice_strb : out std_logic;
    l_commitdate_valid : out std_logic;
    l_commitdate_ready : in std_logic;
    l_commitdate_data : out std_logic_vector(25 downto 0);
    l_commitdate_last : out std_logic;
    l_commitdate_strb : out std_logic;
    l_receiptdate_valid : out std_logic;
    l_receiptdate_ready : in std_logic;
    l_receiptdate_data : out std_logic_vector(25 downto 0);
    l_receiptdate_last : out std_logic;
    l_receiptdate_strb : out std_logic;
    l_shipinstruct_valid : out std_logic;
    l_shipinstruct_ready : in std_logic;
    l_shipinstruct_data : out std_logic_vector(199 downto 0);
    l_shipinstruct_last : out std_logic;
    l_shipinstruct_strb : out std_logic;
    l_discount_valid : out std_logic;
    l_discount_ready : in std_logic;
    l_discount_data : out std_logic_vector(49 downto 0);
    l_discount_last : out std_logic;
    l_discount_strb : out std_logic;
    l_partkey_valid : out std_logic;
    l_partkey_ready : in std_logic;
    l_partkey_data : out std_logic_vector(31 downto 0);
    l_partkey_last : out std_logic;
    l_partkey_strb : out std_logic;
    l_shipmode_valid : out std_logic;
    l_shipmode_ready : in std_logic;
    l_shipmode_data : out std_logic_vector(79 downto 0);
    l_shipmode_last : out std_logic;
    l_shipmode_strb : out std_logic
  );
end test_project__tpch__lineitem_i_com;

architecture Behavioral of test_project__tpch__lineitem_i_com is
begin
end Behavioral;