#version 450 core

layout (location = 0) in uint diffuse_index;
layout (location = 1) in vec3 position;
layout (location = 2) in uvec4 color;
layout (location = 3) in vec2 uv;

out VS_OUTPUT {
    vec3 Color;
} OUT;

void main()
{
    gl_Position = vec4(position, 1.0);
    OUT.Color = vec3(color[0] / 255f, color[1] / 255f, color[2] / 255f);
}