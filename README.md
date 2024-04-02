# uintx
Unaligned integer types in rust with arithmetic operations

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

Each of these types implement and provide all the functionality of u32.
In addition to that they also overload all operators for the next largest aligned number.
Example:
```rust
#[test]
fn test() {
  let mut num : u24 = u24::from(12u32);
  num += 1u32;
  num = num + 1u32;
  num += u24::from(4);
  num = num + u24::from(4);
  
  assert_eq!(num, 22)
}
```

### Additional optional crate features
#### num_traits_support
Enabling this feature causes all types of this crate to implement the
PrimInt trait and all required super traits from the num_traits crate.

#### ux_support
Enabling this feature enables Into and From conversions for all numeric types provided by
the ux crate. This can be useful if you want to read unaligned data from a block of memory
but need to perform signed math on it. It is also useful if your codebase already uses the ux crate.

Conversions from uintx to ux and vice versa never fail. They always clamp the values.
So converting an u24 into an ux::u12 will never fail but all bits beyond bit #12 are discarded.
The same goes for converting an ux::u28 into an u24. This will discard all bits beyond bit #24.

#### intx_support
Enabling this feature enables Into and From conversions for all numeric types provided by
the intx crate. This is mainly useful if you need signed numbers with an accurate sizeof.

Beware that the intx crate does not implement any arithmetic operations so this is probably only
useful if you already use intx in your codebase.

#### unsafe_fetch
This feature provides some functions for every type to perform arithmetic operations
using less fetch instructions (and fewer instructions overall) compares to the
memory safe std::ops. It all boils down to how the CPU fetches the data from memory,
so your mileage may vary depending on the CPU architecture. 
I am mainly targeting x86_64 with this feature.

A normal add operation of 2 u24 ints would compile into 2 fetches+1 shift for each u24, 
then an add operation, then to store the result 2 stores+1 shift.
This is memory safe but not the fastest.

The function: u24::unsafe_add_with_aligned_into_aligned(u24, u32) -> u32

would compile into 1 fetch+1 and for the u24 and 1 fetch for the u32, then an add operation, then 1 store. 
This is faster than the above. Keep in mind that this function implicitly transforms the output to an u32.
There are variants of this function that output an u24 too or accept 2 u24s as input. They require 
a bit more instructions but still fewer than the memory save "+" operator.

Why are these unsafe functions not memory safe?

You probably know already that the CPU has no instruction to fetch just 3 bytes from memory,
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
not faster when using the unsafe functions. CPU Architectures that do not permit unaligned
memory access for all primitive types will probably cause a SIGBUS if this feature is used. 
(Again I recommend to use this feature with x86_64 only)

### Alternatives
As mentioned in the features the intx and ux library provide similar functionality than this crate.

My personal reasons for not using them are as follows
#### intx
- No arithmetic operations implemented. If you need more
  than println() or conversion into an aligned type then your back to implementing everything on your own.
- No clamping constructors for any of the types. There is only try_into and that requires error handling.
  - Ex: If you need the number 0xAFFFF as an U24 then you need to use try_into or from_ne_bytes.
- You cannot have any constants from intx types as none of the constructors are const.
#### ux
- The values are not the size they claim to be. I.e. ux::u24 has 4 bytes in memory 
  making them unsuitable for slices of existing unaligned data that is read from a file for example.
- All types beyond u63/i63 are not really implemented properly?
  - All Into and From implementations for those values are just missing.
  - I need stuff like u72 to work properly.

### Motivation for making this library
This library was developed to assist in dealing with a lot of RGB,BGR, CMYKOG, CMYKOGV,... 
pixel buffers in esoteric "Pixel" formats where arithmetic operations have 
to be performed before the pixel represents an actual valid value. 
Doing this with many &| and shift operations leads to very unreadable code.