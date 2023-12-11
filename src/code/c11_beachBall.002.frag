#version 300 es

precision mediump float;

#define PI 3.14159265358979

out vec4 fragColor;

uniform int objEnum;
uniform vec3 setColor;
uniform float ambientBrightness;
uniform float diffuseBrightness;
uniform vec3 lightLocation;

vec3
shadedSphereWithSpecular(vec3 mpos, vec3 color)
{
  vec3 v = lightLocation - mpos;
  float cosine = dot(normalize(mpos), normalize(v));
  float cf = clamp(cosine * diffuseBrightness, 0.0, 1.0);
  vec3 incident = normalize(mpos - cameraLocation);   // Incident vector
  vec3 n = normalize(mpos); // Normal vector to surface of the sphere
  vec3 r = reflect(incident, n);   // Reflected vector
  vec3 realmpos = mpos;
  float cosAngle = dot(normalize(r), normalize(v));
  cf = clamp(cf + ambientBrightness, 0.0, 1.0);
  vec3 lightColor = vec3(1.0, 1.0, 1.0);
  vec3 baseColor = cf * color;
  if (cosAngle < 0.0) {
    return baseColor;
  }
  float shinyness = 5.0;
  float factor = pow(cosAngle, shinyness);
  return ((1.0 - factor) * baseColor) + (factor * lightColor);
}

void
main()
{
  vec3 f3 = shadedSphereWithSpecular(mPosition, setColor);
  fragColor = vec4(f3.rgb, 1.0);  // Set opacity to 100%
}
