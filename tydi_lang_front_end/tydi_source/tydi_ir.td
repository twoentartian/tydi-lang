package tydi_ir;

type bit8_stream = Stream(Bit(8), d = 5);

streamlet basic0 {
  in: bit8_stream in,
  out: bit8_stream out,
};