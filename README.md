# Ray Tracer
This is a ray tracer based on [Peter Shirley's ray tracing guide in C++.](https://raytracing.github.io/books/RayTracingInOneWeekend.html) I wanted to complete a personal project in rust and thought this would be a great starting point since I could use my knowledge of C++ as well. This ray tracer can create scenes with spheres and cubes of various materials like metal and glass. Running this program will create an image file named by `output.ppm` of the raytraced scene, but you can pass a different name as a command line argument if you prefer. Please email me at 3069391@gmail.com or comment on this project page should you have any questions about the ray tracer, suggestions for further improvement, or have found any bugs.

## Screenshots


## Setup Instructions
1. First, download the source code, either by executing a `git clone https://github.com/x2dtu/ray-tracer.git` in a terminal or downloading the project as a zip through the Github page and extracting that zip.
2. This project uses rust. Make sure you have rust installed so you can compile and run an executable for your system. You can find rust installation instructions [here](https://www.rust-lang.org/tools/install).
3. To run this program, in the terminal at the root of the project, execute a `cargo run` to compile and run the program. If you want to name the output picture file, then do a `cargo run <filname>.ppm`, making sure that the picture file is of .ppm type.

## How to create your own scenes
There are numerous ways to edit the program to your liking. By default, it will create a high sample scene of randomly spawned spheres and cubes of random materials. The higher the samples per pixel constant, the more refined the image will be but the longer the program will run for. The lower it is, the more granular the picture will turn out. The randomness paramters in the random_scene() function can be updated to spawn more spheres than cubes or whatever combination you might want. Another way of editing the final picture is through the camera, where you can change various parameters like it aperture diameter, focus distance, where it looks at, where its positioned, and others. A possible improvement for this project that I have thought of is to create a GUI where users can edit these parameters without having to manually edit the source code.
