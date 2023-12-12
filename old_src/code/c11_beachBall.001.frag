#version 300 es
precision mediump float;
out vec4 fragColor;

uniform int objEnum;
uniform vec3 setColor;
uniform float ambientBrightness;
uniform float diffuseBrightness;
uniform vec3 lightLocation;

vec3
shadedSphere(vec3 mpos, vec3 color)
{
  vec3 v = lightLocation - mpos;
  float cosine = dot(normalize(mpos), normalize(v)); // Cosine
  if (cosine < 0.0) {
    return ambientBrightness * color;
  }
  float cf = clamp(cosine * diffuseBrightness, 0.0, 1.0);
  cf = clamp(cf + ambientBrightness, 0.0, 1.0);  
  return cf * color;
}

void
main()
{
  vec3 f3 = shadedSphere(mPosition, setColor);
  fragColor = vec4(f3.rgb, 1.0);  // Set opacity to 100%
}
