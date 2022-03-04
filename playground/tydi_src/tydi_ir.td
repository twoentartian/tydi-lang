package tydi_ir;

type bit8_stream = Stream(Bit(8), d = 5, t = 2.5);

const eight = 8;

type Group rgb {
  r: Bit(eight),
  g: Bit(eight),
  b: Bit(eight),
};

type rgb_stream = Stream(rgb);


streamlet data_bypass<data_type: type> {
  input: data_type in,
  output: data_type out,
};

impl impl_data_bypass<data_type: type> of data_bypass<type data_type> {
  input => output,
};

streamlet data_demux<channel:int, data_type: type, cd:clockdomain> {
  inputs: data_type [channel] in `cd,
  outputs: data_type [channel] out `cd,
};


impl impl_data_demux<channel:int, data_type: type, cd:clockdomain> of data_demux<channel, type data_type, cd> {
  instance bypass(impl_data_bypass<type data_type>) [channel],
  for i in (0=1=>channel) {
    bypass[i].output => outputs[i],
    inputs[i] => bypass[i].input,
  }
};

const cd0 = 100MHz;

impl data_demux_rgb_5(impl_data_demux<5, type rgb_stream, cd0>);
impl data_demux_rgb_100(impl_data_demux<20, type rgb_stream, 50kHz>);
