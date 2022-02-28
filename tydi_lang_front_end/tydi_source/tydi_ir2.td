package tydi_ir2;

import tydi_ir;


/*
streamlet streamlet1<data_cd1 : clockdomain, data_cd2 : clockdomain, data_type: type> {
  data1_in: data_type in 'data_cd1
  data1_out: data_type out 'data_cd1

  data2_in: data_type in 'data_cd2
  data2_out: data_type out 'data_cd2

};

clockdomain cd1 = 200MHz;

impl impl1<ck:clockdomain> of streamlet<ck, ck, Stream(Bit(8))> {
  ...
};

clockdomain cd2;

impl impl2 of streamlet<cd2, cd1, Stream(Bit(16))> {
  ...
};

*/






impl component of streamlet {

  data_in_0 : in Stream(Bit(8)),
  data_out_0 : out Stream(Bit(8)),
  data_in_1 : in Stream(Bit(8)),
  data_out_1 : out Stream(Bit(8)),

  process {
    state component_state = "0"; // init state as 0

    set_ack(data_in_0, 2);
    set_ack(data_in_1, 1);

    event receive(data_in_0) && receive(data_in_1) {

      if (component_state == "0") {
        ...
        delay(5ns);
        send(data_out_0, 0b11110000);

        //for composite data types: Group(a: Bit(8), b: Bit(8))
        //send(data_out_0, Group(a=0x11110000, b=0x11110000));
        //send(data_out_0, Union(a=0x11110000));
        //for composite data types: Group(a: Bit(8), b: Stream(Bit(8)))
        //send(data_out_0.b, 0x11110000);
        //
        //

        component_state = "1";
      }
      else if (component_state == "1") {


        component_state = "2";
      }
      else if (component_state == "2") {


        component_state = "1";
      }


      ack(data_in_1);
      ack(data_in_0);
    };

    event data_in_0 {


      ack(data_in_0);
    };
  },

}