#version 300 es

precision highp float;

uniform uint fragEnum;
uint ENUM_RED = 0u;
uint ENUM_GREEN = 1u;
uint ENUM_BLUE = 2u;
uint ENUM_BLACK = 3u;
uint ENUM_WHITE = 4u;
uint ENUM_CLEAR_RED = 5u;
uint ENUM_CLEAR_GREEN = 6u;
uint ENUM_CLEAR_BLUE = 7u;
uint ENUM_REFLECTIVE_RED = 8u;
uint ENUM_SPECTRAL_RED = 9u;
uint ENUM_BEACHBALL = 10u;

uniform vec3 lightPos;
uniform mat4 cameraMatrix;

in vec4 inputPos;
out vec4 outColor;

float AMBIENT_BRIGHTNESS = 0.4;
float DIFFUSE_BRIGHTNESS = 0.6;

vec3 getBeachBallColor(){
  vec3 mpos = normalize(vec3(inputPos));
  float x = mpos.x;
  float y = mpos.y;
  float r = x*x + y*y;
  if (r < 0.03) {
    return vec3(1, 1, 0);
  }
  float c2 = x*x/r;
  if (c2 < 0.75) {
    if (x > 0.0) {
      if (y > 0.0) {
	return vec3(0, 1, 0);
      } else {
	return vec3(0, 0, 1);
      }
    } else {
      return vec3(1, 1, 1);
    }
  } else {
    if (x > 0.0) {
      return vec3(1, 1, 1);
    } else {
      return vec3(1, 0, 0);
    }
  }
  return vec3(1.0, 0.0, 0.0);
}

float diffuse_reflect_factor(){
  vec3 dir = vec3(lightPos) - vec3(inputPos);
  float cosine = dot(normalize(dir), normalize(vec3(inputPos)));
  float cf = clamp((AMBIENT_BRIGHTNESS * cosine) + DIFFUSE_BRIGHTNESS, 0.0, 1.0);
  return cf;
}

float specular_reflect_factor(){
  vec3 camera_pos = vec3(inverse(cameraMatrix) * vec4(0.0, 0.0, 0.0, 1.0));
  vec3 incident = -(camera_pos - vec3(inputPos));
  vec3 optimal_dir = normalize(reflect(incident, normalize(vec3(inputPos))));
  vec3 light_dir = normalize(lightPos - vec3(inputPos));
  float cos = dot(optimal_dir, light_dir);
  float shinyness = 5.0;
  float factor = pow(clamp(cos, 0.0, 1.0), shinyness);
  return factor;
}

void main() {
  if(fragEnum == ENUM_RED){
    outColor = vec4(1.0, 0.0, 0.0, 1.0);
  }else if(fragEnum == ENUM_GREEN){
    outColor = vec4(0.0, 1.0, 0.0, 1.0);
  }else if(fragEnum == ENUM_BLUE){
    outColor = vec4(0.0, 0.0, 1.0, 1.0);
  }else if(fragEnum == ENUM_BLACK){
    outColor = vec4(0.0, 0.0, 0.0, 1.0);
  }else if(fragEnum == ENUM_WHITE){
    outColor = vec4(1.0, 1.0, 1.0, 1.0);
  }else if(fragEnum == ENUM_CLEAR_RED){
    outColor = vec4(0.5, 0.0, 0.0, 0.5);
  }else if(fragEnum == ENUM_CLEAR_GREEN){
    outColor = vec4(0.0, 0.5, 0.0, 0.5);
  }else if(fragEnum == ENUM_CLEAR_BLUE){
    outColor = vec4(0.0, 0.0, 0.5, 0.5);
  }else if(fragEnum == ENUM_REFLECTIVE_RED){
    outColor = vec4(diffuse_reflect_factor() * vec3(1.0, 0.0, 0.0), 1.0);
  }else if(fragEnum == ENUM_SPECTRAL_RED){
    vec4 regular_color = vec4(diffuse_reflect_factor() * vec3(1.0, 0.0, 0.0), 1.0);
    vec4 light_color = vec4(1.0, 1.0, 1.0, 1.0);
    float specular_factor = specular_reflect_factor();
    outColor = (specular_factor * light_color) + ((1.0-specular_factor) * regular_color);
  }else if(fragEnum == ENUM_BEACHBALL){
    vec4 regular_color = vec4(diffuse_reflect_factor() * getBeachBallColor(), 1.0);
    vec4 light_color = vec4(1.0, 1.0, 1.0, 1.0);
    float specular_factor = specular_reflect_factor();
    outColor = (specular_factor * light_color) + ((1.0-specular_factor) * regular_color);
  }else{
    outColor = inputPos;
  }
}
