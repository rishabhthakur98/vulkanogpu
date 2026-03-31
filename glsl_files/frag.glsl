#version 450
layout(location = 0) in vec3 frag_pos; // Received from vertex shader
layout(location = 0) out vec4 f_color;

layout(push_constant) uniform PushConstants {
    mat4 mvp;
    vec4 color; // NEW: The Material Color!
} pc;

void main() {
    // FAKE 3D LIGHTING: We mix the base material color with the 3D coordinates.
    // This makes the top/right of the cube brighter than the bottom/left!
    vec3 fake_light = frag_pos + vec3(0.5); 
    f_color = vec4(pc.color.rgb * fake_light, pc.color.a);
}