package tydi_ir; //package name must be the same with the file name

const cd: clockdomain;
const cd2: clockdomain = "cd_1";

streamlet s {
  port_in: Stream(Bit(8)) in `cd2,
  port_out: Stream(Bit(8)) out `cd2,
};

external impl i of s {

};

impl i2 of s {
  instance inner(i),

  inner.port_out => port_out,
  port_in => inner.port_in,
};