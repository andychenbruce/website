static void
drawBillboard(mat4 cm, float sz)
{
  auto removeTranslation = [] (float *m) {
    m[3]  = 0.0;
    m[7]  = 0.0;
    m[11] = 0.0;
    m[12] = 0.0;
    m[13] = 0.0;
    m[14] = 0.0;
    m[15] = 1.0;
  };
  auto removeScaling = [] (float *m) {
    float sx = sqrt(m[0]*m[0] + m[4]*m[4] + m[8]*m[8]);   // Scale-X
    float sy = sqrt(m[1]*m[1] + m[5]*m[5] + m[9]*m[9]);   // Scale-Y
    float sz = sqrt(m[2]*m[2] + m[6]*m[6] + m[10]*m[10]); // Scale-Z
    m[0] /= sx;  // Reverse the scaling
    m[1] /= sy;
    m[2] /= sz;
    m[4] /= sx;
    m[5] /= sy;
    m[6] /= sz;
    m[8] /= sx;
    m[9] /= sy;
    m[10] /= sz;
  };
  
  mat4 rm = cm;
  float *p = glm::value_ptr(rm);
  removeTranslation(p);
  removeScaling(p);
  // Reverse the rotation while keeping the original scaling and translations.
  mat4 xm = cm * glm::inverse(rm);
  xm = glm::scale(xm, vec3(sz, sz, sz));
  setCombinedMatrix(glm::value_ptr(xm));
  drawCircle32(OE_FireBall);
}
