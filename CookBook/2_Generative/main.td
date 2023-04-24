package main;

type bit8_stream = Stream(Bit(8), d = 5, t = 2.5);

type bit8 = Bit(8);

type bit8_1 = Bit(8);

//data_bypass
streamlet data_bypass {
  input: bit8_stream in,
  output: bit8_stream out,
};

impl impl_data_bypass of data_bypass {
  input => output,
};

//data_bypass2
streamlet data_bypass2 {
  input: bit8_stream in,
  output: bit8_stream out,
};

impl impl_data_bypass2 of data_bypass2 {
  input => output,
};


const channel = 10;

streamlet data_bypass_channel {
  inputs: bit8_stream [channel] in `"10kHz",
  outputs: bit8_stream [channel] out `"10kHz",
};

const use_data_bypass2 = true;
impl impl_data_bypass_channel of data_bypass_channel {
  if (use_data_bypass2) {
    instance bypass(impl_data_bypass) [channel],
    for i in (0=1=>channel) {
      bypass[i].output => outputs[i],
      inputs[i] => bypass[i].input,
    }
  }
  //elif ()
  else {
    instance bypass(impl_data_bypass2) [channel],
    for i in (0=1=>channel) {
      bypass[i].output => outputs[i],
      inputs[i] => bypass[i].input,
    }
  }


};






//void component, always acknowledge the handshake
streamlet void_s<type_in: type> {
  input: type_in in,
};

external impl void_i<type_in: type> of void_s<type type_in> {

};

//data duplicator
streamlet duplicator_s<data_type: type, output_channel: int> {
  input: data_type in,
  output: data_type [output_channel] out,
};

external impl duplicator_i<data_type: type, output_channel: int> of duplicator_s<type data_type, output_channel> {

};