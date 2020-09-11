Bit serde
==========

Bitserde is a toy project of mine to have a solution similar to bincode, except the inner buffers comprised of BitVecs and slices. It is very slow, around 4.4x slower then bincode. I haven't measured memory usage, but I presume it will consume more.

It doesn't support maps at the moment but this will be added in the future. Other features will need to be implemented on your own such as parsing strings; they are out of scope since these change depending on the binary format.