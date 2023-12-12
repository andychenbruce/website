vec3
globe(vec3 mpos)
{
  float cf = 1.0;
  mpos = normalize(mpos);
  float x = mpos.x;
  float y = mpos.y;
  float z = mpos.z;
  float a = -(atan2(z, x) / (2.0 * PI));
  float b = 0.5 - (asin(y) / PI);
  return texture(sampler, vec2(a, b)).rgb * cf;
}