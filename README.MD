# A real-time ray tracer written in Rust

![](https://github.com/Ancientkingg/rust-raytracer/blob/master/picture.png?raw=true)

I have written this project as practice to learn Rust with the aim to know the language a lot better and become a lot more comfortable with it. The initial influence to write this little raytracer was when I came across [this guide]()https://raytracing.github.io/books/RayTracingInOneWeekend.html) written by Peter Shirley. I decided to follow it through to see if I could actually write a raytracer, since I always found graphics quite interesting to work with, but had never actually written an actual software raytracer before.

Nonetheless, I started following the guide step by step, whilst reinterpreting the code into Rust. Since the guide was originally intended for C++ with its classes and inheritance, I spent most of my time trying to understand the guide and figuring out how to write this C++ code into 'rustacean' etiquette (using mostly traits). During this process of as it were conversion, I stumbled across [another guide](https://misterdanb.github.io/raytracinginrust/) that was written by Daniel Busch. It seemed like this guide was an exact copy of Peter Shirley's, with one major difference. It was aimed at and used the Rust language. This guide helped me a lot when I was struggling to convert C++'s inheritance into terms and syntax that was applicable for Rust.



After finishing the guide I decided to take it one step further by making the render loop sort of async, adding an fps counter and movement and rotation controls, which means when you launch this raytracer you can use WASD, LCTRL, the spacebar and your mouse to move around in the scene. Secondly, I wanted to make a nice screenshot for this README so decided to add another control (P) that disabled controls, removed the FPS counter and enabled my janky version of temporal blending (basically overlaying frames on top of each other).

To run the raytracer either compile it in release mode and then launch the executable

`cargo build --release`

or run it directly

`cargo run --release`



In the code there are some constants which control the resolution and the fps the raytracer tries to maintain, which you can change the change the result.

## What I used

Obviously you could look in the cargo.toml to see what dependencies I used, but to summarize the most important crates in this project were most likely `piston_window`, `image` and `nalgebra_glm`. I used `piston__window` for the actual window spawning and key listening (for the controls); I used `image` for the image buffer and `nalgebra_glm` for all the linear algebra.


