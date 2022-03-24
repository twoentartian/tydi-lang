package tydi_ir;

const cd = 100MHz;

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
  d_in: bit8_stream in `cd,
  d_out: bit8_stream out `cd,
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

  process {
    state component_state = "0"; // init state as 0
    set_ack(data_in_0, 2);
    set_ack(data_in_1, 1);

    event receive(data_in_0) && receive(data_in_1) {
      if (component_state == "0") {
        delay_cycle(5, 100MHz);
        send(data_out_0, 0b11110000);
        //do we need read(data_in_0)?

        //for composite data types: Group(a: Bit(8), b: Bit(8))
        send(data_out_0, Group(a=0x11110000, b=0x11110000));
        send(data_out_0, Union(a=0x11110000));
        //for composite data types: Group(a: Bit(8), b: Stream(Bit(8)))
        send(data_out_0->b, 0x11110000);
        //
        //

        assign component_state = "1";
      }
      elif (component_state == "1") {
        assign component_state = "2";
      }
      elif (component_state == "2") {
        assign component_state = "1";
      }
      ack(data_in_1);
      ack(data_in_0);
    };

    event receive(data_in_0) {
      ack(data_in_0);
    };

  },


};

streamlet streamlet_template<i:int, t:type, cd: clockdomain> {
  d_in: t [i] in `cd,
  d_out: t out `cd,
  test: Stream(Bit(8)) in `cd,
};

impl impl_template2<i:int, cd: clockdomain> of streamlet_template<i, type rgb_stream, cd> {
  instance test_inst(basic0_1) [i],
};

impl impl_template_instance_1(impl_template<3>);

impl impl_template_instance_2(impl_template2<5, 500kHz>);


//impl1




//impl2