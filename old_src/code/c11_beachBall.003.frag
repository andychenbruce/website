#version 300 es

precision mediump float;

#define PI 3.14159265358979

out vec4 fragColor;

vec3
beachBall(vec3 mpos)
{
  mpos = normalize(mpos);
  float x = mpos.x;
  float z = mpos.z;
  float r = x*x + z*z;
  if (r < 0.03) {
    return vec3(1, 1, 0); // Yellow
  }
  float c2 = x*x/r;
  if (c2 < 0.75) {
    if (x > 0.0) {
      if (z > 0.0) {
	return vec3(0, 1, 0); // Green;
      } else {
	return vec3(0, 0, 1); // Blue
      }
    } else {
      return vec3(1, 1, 1); // White
    }
  } else {
    if (x > 0.0) {
      return vec3(1, 1, 1); // White
    } else {
      return vec3(1, 0, 0); // Red
    }
  }
}

void
main()
{
  vec3 f3 = beachball(mPosition);
  fragColor = vec4(f3.rgb, 1.0);  // Set opacity to 100%
}
