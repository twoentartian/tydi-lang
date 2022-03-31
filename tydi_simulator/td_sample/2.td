package test;

type stream_byte = Stream(Bit(8));

streamlet byte_bypass {
  input: stream_byte in,
  output: stream_byte out,
};

impl byte_bypass1 of byte_bypass {
    input => output,
};

impl byte_bypass2 of byte_bypass {
    instance inst(byte_bypass1),

    input => inst.input,
    inst.output => output,
};