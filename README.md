# Rust version of [_Ray Tracer in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html)

Not much to say, I (slowly) followed the tutorial and got the expected result.

I used Rayon to parallelize each scanline, but even with that, rendering the final scene took a really long time (Ryzen 9 5900HX 8c/16t), maybe I'll find a way to optimize that (get rid of Arc?).

I'm not sure if the pattern has a name, but I replaced the "pass-by-reference plus boolean return for error" with `Option<T>`, which I find much more pleasant and clear to work with.
