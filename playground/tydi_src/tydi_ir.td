package tydi_ir;

type bit8_stream = Stream(Bit(8), d = 5, t = 2.5);

type Group rgb {
  r: Bit(8),
  g: Bit(8),
  b: Bit(8),
};

type rgb_stream = Stream(rgb);


streamlet data_bypass<data_type: type> {
  input: data_type in,
  output: data_type out,
};

impl impl_data_bypass<data_type: type> of data_bypass<type data_type> {
  input => output,
};

streamlet data_demux<channel:int, data_type: type> {
  inputs: data_type [channel] in,
  outputs: data_type [channel] out,
};

impl impl_data_demux<channel:int, data_type: type> of data_demux<channel, type data_type> {
  instance bypass(impl_data_bypass<type data_type>) [channel],
  for i in (0=1=>channel) {
    bypass[i].output => outputs[i],
    inputs[i] => bypass[i].input,
  }
};

impl data_demux_rgb_5(impl_data_demux<5, type rgb_stream>);
impl data_demux_rgb_100(impl_data_demux<5, type rgb_stream>);
