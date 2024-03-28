# uintx
A rust library that implements unaligned unsigned integer types with arithmetic functions
that have exactly the size of the unaligned type in memory.

### Implemented types
- u24
- u40
- u48
- u56
- u72
- u80
- u88
- u96
- u104
- u112
- u120

### Unsafe arithmetic functions
In addition to the normal std::ops each type offers unsafe functions. 
These functions depending on the CPU architecture compile into
fewer instructions than using the regular std::ops (ex: "+" operator). 
It all boils down to how the CPU fetches the data from memory. 

#### A normal add operation of 2 u24 ints 
would compile into 2 fetches+1 shift for each u24, then an add operation, then to store the result 2 stores+1 shift.
This is memory safe but not the fastest.

#### The function: u24::unsafe_add_with_aligned_into_aligned(u24, u32) -> u32

would compile into 1 fetch+1 and, then an add operation, then 1 store. This is faster than the above.
Keep in mind that this function implicitly transforms the output to an u32.
There are variants of this function that output an u24 too or accept 2 u24s as input.

#### Why are these unsafe functions not memory safe?
You can probably know already that the CPU has no instruction to fetch just 3 bytes from memory,
The unsafe functions always fetch the next largest possible type. In u24 this would be u32.
So this function will read 1 more byte than a buffer (slice/vec) may have. The additional byte does not
have an influence on the computation result, but it is read nonetheless. This may cause your application
to segfault or cause other unintended consequences (memory mapped io for example may do something funny)
This is pretty much exclusively intended to operate on buffers that are padded to have a size dividable by 
the next largest aligned integer. So for u24 the buffer (slice/vec) should have a size in bytes dividable by 4
to safely use the unsafe operations. I would not recommend on using this on stack variables. 
(The layout of the stack is hard/impossible to predict). If you don't want to deal with this then I recommend
you do not use the unsafe functions and stick to the safe std::ops. I also recommend verification of the compiled
code using a disassembler when using the unsafe functions. Some Integers (depending on size/architecture) are
not faster when using the unsafe functions.

### Alternatives
If you need signed/other integers consider using the "ux" or "intx" library. 

The main differences are:
- "ux" uses the next biggest aligned integer to represent the type in memory. (sizeof::<u24> == 4)
- "intx" does not implement arithmetic operations but (sizeof::<u24> == 3 YAY!)

### Motivation:
This library was developed to assist in dealing with a lot of RGB,BGR, CMYKOG, CMYKOGV,... pixel buffers in esoteric "Pixel" formats where 
gamma corrections and other arithmetic operations have to be performed before the pixel represents
an actual valid value. Doing this with native u32 is somewhat prone to errors.

Ex: RGB & BGR
Having a 3 byte u24 allows me to take a slice from the pixel raster raw pointer. 
This rules out the ux library and processing the raster as an u32 directly would require
manual iteration over every byte and shifting it into an u32 manually. Because I have to process absolutely
HUGE images performance becomes a concern rather quickly and using intx and converting to u32 for example
leads to cumbersome code and not the best possible performance due to the absence of the above-mentioned unsafe functions.

### Future work:
I will probably have to implement integers larger than u128 sometime in the future, 
because 10 channel 16 bit per channel pixels (CMYKOGV+W&V+Alpha) are a thing.

I may also implement sign extender functionality to convert from u24 -> i24 -> i32.
Currently, conversions from u24 to i32 are implemented like this: u24 -> u32 -> i32. (its impossible to get a negative i32 from an u24)
But this is not needed for my purposes. So I may end up not doing it.