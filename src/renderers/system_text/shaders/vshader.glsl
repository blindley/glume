#version 330 core
layout (location = 0) in vec2 inPos;

uniform vec2 windowSize;

void main()
{
    vec2 pos = vec2(160.0, 200.0) * inPos / windowSize + vec2(-0.99, 0.99);
    gl_Position = vec4(pos, 0.0, 1.0);
}
