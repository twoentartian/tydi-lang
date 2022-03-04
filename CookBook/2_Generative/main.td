package main; //package name must be the same with the file name

type bit8_stream = Stream(Bit(8), d = 5, t = 2.5);

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
  inputs: bit8_stream [channel] in `10kHz,
  outputs: bit8_stream [channel] out `10kHz,
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
