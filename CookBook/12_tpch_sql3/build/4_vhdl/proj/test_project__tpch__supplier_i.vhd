library ieee;
use ieee.std_logic_1164.all;

library work;
use work.proj.all;

entity test_project__tpch__supplier_i_com is
  port (
    clk : in std_logic;
    rst : in std_logic;
    s_suppkey_valid : in std_logic;
    s_suppkey_ready : out std_logic;
    s_suppkey_data : in std_logic_vector(31 downto 0);
    s_suppkey_last : in std_logic;
    s_suppkey_strb : in std_logic;
    s_address_valid : out std_logic;
    s_address_ready : in std_logic;
    s_address_data : out std_logic_vector(7 downto 0);
    s_address_last : out std_logic_vector(1 downto 0);
    s_address_strb : out std_logic;
    s_nationkey_valid : out std_logic;
    s_nationkey_ready : in std_logic;
    s_nationkey_data : out std_logic_vector(31 downto 0);
    s_nationkey_last : out std_logic;
    s_nationkey_strb : out std_logic;
    s_acctbal_valid : out std_logic;
    s_acctbal_ready : in std_logic;
    s_acctbal_data : out std_logic_vector(49 downto 0);
    s_acctbal_last : out std_logic;
    s_acctbal_strb : out std_logic;
    s_name_valid : out std_logic;
    s_name_ready : in std_logic;
    s_name_data : out std_logic_vector(199 downto 0);
    s_name_last : out std_logic;
    s_name_strb : out std_logic;
    s_phone_valid : out std_logic;
    s_phone_ready : in std_logic;
    s_phone_data : out std_logic_vector(119 downto 0);
    s_phone_last : out std_logic;
    s_phone_strb : out std_logic;
    s_comment_valid : out std_logic;
    s_comment_ready : in std_logic;
    s_comment_data : out std_logic_vector(7 downto 0);
    s_comment_last : out std_logic_vector(1 downto 0);
    s_comment_strb : out std_logic
  );
end test_project__tpch__supplier_i_com;

architecture Behavioral of test_project__tpch__supplier_i_com is
begin
end Behavioral;