#version 450 core

layout (location = 0) in uint diffuse_index;
layout (location = 1) in vec3 position;
layout (location = 2) in vec3 normal;
layout (location = 3) in vec4 color;
layout (location = 4) in vec2 uv;

uniform mat4 view_projection;
uniform vec3 model;

out VS_OUTPUT {
    vec4 Color;
} OUT;

void main()
{
    gl_Position = view_projection * vec4(position, 1.0);
    OUT.Color = color / 255.0f;
}