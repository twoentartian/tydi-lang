package tydi_ir; //package name must be the same with the file name

streamlet s {
  port_in: Stream(Bit(8)) in,
  port_out: Stream(Bit(8)) out,
};

external impl i of s {

};

impl i2 of s {
  instance inner(i),

  inner.port_out => port_out,
  port_in => inner.port_in,
};