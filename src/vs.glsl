#version 450 core

layout (location = 0) in uint diffuse_index;
layout (location = 1) in vec3 position;
layout (location = 2) in vec4 color;
layout (location = 3) in vec2 uv;

out VS_OUTPUT {
    vec4 Color;
} OUT;

void main()
{
    gl_Position = vec4(position, 1.0);
    OUT.Color = vec4(float(color[0]) / 255.0f, float(color[1]) / 255.0f, float(color[2]) / 255.0f, float(color[3]) / 255.0f);
}