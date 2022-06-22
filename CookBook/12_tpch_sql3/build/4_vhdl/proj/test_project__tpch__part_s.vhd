library ieee;
use ieee.std_logic_1164.all;

library work;
use work.proj.all;

entity test_project__tpch__part_s_com is
  port (
    clk : in std_logic;
    rst : in std_logic;
    p_partkey_valid : in std_logic;
    p_partkey_ready : out std_logic;
    p_partkey_data : in std_logic_vector(31 downto 0);
    p_partkey_last : in std_logic;
    p_partkey_strb : in std_logic;
    p_name_valid : out std_logic;
    p_name_ready : in std_logic;
    p_name_data : out std_logic_vector(7 downto 0);
    p_name_last : out std_logic_vector(1 downto 0);
    p_name_strb : out std_logic;
    p_comment_valid : out std_logic;
    p_comment_ready : in std_logic;
    p_comment_data : out std_logic_vector(7 downto 0);
    p_comment_last : out std_logic_vector(1 downto 0);
    p_comment_strb : out std_logic;
    p_mfgr_valid : out std_logic;
    p_mfgr_ready : in std_logic;
    p_mfgr_data : out std_logic_vector(199 downto 0);
    p_mfgr_last : out std_logic;
    p_mfgr_strb : out std_logic;
    p_type_valid : out std_logic;
    p_type_ready : in std_logic;
    p_type_data : out std_logic_vector(7 downto 0);
    p_type_last : out std_logic_vector(1 downto 0);
    p_type_strb : out std_logic;
    p_container_valid : out std_logic;
    p_container_ready : in std_logic;
    p_container_data : out std_logic_vector(79 downto 0);
    p_container_last : out std_logic;
    p_container_strb : out std_logic;
    p_brand_valid : out std_logic;
    p_brand_ready : in std_logic;
    p_brand_data : out std_logic_vector(79 downto 0);
    p_brand_last : out std_logic;
    p_brand_strb : out std_logic;
    p_retailprice_valid : out std_logic;
    p_retailprice_ready : in std_logic;
    p_retailprice_data : out std_logic_vector(49 downto 0);
    p_retailprice_last : out std_logic;
    p_retailprice_strb : out std_logic;
    p_size_valid : out std_logic;
    p_size_ready : in std_logic;
    p_size_data : out std_logic_vector(31 downto 0);
    p_size_last : out std_logic;
    p_size_strb : out std_logic
  );
end test_project__tpch__part_s_com;

architecture Behavioral of test_project__tpch__part_s_com is
begin
end Behavioral;