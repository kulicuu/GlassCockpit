#version 300 es

layout(location=0) in vec3 aPosition;
layout(location=1) in vec3 aVelocity;
layout(location=2) in vec3 aColor;

// layout (std140) uniform Mass {
//     float mass1Factor;
//     float mass2Factor;
//     float mass3Factor;
//     vec4 mass1Position;
//     vec4 mass2Position;
//     vec4 mass3Position;
// };

out vec3 vPosition;
out vec3 vVelocity;
out vec3 vColor;
void main() {    
    vec3 position = aPosition;

    vec3 velocity = aVelocity;

    velocity += (position * 0.005);

    vPosition = position + velocity;
    vVelocity = velocity;

    vColor = aColor;
    gl_PointSize = 2.4;
    gl_Position = vec4(position + velocity, 1.0);
}