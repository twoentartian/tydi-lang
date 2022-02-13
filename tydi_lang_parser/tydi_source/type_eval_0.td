package type_eval_0;

import type_eval_1;

const value_1 = 1;

type logical_null = Null;
type logical_null2 = logical_null;
type bit1 = Bit(1+1);
type bit1_alias = bit1;
type bit16 = Bit(16);

type Group group0 {
    const value_2 = 4,
    a : Bit(4 * 16),

};

type Union union0 {
    const value_2 = 8,
    a : Bit(value_2 * 2 + value_1),
    external: type_eval_1.bit16,
};

type stream0 = Stream(group0, d = 1, u = union0, t = 1.5*5, s = "flatten", c = 5, r = "Reverse", x = false);

