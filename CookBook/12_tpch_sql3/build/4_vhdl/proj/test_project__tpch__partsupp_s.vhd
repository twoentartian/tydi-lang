library ieee;
use ieee.std_logic_1164.all;

library work;
use work.proj.all;

entity test_project__tpch__partsupp_s_com is
  port (
    clk : in std_logic;
    rst : in std_logic;
    ps_suppkey_valid : in std_logic;
    ps_suppkey_ready : out std_logic;
    ps_suppkey_data : in std_logic_vector(31 downto 0);
    ps_suppkey_last : in std_logic;
    ps_suppkey_strb : in std_logic;
    ps_partkey_valid : in std_logic;
    ps_partkey_ready : out std_logic;
    ps_partkey_data : in std_logic_vector(31 downto 0);
    ps_partkey_last : in std_logic;
    ps_partkey_strb : in std_logic;
    ps_supplycost_valid : out std_logic;
    ps_supplycost_ready : in std_logic;
    ps_supplycost_data : out std_logic_vector(49 downto 0);
    ps_supplycost_last : out std_logic;
    ps_supplycost_strb : out std_logic;
    ps_availqty_valid : out std_logic;
    ps_availqty_ready : in std_logic;
    ps_availqty_data : out std_logic_vector(31 downto 0);
    ps_availqty_last : out std_logic;
    ps_availqty_strb : out std_logic;
    ps_comment_valid : out std_logic;
    ps_comment_ready : in std_logic;
    ps_comment_data : out std_logic_vector(7 downto 0);
    ps_comment_last : out std_logic_vector(1 downto 0);
    ps_comment_strb : out std_logic
  );
end test_project__tpch__partsupp_s_com;

architecture Behavioral of test_project__tpch__partsupp_s_com is
begin
end Behavioral;