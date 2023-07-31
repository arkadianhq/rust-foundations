fn main() {
    // Methods ending with a ! are macros. Macros are used for metaprogramming
    println!("Hello World");

    // Regular comments
    /*
       Block comments
    */

    /*
       Primitives
       * Scalar Types
       * Compound Types
      let <variable_name>: <variable_type> = <literal>;
    */
    let boolean: bool = true;

    let signed_int_8_bit: i8 = std::i8::MAX;
    let signed_int_16_bit: i16 = std::i16::MAX;
    let signed_int_32_bit: i32 = std::i32::MAX;
    let signed_int_64_bit: i64 = std::i64::MAX;

    let unsigned_int_8_bit: u8 = std::u8::MAX;
    let unsigned_int_16_bit: u16 = std::u16::MAX;
    let unsigned_int_32_bit: u32 = std::u32::MAX;
    let unsigned_int_64_bit: u64 = std::u64::MAX;

    let float_32: f32 = 12.2;
    let float_64: f64 = 123.123;

    let empty: () = ();

    let char_4_bytes_unicode = 'a';

    println!("{}", boolean);
    println!("{}", signed_int_8_bit);
    println!("{}", signed_int_16_bit);
    println!("{}", signed_int_32_bit);
    println!("{}", signed_int_64_bit);
    println!("{}", unsigned_int_8_bit);
    println!("{}", unsigned_int_16_bit);
    println!("{}", unsigned_int_32_bit);
    println!("{}", unsigned_int_64_bit);
    println!("{}", float_32);
    println!("{}", float_64);
    println!("{:?}", empty);
    println!("{}", char_4_bytes_unicode);
}
