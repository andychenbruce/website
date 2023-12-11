//
//  Fragment shader subroutine for drawing a flame
//

vec4
fireball(vec3 mpos)
{
  float r = fireBallRadius;
  float opacity = 0.0;
  vec4 color;
  r *= 3.0;
  if (r < 0.5) {
    color.r = 1.0;
    color.g = 1.0;
    color.b = (1.0 - 2.0*r);
    opacity = 1.0 - r;
  } else if (r < 1.0) {
    color.r = 1.0;
    color.g = (2.0 - 2.0*r);
    color.b = 0.0;
    opacity = 0.1 + 0.4 * (2.0 - 2.0*r);
  } else if (r < 2.0) {
    color.r = (1.0 - r/2.0);
    color.g = 0.0;
    color.b = 0.0;
    opacity = 0.01 + 0.09 * (2.0 - r);
  } else {
    color.r = 0.0;
    color.g = 0.0;
    color.b = 0.0;
    if (r < 3.0) {
      opacity = 0.01 * (2.0 - r);
    } else {
      opacity = 0.0;
    }
  }
  color.a = opacity * (1.0 - length(mpos));
  return color;
}
