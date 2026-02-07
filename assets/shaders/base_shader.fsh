#version 450 core
out vec4 FragColor;

in vec2 texCoords;

uniform sampler2D albedoTexture;

void main()
{
    vec3 lightColor = vec3(1.0, 1.0, 1.0);
    float ambientStrength = 1.0;
    vec3 ambient = ambientStrength * lightColor;

    vec4 objectColor = texture(albedoTexture, texCoords);
    vec3 result = ambient * objectColor.rgb;

    FragColor = vec4(result, objectColor.a);
}
