use std::slice;

fn main() {
    // Unsafe rust offers up the following extra abilities (super powers)
    // * Dereference raw pointers
    // * Call Unsafe methods or functions
    // * Access or modify mutable Static variables
    // * Implement an unsafe trait
    // * Access fields within a Union
    //
    // Nothing else changes. The borrow checker and all other assurances (other than those listed
    // above) still work as normal

    // Dereferencing a Raw Pointer //
    let mut num = 5;

    // Creating raw pointers is fine in standard Rust. You only run in to issues when trying to
    // access them
    let r1 = &num as *const i32; // create an immutable raw pointer from as reference
    let r2 = &mut num as *mut i32; // create an mutable raw pointer from as reference

    // Raw pointers can be mutable and immutable, just like all other types
    // mutable = *const <type>
    // immutable = *mut <type>
    // *const is type, we are not Dereferencing anything here, the * is part of the type name

    // This will raise a compile error when dereferenced outside of a unsafe block
    // println!("r1 is : {}", *r1);

    unsafe {
        println!("r1 is : {}", *r1);
        println!("r2 is : {}", *r2);
    }

    // Calling an Unsafe Function or Method //
    unsafe fn dangerous() {}

    // Again, calling this function outside of an unsafe block will raise a compile error
    // dangerous();
    unsafe {
        dangerous();
    }

    // Creating a safe abstraction around unsafe code //
    //fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    //    // This exists in the standard library and we implement here for the purpose of learning
    //    let len = values.len();

    //    assert!(mid <= len);

    //    // This will raise a compile error as Rust thinks we are trying to borrow
    //    // the same data mutably more than once. It does not know that the data does not overlap
    //    // We cannot do this without using unsafe Rust!
    //    (&mut values[..mid], &mut values[mid..])
    //}

    fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        // Even though we are using unsafe code, we have created a safe abstraction around it.
        // You do not need to use unsafe to call this function, for it is safe
        let len = values.len();

        // A slice is a pointer to some data and its length
        // We use .as_mut_ptr it get the raw pointer to the slice
        let ptr = values.as_mut_ptr(); // type of pointer is *mut i32

        assert!(mid <= len);

        unsafe {
            // slice::from_raw_parts_mut - Creates a slice from a raw pointer and a length
            // This is unsafe as it must trust the that pointer is valid
            //
            // ptr.add() is also unsafe as it must trust that the offset is also a valid pointer
            //
            // In this case we know they are valid as we assert that the mid is within the length
            // of the slice - This is a good use of unsafe
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // This is a bad use of unsafe
    let address = 0x01234usize; // Raw memory address
    let r = address as *mut i32;

    // This will likey crash when the slice is used as we have no idea what this slice will
    // contain, or even if the memory location is valid
    let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };

    // Using Extern to call External Code
    //
    // This allows us to create and use an FFI (Foreign function Interface) interface
    // this allows us to call a function from another language
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    // The FFI is unsafe as other languages do not offer the same assurances as Rust, it's up to
    // the programmer to make sure all is OK, this is why it is unsafe
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // Accessing or Modifying a Mutable Static Variable //

    // static variable are Rusts globals
    static HELLO_WORLD: &str = "Hello, world!";
    println!("Don't forget to say: {}", HELLO_WORLD);

    // The difference between static variables and constants is that static variables have a fixed
    // address in memory. Constants can duplicate their values when they are used
    // Also static variables can be mutable
    // Accessing and modifying a static variable is unsafe
    static mut COUNTER: u32 = 0;

    fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }

    add_to_count(3);

    // accessing and modifying is unsafe mostly because of possible race conditions in theads.
    // This is single threaded so is fine
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    // Implementing an Unsafe Trait //

    // Aa unsafe trait is one that has same invariants that the compilier can not verify
    unsafe trait Foo {
        // methods here
    }

    unsafe impl Foo for i32 {
        // methods here
    }

    // Accessing Fields of a Union //
    // A union is similar to a struct, but only one declared field is used in a particular instance at one time
    // Unions are only really used to interface between C so no examples here
}
