#version 450

const int MAX_CUSTOM_LIGHTS = 256;

struct CustomLight {
    vec3 pos;
    float inverse_radius;
    vec4 color;
};

layout(location = 0) in vec3 v_Position;
layout(location = 1) in vec3 v_Normal;
layout(location = 2) in vec2 v_Uv;

layout(location = 0) out vec4 o_Target;

layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};

layout(set = 2, binding = 0) uniform CustomLights {
    vec3 AmbientColor;
    uvec4 NumLights;
    CustomLight SceneLights[MAX_CUSTOM_LIGHTS];
};

layout(set = 3, binding = 0) uniform CustomMaterial_albedo {
    vec4 Albedo;
};
# ifdef CUSTOMMATERIAL_ALBEDO_TEXTURE
layout(set = 3, binding = 1) uniform texture2D CustomMaterial_albedo_texture;
layout(set = 3, binding = 2) uniform sampler CustomMaterial_albedo_texture_sampler;
# endif

void main() {
    vec4 output_color = Albedo;
# ifdef CUSTOMMATERIAL_ALBEDO_TEXTURE
    output_color *= texture(
        sampler2D(StandardMaterial_albedo_texture, StandardMaterial_albedo_texture_sampler),
        v_Uv);
# endif

# ifndef CUSTOMMATERIAL_UNLIT
    vec3 normal = normalize(v_Normal);
    vec3 color = AmbientColor;

    for (int i = 0; i < int(NumLights.x) && i < MAX_CUSTOM_LIGHTS; ++i) {
        CustomLight light = SceneLights[i];

        // Compute attenuation
        vec3 light_vector = (light.pos.xyz - v_Position) * light.inverse_radius;
        float attenuation = clamp(1.0 - dot(light_vector, light_vector), 0.0, 1.0);

        attenuation *= attenuation;

        // Compute Lambertian diffuse term
        vec3 light_dir = normalize(light.pos.xyz - v_Position);
        float diffuse = max(0.0, dot(normal, light_dir));

        // Add light contribution
        color += diffuse * light.color.xyz * attenuation;
    }
    
    output_color.xyz *= color;
# endif

    // Simple tonemapping
    o_Target = vec4(vec3(1.0) - exp(-output_color.rgb), output_color.a);
}