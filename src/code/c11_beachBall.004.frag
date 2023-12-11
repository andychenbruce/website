
vec3
beachBallWithSpecular(vec3 mpos)
{
  return shadedSphereWithSpecular(mpos, beachBall(mpos));
}

void
main()
{
  vec3 f3 = beachBallWithSpecular(vec3 mpos)
  fragColor = vec4(f3.rgb, 1.0);  // Set opacity to 100%
}
