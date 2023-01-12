#version 450 core
in VS_OUTPUT {
    vec4 Color;
} IN;

out vec4 Color;

void main()
{
    Color = IN.Color;
}
