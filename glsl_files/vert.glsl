#version 450
layout(location = 0) in vec3 position;

// This receives our camera matrix from Rust
layout(push_constant) uniform PushConstants {
    mat4 mvp;
} pc;

void main() {
    // Multiply the matrix by the position to simulate 3D camera movement
    gl_Position = pc.mvp * vec4(position, 1.0);
}