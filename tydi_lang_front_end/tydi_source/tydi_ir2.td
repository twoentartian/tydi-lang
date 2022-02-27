package tydi_ir2;

import tydi_ir;


/*
streamlet streamlet1<data_cd1 : clockdomain, data_cd2 : clockdomain, data_type: type> {
  data1_in: data_type in 'data_cd1
  data1_out: data_type out 'data_cd1

  data2_in: data_type in 'data_cd2
  data2_out: data_type out 'data_cd2

};

clockdomain cd1;

impl impl1<ck:clockdomain> of streamlet<ck, ck, Stream(Bit(8))> {
  ...
};

clockdomain cd2;

impl impl2 of streamlet<cd2, cd1, Stream(Bit(16))> {
  ...
};


*/