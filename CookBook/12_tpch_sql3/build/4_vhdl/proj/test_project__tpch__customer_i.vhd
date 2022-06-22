library ieee;
use ieee.std_logic_1164.all;

library work;
use work.proj.all;

entity test_project__tpch__customer_i_com is
  port (
    clk : in std_logic;
    rst : in std_logic;
    c_custkey_valid : in std_logic;
    c_custkey_ready : out std_logic;
    c_custkey_data : in std_logic_vector(31 downto 0);
    c_custkey_last : in std_logic;
    c_custkey_strb : in std_logic;
    c_nationkey_valid : out std_logic;
    c_nationkey_ready : in std_logic;
    c_nationkey_data : out std_logic_vector(31 downto 0);
    c_nationkey_last : out std_logic;
    c_nationkey_strb : out std_logic;
    c_comment_valid : out std_logic;
    c_comment_ready : in std_logic;
    c_comment_data : out std_logic_vector(7 downto 0);
    c_comment_last : out std_logic_vector(1 downto 0);
    c_comment_strb : out std_logic;
    c_mktsegment_valid : out std_logic;
    c_mktsegment_ready : in std_logic;
    c_mktsegment_data : out std_logic_vector(79 downto 0);
    c_mktsegment_last : out std_logic;
    c_mktsegment_strb : out std_logic;
    c_address_valid : out std_logic;
    c_address_ready : in std_logic;
    c_address_data : out std_logic_vector(7 downto 0);
    c_address_last : out std_logic_vector(1 downto 0);
    c_address_strb : out std_logic;
    c_phone_valid : out std_logic;
    c_phone_ready : in std_logic;
    c_phone_data : out std_logic_vector(119 downto 0);
    c_phone_last : out std_logic;
    c_phone_strb : out std_logic;
    c_name_valid : out std_logic;
    c_name_ready : in std_logic;
    c_name_data : out std_logic_vector(7 downto 0);
    c_name_last : out std_logic_vector(1 downto 0);
    c_name_strb : out std_logic;
    c_acctbal_valid : out std_logic;
    c_acctbal_ready : in std_logic;
    c_acctbal_data : out std_logic_vector(49 downto 0);
    c_acctbal_last : out std_logic;
    c_acctbal_strb : out std_logic
  );
end test_project__tpch__customer_i_com;

architecture Behavioral of test_project__tpch__customer_i_com is
begin
end Behavioral;