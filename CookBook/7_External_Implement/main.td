package main;

streamlet s {
  port_in: Stream(Bit(8)) in,
  port_out: Stream(Bit(8)) out,
};

external impl i of s {
  process {

  },
};//all contents in external implement will be ignored except for simulation processing.

impl i2 of s {
  instance inner(i),

  inner.port_out => port_out,
  port_in => inner.port_in,
};