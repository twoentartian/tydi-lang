package tydi_ir;

type bit8_stream = Stream(Bit(8), d = 5, t = 2.5);

type Group group0 {
  r: Bit(8),
  g: Bit(8),
  b: Bit(8),
};

type rgb_stream = Stream(group0);

#streamlet docu basic0#
streamlet basic0 {
  #port document#
  d_in: bit8_stream in,
  d_out: bit8_stream out,
};

#impl docu basic0_1#
impl basic0_1 of basic0 {

};

#impl docu impl_basic0#
impl impl_basic0 of basic0 {

  #instance document#
  instance test_inst(basic0_1),

  #connection document#
  d_in =1=> test_inst.d_in,
  test_inst.d_out => d_out,
};

impl impl_template<i:int> of basic0 {
  instance test_inst(basic0_1) [i],
};

streamlet streamlet_template<i:int, t:type> {
  d_in: t [i] in,
  d_out: t out,
  test: Stream(Bit(8)) in,
};

impl impl_template2<i:int> of streamlet_template<i, type rgb_stream> {
  instance test_inst(basic0_1) [i],
};

impl impl_template_instance_1(impl_template<3>);

impl impl_template_instance_2(impl_template2<5>);