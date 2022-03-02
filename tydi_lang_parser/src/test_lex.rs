#[allow(unused_imports)]
use pest::Parser;

#[derive(Parser)]
#[grammar = "tydi_lang_syntax.pest"]
struct TydiParser;

#[cfg(test)]
mod lex_tests {
    use super::*;

    #[test]
    fn parse_simple_id0() {
        let mut parse_result = TydiParser::parse(Rule::ID, "abcdefg").expect("unsuccessful parse");
        println!("{}",parse_result);
        let parse_result = parse_result.next().unwrap();
        let mut pass : bool = false;

        match parse_result.as_rule() {
            Rule::ID => {
                let value: &str = parse_result.as_str();
                if value == "abcdefg" {
                    pass = true;
                }
            }
            _ => unreachable!(),
        }

        assert!(pass);
    }
    #[test]
    fn parse_simple_int_normal() {
        let mut parse_result = TydiParser::parse(Rule::INT, "123_456_789").expect("unsuccessful parse");
        println!("{}",parse_result);
        let parse_result = parse_result.next().unwrap();
        let mut pass : bool = false;

        match parse_result.as_rule() {
            Rule::INT_RAW_NORAML => {
                let value: &str = parse_result.as_str();
                if value == "123_456_789" {
                    pass = true;
                }
            }
            _ => unreachable!(),
        }

        assert!(pass);
    }
    #[test]
    fn parse_simple_int_binary() {
        let mut parse_result = TydiParser::parse(Rule::INT, "0b010101").expect("unsuccessful parse");
        println!("{}",parse_result);
        let parse_result = parse_result.next().unwrap();
        let mut pass : bool = false;

        match parse_result.as_rule() {
            Rule::INT_RAW_BIN => {
                let value: &str = parse_result.as_str();
                if value == "0b010101" {
                    pass = true;
                }
            }
            _ => unreachable!(),
        }

        assert!(pass);
    }
    #[test]
    fn parse_simple_int_oct() {
        let mut parse_result = TydiParser::parse(Rule::INT, "0o01234567")
            .expect("unsuccessful parse");
        println!("{}",parse_result);
        let parse_result = parse_result.next().unwrap();
        let mut pass : bool = false;

        match parse_result.as_rule() {
            Rule::INT_RAW_OCT => {
                let value: &str = parse_result.as_str();
                if value == "0o01234567" {
                    pass = true;
                }
            }
            _ => unreachable!(),
        }

        assert!(pass);
    }
    #[test]
    fn parse_simple_int_hex() {
        let mut parse_result = TydiParser::parse(Rule::INT, "0x0123456789abcdef")
            .expect("unsuccessful parse");
        println!("{}",parse_result);
        let parse_result = parse_result.next().unwrap();
        let mut pass : bool = false;

        match parse_result.as_rule() {
            Rule::INT_RAW_HEX => {
                let value: &str = parse_result.as_str();
                if value == "0x0123456789abcdef" {
                    pass = true;
                }
            }
            _ => unreachable!(),
        }

        assert!(pass);
    }
    #[test]
    fn parse_simple_bool() {
        {
            let mut parse_result = TydiParser::parse(Rule::BOOL, "true")
                .expect("unsuccessful parse");
            println!("{}",parse_result);
            let parse_result = parse_result.next().unwrap();
            let mut pass : bool = false;
            match parse_result.as_rule() {
                Rule::BOOL => {
                    let value: &str = parse_result.as_str();
                    if value == "true" {
                        pass = true;
                    }
                }
                _ => unreachable!(),
            }
            assert!(pass);
        }
        {
            let mut parse_result = TydiParser::parse(Rule::BOOL, "false")
                .expect("unsuccessful parse");
            println!("{}",parse_result);
            let parse_result = parse_result.next().unwrap();
            let mut pass : bool = false;
            match parse_result.as_rule() {
                Rule::BOOL => {
                    let value: &str = parse_result.as_str();
                    if value == "false" {
                        pass = true;
                    }
                }
                _ => unreachable!(),
            }
            assert!(pass);
        }

    }
    #[test]
    fn parse_simple_float() {
        {
            let mut parse_result = TydiParser::parse(Rule::FLOAT, "12.34")
                .expect("unsuccessful parse");
            println!("{}",parse_result);
            let parse_result = parse_result.next().unwrap();
            let mut pass : bool = false;

            match parse_result.as_rule() {
                Rule::FLOAT => {
                    let value: &str = parse_result.as_str();
                    if value == "12.34" {
                        pass = true;
                    }
                }
                _ => unreachable!(),
            }
            assert!(pass);
        }
        {
            let mut parse_result = TydiParser::parse(Rule::FLOAT, "0.5")
                .expect("unsuccessful parse");
            println!("{}",parse_result);
            let parse_result = parse_result.next().unwrap();
            let mut pass : bool = false;

            match parse_result.as_rule() {
                Rule::FLOAT => {
                    let value: &str = parse_result.as_str();
                    if value == "0.5" {
                        pass = true;
                    }
                }
                _ => unreachable!(),
            }
            assert!(pass);
        }
    }
    #[test]
    fn parse_simple_string() {
        {
            let mut parse_result = TydiParser::parse(Rule::STR, "\" //1234213fwewq \"")
                .expect("unsuccessful parse");
            println!("{}",parse_result);
            let parse_result = parse_result.next().unwrap();
            let mut pass : bool = false;

            match parse_result.as_rule() {
                Rule::STR => {
                    let value: &str = parse_result.as_str();
                    if value == "\" //1234213fwewq \"" {
                        pass = true;
                    }
                }
                _ => unreachable!(),
            }
            assert!(pass);
        }
        {
            let mut parse_result = TydiParser::parse(Rule::STR, "\" 123hfuedfa3432dcvs3<L:KOP:K{Kiyr3289r \"")
                .expect("unsuccessful parse");
            println!("{}",parse_result);
            let parse_result = parse_result.next().unwrap();
            let mut pass : bool = false;

            match parse_result.as_rule() {
                Rule::STR => {
                    let value: &str = parse_result.as_str();
                    if value == "\" 123hfuedfa3432dcvs3<L:KOP:K{Kiyr3289r \"" {
                        pass = true;
                    }
                }
                _ => unreachable!(),
            }
            assert!(pass);
        }
    }
    #[test]
    fn parse_simple_array() {
        {
            let mut parse_result = TydiParser::parse(Rule::ArrayExp, "{1,2,3,4,5,6}")
                .expect("unsuccessful parse");
            println!("{}",parse_result);
            let parse_result = parse_result.next().unwrap();
            let mut pass : bool = false;

            match parse_result.as_rule() {
                Rule::ArrayExp => {
                    let value: &str = parse_result.as_str();
                    if value == "{1,2,3,4,5,6}" {
                        pass = true;
                    }
                }
                _ => unreachable!(),
            }
            assert!(pass);
        }
        {
            let mut parse_result = TydiParser::parse(Rule::ArrayRange, "(1=1=>5)")
                .expect("unsuccessful parse");
            println!("{}",parse_result);
            let parse_result = parse_result.next().unwrap();
            let mut pass : bool = false;

            match parse_result.as_rule() {
                Rule::ArrayRange => {
                    let value: &str = parse_result.as_str();
                    if value == "(1=1=>5)" {
                        pass = true;
                    }
                }
                _ => unreachable!(),
            }
            assert!(pass);
        }
    }
    #[test]
    fn parse_basic_logical_types() {
        {
            let code = "Null";
            let mut parse_result = TydiParser::parse(Rule::LogicalNullType, code).expect("unsuccessful parse");
            println!("{}",parse_result);
            let parse_result = parse_result.next().unwrap();
            let mut pass : bool = false;

            match parse_result.as_rule() {
                Rule::LogicalNullType => {
                    let value: &str = parse_result.as_str();
                    if value == code {
                        pass = true;
                    }
                }
                _ => unreachable!(),
            }
            assert!(pass);
        }
        {
            let code = "Bit(x)";
            let mut parse_result = TydiParser::parse(Rule::LogicalBitType, code).expect("unsuccessful parse");
            println!("{}",parse_result);
            let parse_result = parse_result.next().unwrap();
            let mut pass : bool = false;

            match parse_result.as_rule() {
                Rule::LogicalBitType => {
                    let value: &str = parse_result.as_str();
                    if value == code {
                        pass = true;
                    }
                }
                _ => unreachable!(),
            }
            assert!(pass);
        }
    }
    #[test]
    fn parse_compound_logical_types() {
        {
            let code = "Union A {
  a : Bit(10),
  b : Stream(A, d=0, t=\"user type\"),
  c : Stream(A, t=\"user type\"),
  d : Stream(A, d=0),
  e : Stream(A),
}";
            let mut parse_result = TydiParser::parse(Rule::LogicalUnionType, code).expect("unsuccessful parse");
            println!("{}",parse_result);
            let parse_result = parse_result.next().unwrap();
            let mut pass : bool = false;

            match parse_result.as_rule() {
                Rule::LogicalUnionType => {
                    let value: &str = parse_result.as_str();
                    if value == code {
                        pass = true;
                    }
                }
                _ => unreachable!(),
            }
            assert!(pass);
        }
        {
            let code = "Group A { \
  a : Bit(10), \
  b : Stream(A, d=0, t=\"user type\"), \
  c : Stream(A, t=\"user type\"), \
  d : Stream(A, d=0), \
  e : Stream(A), \
}";
            let mut parse_result = TydiParser::parse(Rule::LogicalGroupType, code).expect("unsuccessful parse");
            println!("{}",parse_result);
            let parse_result = parse_result.next().unwrap();
            let mut pass : bool = false;

            match parse_result.as_rule() {
                Rule::LogicalGroupType => {
                    let value: &str = parse_result.as_str();
                    if value == code {
                        pass = true;
                    }
                }
                _ => unreachable!(),
            }
            assert!(pass);
        }

    }
    #[test]
    fn parse_examples_type_assign() {
        {
            let code = "package test;
type A = Bit(1);
type Group A {
  a : Bit(10),
  b : Bit(11),
};
type Union A {
  a : Bit(10),
  b : Bit(11),
};
type A = Union A {
  a : Bit(10),
  b : Stream(A, d=0, t=\"user type\"),
  c : Stream(A, t=\"user type\"),
  d : Stream(A, d=0),
  e : Stream(A),
};";
            let mut parse_result = TydiParser::parse(Rule::Start, code).expect("unsuccessful parse");
            println!("{}", parse_result);
            let parse_result = parse_result.next().unwrap();
            let mut pass: bool = false;

            match parse_result.as_rule() {
                Rule::Start => {
                    let value: &str = parse_result.as_str();
                    if value == code {
                        pass = true;
                    }
                }
                _ => unreachable!(),
            }
            assert!(pass);
        }
    }
    #[test]
    fn parse_examples_const_assign() {
        {
            let code = "package test;
const i = 1;
const i : int = 1;
const i : str = 1;";
            let mut parse_result = TydiParser::parse(Rule::Start, code).expect("unsuccessful parse");
            println!("{}", parse_result);
            let parse_result = parse_result.next().unwrap();
            let mut pass: bool = false;

            match parse_result.as_rule() {
                Rule::Start => {
                    let value: &str = parse_result.as_str();
                    if value == code {
                        pass = true;
                    }
                }
                _ => unreachable!(),
            }
            assert!(pass);
        }
    }
    #[test]
    fn parse_examples_streamlet0() {
        {
            let code = "package test;
streamlet string <char : type> {
  a : char out,
  b : Bit(10) out,
};
streamlet string <char_length : int> {
  a : Bit(char_length) out,
  b : Bit(10) out,
  c : Stream(A) out,
  d : Stream(A) in,
};";
            let mut parse_result = TydiParser::parse(Rule::Start, code).expect("unsuccessful parse");
            println!("{}", parse_result);
            let parse_result = parse_result.next().unwrap();
            let mut pass: bool = false;

            match parse_result.as_rule() {
                Rule::Start => {
                    let value: &str = parse_result.as_str();
                    if value == code {
                        pass = true;
                    }
                }
                _ => unreachable!(),
            }
            assert!(pass);
        }
    }
    #[test]
    fn parse_examples_streamlet1() {
        {
            let code = "package test;
type Group t{
  d:Bit(10),
};
streamlet string <char_length : int> {
  a : t out,
};";
            let mut parse_result = TydiParser::parse(Rule::Start, code).expect("unsuccessful parse");
            println!("{}", parse_result);
            let parse_result = parse_result.next().unwrap();
            let mut pass: bool = false;

            match parse_result.as_rule() {
                Rule::Start => {
                    let value: &str = parse_result.as_str();
                    if value == code {
                        pass = true;
                    }
                }
                _ => unreachable!(),
            }
            assert!(pass);
        }
    }
    #[test]
    fn parse_examples_app() {
        {
            let code = "package test;

import a;

const i = 1u;
const i = -1;
const i : int = 1;
const i : str = \"1\";

type t = Bit(10);
type A = Bit(1);
type A = Group A {
  a : Bit(10),
  b : Bit(11),
};
type A = Union A {
  a : Bit(10),
  b : Bit(11),
};
type A = Stream(A, d=0, t=\"user type\");
type A = Union A {
  a : Bit(10),
  b : Stream(A, d=0, t=\"user type\"),
  c : Stream(A, t=\"user type\"),
  d : Stream(A, d=0),
  e : Stream(A),
};

// define a streamlet
streamlet adder<N : int> {
  a : Bit(N) in,
  b : Bit(N) in,
  Out : Bit(N) out,
  overflow: Bit(1) out,
};

streamlet demux<N:int, DATATYPE : type> {
  In : DATATYPE in,
  selection : Bit(8) in,
  Out : DATATYPE[N] out,
};

// define an implementation for a streamlet
impl adder_8b <n:int> of adder<8> {
  process{},
};

streamlet adder_4 {
  a : Bit(8) in,
  b : Bit(8) in,
  c : Bit(8) in,
  d : Bit(8) in,
  overflow: Bit(1) out,
  Out : Bit(8) out,
};

impl adder_4port_8bit of adder_4port<8> {
  instance adder_1(adder_8b),
  instance adder_2(adder_8b),
  instance adder_3(adder_8b),

  self.a => adder_1.a,
  self.b => adder_1.b,
  self.c => adder_2.a,
  self.d => adder_2.b,
  adder1.Out => adder_3.a,
  adder2.Out => adder_3.b,
  adder3.Out => self.Out \"netName\",

  process{},
};

impl adder_8bit of adder<8> {
  instance adder_1(adder_8b),
  instance adder_2(adder_8b),
  instance adder_3(adder_8b),

  self.a => adder_1.a,
  self.b => adder_1.b,
  self.c => adder_2.a,
  self.d => adder_2.b,
  adder1.Out => adder_3.a,
  adder2.Out => adder_3.b,
  adder3.Out => self.Out \"netName\",

  process{},
};

impl adder_4_3adders of adder_4 {
  instance adders(adder_8b) [3],

  self.a => adders[0].a,
  self.b => adders[0].b, // maybe sugering: [self.a,self.b] => [adder_1.a, adder_1.b]
  self.c => adders[1].a,
  self.d => adders[1].b,
  adders[0].Out => adders[2].a,
  adders[1].Out => adders[2].b,
  adders[2].Out => self.Out \"netName\",

  process{},
};";
            let mut parse_result = TydiParser::parse(Rule::Start, code).expect("unsuccessful parse");
            println!("{}", parse_result);
            let parse_result = parse_result.next().unwrap();
            let mut pass: bool = false;

            match parse_result.as_rule() {
                Rule::Start => {
                    let value: &str = parse_result.as_str();
                    if value == code {
                        pass = true;
                    }
                }
                _ => unreachable!(),
            }
            assert!(pass);
        }
    }
    #[test]
    fn parse_examples_if_for() {
        {
            let code = "package test;

impl adder_4_3adders<flag:bool> of adder_4 {
    instance adders(adder_8b) [3],

    self.a => adders[0].a,
    self.b => adders[0].b, // maybe sugering: [self.a,self.b] => [adder_1.a, adder_1.b]
    self.c => adders[1].a,
    self.d => adders[1].b,
    adders[0].Out => adders[2].a,
    adders[1].Out => adders[2].b,
    adders[2].Out => self.Out \"netName\",


    for i in (0 =1=> 5) {

    }

    instance adder_1(adder_8b<0>),
    instance adder_2(adder_8b<0>),
    instance adder_3(adder_8b<1>),

    self.a => adder_1.a,
    self.b => adder_1.b, // maybe sugering: [self.a,self.b] => [adder_1.a, adder_1.b]
    self.c => adder_2.a,
    self.d => adder_2.b,
    adder1.Out => adder_3.a,
    adder2.Out => adder_3.b,
    adder3.Out => self.Out \"netName\",


  process{},
};";
            let mut parse_result = TydiParser::parse(Rule::Start, code).expect("unsuccessful parse");
            println!("{}", parse_result);
            let parse_result = parse_result.next().unwrap();
            let mut pass: bool = false;

            match parse_result.as_rule() {
                Rule::Start => {
                    let value: &str = parse_result.as_str();
                    if value == code {
                        pass = true;
                    }
                }
                _ => unreachable!(),
            }
            assert!(pass);
        }
    }
    #[test]
    fn parse_examples_parallel_adder() {
        {
            let code = "package test;

// define a streamlet
streamlet adder<N : int> {
  a : Bit(N) in,
  b : Bit(N) in,
  c: Null out,
  Out : Bit(N) out,
  overflow: Bit(1) out,
};

// define an implementation for a streamlet
impl adder_8b <n:int> of adder<8> {
  process{},
};

streamlet adder <N: int> {
  inputs: Bit(8) [N] in,
  overflow: Bit(1) [N / 2] out,
  Out : Bit(8) [N / 2] out,
  global_overflow : Bit(1) out,
};

impl adder_ <N:int> of adder <N> {
  instance adders(adder_8b) [N / 2],
  instance and_gate(std_and_gate<N / 2>),
  for i in (0=1=>N/2) {
    self.inputs[i*2] => adders[i].a,
    self.inputs[i*2+1] => adders[i].b,
    adders[i].overflow => self.overflow[i],
    adders[i].overflow => and_gate.input[i],
    adders[i].Out => self.Out,
  }
  and_gate.output => self.overflow,

  process{},
};";
            let mut parse_result = TydiParser::parse(Rule::Start, code).expect("unsuccessful parse");
            println!("{}", parse_result);
            let parse_result = parse_result.next().unwrap();
            let mut pass: bool = false;

            match parse_result.as_rule() {
                Rule::Start => {
                    let value: &str = parse_result.as_str();
                    if value == code {
                        pass = true;
                    }
                }
                _ => unreachable!(),
            }
            assert!(pass);
        }
    }
    #[test]
    fn parse_examples_delayed_net() {
        {
            let code = "package test;
streamlet adder<N:int> {
  inputs: Bit(8) [N] in,
  Out : Bit(8) [N/2] in,
  overflow : Bit(1) out,
};

impl adder_ <N:int> of adder <N> {
  instance adders(adder_8b) [N / 2],
  instance and_gate(std_and_gate<N / 2>),
  for i in (0=1=>N/2) {
    self.inputs[i*2] => adders[i].a,
    self.inputs[i*2+1] => adders[i].b,
    adders[i].overflow => self.overflow[i],
    adders[i].overflow => and_gate.input[i],
    adders[i].Out =1=> self.Out,
  }
  and_gate.output => global_overflow,

  process{},
};";
            let mut parse_result = TydiParser::parse(Rule::Start, code).expect("unsuccessful parse");
            println!("{}", parse_result);
            let parse_result = parse_result.next().unwrap();
            let mut pass: bool = false;

            match parse_result.as_rule() {
                Rule::Start => {
                    let value: &str = parse_result.as_str();
                    if value == code {
                        pass = true;
                    }
                }
                _ => unreachable!(),
            }
            assert!(pass);
        }
    }
    #[test]
    fn parse_examples_instance() {
        {
            let code = "package test;

impl adder_ <N:int> of adder <N> {
  instance adders(adder_8b) [N / 2],
  instance and_gate(std_and_gate<N / 2>),
  for i in (0=1=>N/2) {
    self.inputs[i*2] => adders[i].a,
    self.inputs[i*2+1] => adders[i].b,
    adders[i].overflow => self.overflow[i],
    adders[i].overflow => and_gate.input[i],
    adders[i].Out =1=> self.Out,
  }
  and_gate.output => global_overflow,

  process{},
};

impl add_8(adder_<8>);";
            let mut parse_result = TydiParser::parse(Rule::Start, code).expect("unsuccessful parse");
            println!("{}", parse_result);
            let parse_result = parse_result.next().unwrap();
            let mut pass: bool = false;

            match parse_result.as_rule() {
                Rule::Start => {
                    let value: &str = parse_result.as_str();
                    if value == code {
                        pass = true;
                    }
                }
                _ => unreachable!(),
            }
            assert!(pass);
        }
    }
    #[test]
    fn parse_examples_mux_demux() {
        {
            let code = "package test;

streamlet process_unit<inputT:type, outputT:type> {
  input: inputT in,
  output : outputT out,
};

impl process_unit_0<inputT:type, outputT:type, delay:int> of process_unit<inputT> {
  instance single_PU(PU) [delay],
  instance demux(std_demux<inputT, delay>),
  instance mux(std_mux<outputT, delay>),

  self.input => demux.input,
  mux.output => self.output,
  for i in (0=1=>delay) {
    demux.out[i] => single_PU[i].input,
    single_PU[i].output => mux.input[i],
  }

};";
            let mut parse_result = TydiParser::parse(Rule::Start, code).expect("unsuccessful parse");
            println!("{}", parse_result);
            let parse_result = parse_result.next().unwrap();
            let mut pass: bool = false;

            match parse_result.as_rule() {
                Rule::Start => {
                    let value: &str = parse_result.as_str();
                    if value == code {
                        pass = true;
                    }
                }
                _ => unreachable!(),
            }
            assert!(pass);
        }
    }
    #[test]
    fn parse_external() {
        {
            let code = "package test;

import external_package;

const flag = true;
const num_instance = 8;
const num_stream = 2;

const external_var = pack.a;
const f = 1.0;
const g = 1;
//const v = f/g;
const v = g/f;

type f = Bit(1);

type location = Union location_ {
  const i = 1,
  x : external.location,
  y : Bit(32),
};

type stream0 = Stream(Bit(4));
type stream1 = Stream(Bit(8));

streamlet sl0<i:int, t:type> {
  const i0 = 1,
  port_in : stream0 in,
  port_out : stream0 out,

  in_ : stream1 in,

  in_array : t [num_stream] in,
  out_array : t [num_stream] out,
};

streamlet sl1 {
  port_in : stream0 in,
  port_out : stream0 out,

  in_array : stream0 [num_stream] in,
  out_array : stream0 [num_stream] out,
};

streamlet sl2 {
  port_in : external. stream0 in,
  port_out : stream0 out,

  in_array : stream0 [num_stream] in,
  out_array : stream0 [num_stream] out,
};

impl temp_impl of external.sl0<1, type Bit(1)> {

};
impl tmux<n: int, ts: impl of sl0<num_instance, type Bit(1)>> of sl0<n, type stream0> {
  const i = 10,
  instance test_inst(ts),
  instance external_inst(external.streamlet),

  test_inst.Out => test_inst.In,

  process{},
};
impl test of sl0<1, type Bit(1)> {
  instance inst0(tmux<1, impl test>),
};

//////////////////////////////REGION2

impl temp_impl2 of sl1 {

};
impl tmux2<n: int, ts: impl of sl1> of sl0<n, type stream0> {
  instance test_inst(ts),

  test_inst.Out =1=> test_inst.In,

  process{},
};
impl test2 of sl1 {
  instance inst0(tmux2<1,impl temp_impl2>),
};


//////////////////////////////REGION3
streamlet sl4<i:int> {
  port_in : stream0 in,
  port_out : stream0 out,
};


impl temp_impl3 of sl4<1> {

};
impl tmux3<n: int, ts: impl of external.sl4<1> > of sl0<n, type stream0> {
  instance test_inst(ts),
  test_inst.Out => test_inst.In,
  process{},
};
impl test3 of sl1 {
  instance inst0(tmux3<1, impl e.temp_impl3>),
};";
            let mut parse_result = TydiParser::parse(Rule::Start, code).expect("unsuccessful parse");
            println!("{}", parse_result);
            let parse_result = parse_result.next().unwrap();
            let mut pass: bool = false;

            match parse_result.as_rule() {
                Rule::Start => {
                    let value: &str = parse_result.as_str();
                    if value == code {
                        pass = true;
                    }
                }
                _ => unreachable!(),
            }
            assert!(pass);
        }
    }
    #[test]
    fn parse_clockdomain() {
        {
            let code = "package test;

import external_package;

const flag = true;
const num_instance = 8;
const num_stream = 2;

const ck = 10MHz;

streamlet sl1 {
  port_in : stream0 in `200MHz,
  port_out : stream0 out `200MHz,

  in_array : stream0 [num_stream] in `ck,
  out_array : stream0 [num_stream] out `ck,
};
";
            let mut parse_result = TydiParser::parse(Rule::Start, code).expect("unsuccessful parse");
            println!("{}", parse_result);
            let parse_result = parse_result.next().unwrap();
            let mut pass: bool = false;

            match parse_result.as_rule() {
                Rule::Start => {
                    let value: &str = parse_result.as_str();
                    if value == code {
                        pass = true;
                    }
                }
                _ => unreachable!(),
            }
            assert!(pass);
        }
    }
}
