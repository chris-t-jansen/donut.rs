# `donut.rs`

This is a simple reimplementation of Andy Sloane's (@a1k0n) famous `donut.c` program by me for learning purposes. The code in this repository is specifically based on the code from the post ["donut.c without a math library"](https://www.a1k0n.net/2021/01/13/optimizing-donut.html), published January 13, 2021.

For the original 2006 version of `donut.c`, see ["Have a donut."](https://www.a1k0n.net/2006/09/15/obfuscated-c-donut.html) For the 2011 revision of `donut.c` and author's explanation of the theory behind the code, see ["Donut math: how donut.c works"](https://www.a1k0n.net/2011/07/20/donut-math.html).


## Running `donut.rs`

Despite the tongue-and-cheek name, the actual source file is a `main.rs` file like any standard Rust binary generated from `cargo new`. Running the code should be as simple as downloading the code, navigating to the folder within a terminal, and running `cargo run`.


## From C to Rust

#### Logical Changes

While I tried to remain as true as possible to the logic of the original `donut.c` program, rewriting the code in Rust obviously necessitated a number of changes due to Rust's strict type system.

For example, the original code initializes both the text buffer and z-buffer as `int8_t` arrays in a single line above the `main()` function:
```c
int8_t b[1760], z[1760];
```
These are then assigned using two `memset()` calls at the start of the outermost `for` loop:
```c
memset(b, 32, 1760);  // text buffer
memset(z, 127, 1760);   // z buffer
```
The `32` in the first line gets converted to a whitespace ` ` char when printed to the screen, while the `127` is the maximum value of the `int8_t` type.

To implement this in Rust, then, the initialization got moved to the start of the `main()` function and became:
```rust
let mut buffer: [char; BUFFER_SIZE];
let mut z_buffer: [i8; BUFFER_SIZE];
```
Here, `BUFFER_SIZE` is a `const` defined earlier as `const BUFFER_SIZE: usize = 1760;`. This wasn't strictly necessary, but felt like a reasonable change to me, as I wasn't concerned with the original author's goal of code that "runs well on embedded devices which can do 32-bit multiplications and have ~4k of available RAM".[^1]
[^1]: https://www.a1k0n.net/2021/01/13/optimizing-donut.html

Assignment, then, became:
```rust
buffer = [' '; BUFFER_SIZE];
z_buffer = [i8::MAX; BUFFER_SIZE];
```
Because numerical types like `i8` in Rust aren't natively interoperable with the character type `char`, I defined the `buffer` variable instead as a `char` array, and here initialize it directly with a ` ` character. Similarly, `z_buffer` instead uses the constant provided by the standard library, `i8::MAX`.

Another logical change is the conversion to the platform-dependant `usize` type. In Rust, collections that can be accessed via indexing (some variation of `array[i]` in most languages) must be accessed with an index of type `usize`. Because of this, variables used for indexing in the original code, which are `int` types that easily interoperate with the rest of the variables, have to be converted to `usize` types in Rust. This occurs twice:
```rust
let luminance_index: i32 = (-1 * cos_A * x7 - cos_B * ((-1 * sin_A * x7 >> 10) + x2) - cos_i * (cos_j * sin_B >> 10) >> 10) - x5 >> 7;
let luminance_index: usize = usize::try_from(luminance_index).unwrap_or(0);

let o: usize = (x as usize) + ((y as isize).wrapping_mul(80) as usize) % BUFFER_SIZE;
```
In the first case with the `luminance_index` varaible, I've simply used Rust's shadowing to perform the conversion over two lines without having to use some funny `lum_index_integer` varable name, a feature which I'm quite a fan of in Rust.

The `.unwrap_or(0)` at the end of the second line is another convenience afforded by Rust. Because the `luminance_index` of a point will be negative when that particular point is facing away from the imaginary light source in the scene, the conversion from a signed type (`i32`) to an unsigned type (`usize`) will fail sometimes. `.unwrap_or(...)` allows us to convert the `luminance_index` variable from`i32` to `usize`, but provide a specific value as a fallback if the conversion fails (in this case, `0`). It's somewhat similar to the optional `value` parameter that Python supplies in the `.get()` method for `dict`.[^2]
[^2]: https://docs.python.org/3/library/stdtypes.html#dict.get

This convenience actually allows us to elegantly side-step a ternary operation later in the code. To set the right character based on the luminance of the point, the original code uses the following concise line of code:
```c
b[o] = ".,-~:;=!*#$@"[N > 0 ? N : 0];
```
However, because we know that `luminance_index` will be `0` if it was originally negative, the above comparison and inlined default value of `0` are unecessary, allowing for the following:
```rust
buffer[o] = LUMINANCE_CHARS[luminance_index];
```
Since string types in Rust are not accessible by index, the `LUMINANCE_CHARS` here is a `const` array that contains the same 12 characters in the same order.

The second and more difficult of the `usize` conversions is for the `o` variable. This line caused numerous errors while I was testing the code, and I tried multiple different ways of converting it to no avail. As I'm still a newbie to Rust, I eventually turned to A.I. to help me solve the problem (specifically, the *Claude 3 Sonnet* model from [Anthropic](https://www.anthropic.com/)). After some back-and-forth and a couple failed iterations, it produced the following code:
```rust
let o: usize = (x as usize) + ((y as isize).wrapping_mul(80) as usize) % BUFFER_SIZE;
```
While I wish I could say I fully understand why this code works where others failed, I'm not yet enough of a Rustacean to do so. I'm sure there are better ways to do this, and hopefully one day I'll know them and can come back and improve this code. For the time being, however, the above will have to suffice.


#### Semantic Changes

Throughout the code I tried to use more accessible variable names where possible, as I wasn't concerned with slightly increasing the file size. For the most part, I tried to use full words in place of single letters when naming variables. As such, `b` became `buffer` and `z` became `z_buffer`, all the *sin* and *cos* variables went from `sX` and `cX` to `sin_X` and `cos_X` respectively, and `N` became `luminance_index`. This list isn't exhaustive, but I hope it gives you a picture of my thoughts behind the changes.


#### Functional Changes

The only real functional change in this implementation from the original is a change in the time between frames. The original code calls `usleep(15000)` to sleep the thread for 15,000 microseconds (15 milliseconds) and have the animation run at a consistent framerate. I chose to bump this delay up to 35 milliseconds, which causes the program to run closer to 24 frames a second, which I felt looked better. It's a totally subjective choice, so you may find that you disagree with me on what delay looks best.
