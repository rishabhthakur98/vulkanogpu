#version 450
layout(location = 0) in vec3 position;

// Pass the position to the fragment shader for fake lighting
layout(location = 0) out vec3 frag_pos; 

layout(push_constant) uniform PushConstants {
    mat4 mvp;
    vec4 color; // NEW: The Material Color!
} pc;

void main() {
    frag_pos = position;
    gl_Position = pc.mvp * vec4(position, 1.0);
}