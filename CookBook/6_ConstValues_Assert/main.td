package main; //package name must be the same with the file name

const x = 4;

type Group rgb {
  const x = 8, //shadowing

  assert(x == 8),

  r: Bit(x),
  g: Bit(x),
  b: Bit(x),
};

assert(type rgb.x == 8);
assert(x == 4);

streamlet component<data:type> {
  const data = 8,
  const x = type data.x, //deliver the x to the outer scope

  assert(x == 8),

  input: Stream(data) in,
  output: Stream(data) out,
};

assert(streamlet component<type rgb>.x == 8);

impl component_impl<data:type> of component<type data> {
  const x = streamlet component<type data>.x, //deliver the x to the outer scope
  assert(x == 8),

  if (true) {
    assert(x == 8),
  }
  else {
    assert(x == 7),
  }

  for i in (1=1=>2) {
    assert(x == 8),
  }

  input => output,
};

assert(impl component_impl<type rgb>.x == 8);

impl component_impl0(component_impl<type rgb>);