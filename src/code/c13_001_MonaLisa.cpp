static void
initTab(void)
{
  checkGLError();
  loadTexture(TextureMonaLisa);
  loadTexture(TextureAlanTuring);
  loadTexture(TextureVonNeumann);
  loadTexture(TextureDaVinci);
}

static void
resetTab(void)
{
  g.tp->timerFlag = false;
  g.tp->lightsFlag = false;
  g.tp->isAnimated = true;
  g.tp->backgroundColor = 0xd0d0ff;
  g.tp->cameraLocation *= 2.0;
  g.tp->wireFlag = false;
  g.tp->fragmentsFlag = true;
}

static void
restoreTab(void)
{
  //glDisable(GL_BLEND);
}

static void
monaLisa(void)
{
  glm::mat4 mm = mat4(1);
  mm = glm::translate(mm, vec3(1.2, 1.0, 2.0));
  int tx = TextureMonaLisa;
  float wh = ((float) textureTab[tx].width) / textureTab[tx].height;
  const float sc = 1.1;
  mm = glm::scale(mm, vec3(sc * wh, sc, 1.0));
  g.tp->setModelMatrix(mm);
  drawWall(OE_MonaLisa);
}

static void
daVinci(void)
{
  glm::mat4 mm = mat4(1);
  mm = glm::translate(mm, vec3(-1.2, 1.0, 2.0));
  int tx = TextureDaVinci;
  float wh = ((float) textureTab[tx].width) / textureTab[tx].height;
  const float sc = 1.0;
  mm = glm::scale(mm, vec3(sc * wh, sc, 1.0));
  g.tp->setModelMatrix(mm);
  drawWall(OE_DaVinci);
}

static void
alanTuring(void)
{
  glm::mat4 mm = mat4(1);
  mm = glm::rotate(mm, (float) (M_PI/2.0), vec3(0, 1, 0));
  mm = glm::translate(mm, vec3(-1, 1.0, 2.0));
  int tx = TextureAlanTuring;
  float wh = ((float) textureTab[tx].width) / textureTab[tx].height;
  mm = glm::scale(mm, vec3(1.0 * wh, 1.0, 1.0));
  g.tp->setModelMatrix(mm);
  drawWall(OE_AlanTuring);
}

static void
vonNeumann(void)
{
  glm::mat4 mm = mat4(1);
  mm = glm::rotate(mm, (float) (M_PI/2.0), vec3(0, 1, 0));
  mm = glm::translate(mm, vec3(1.0, 2.0, 2.0));
  int tx = TextureVonNeumann;
  float wh = ((float) textureTab[tx].width) / textureTab[tx].height;
  // epf("wh = %f", wh);
  mm = glm::scale(mm, vec3(1.0 * wh, 1.0, 1.0));
  g.tp->setModelMatrix(mm);
  drawWall(OE_VonNeumann);
}

static void
doSphere(float x, float y, float z, GLint oe)
{
  mat4 mm = mat4(1);
  mm = glm::translate(mm, vec3(x, y, z));
  setCameraLocation(g.tp->cameraLocation);
  g.tp->setModelMatrix(mm);
  drawSphere(3, oe);
}

static void
paintTab(void)
{
  setDepthTest(true);
  mat4 mm = glm::mat4(1);
  mm = glm::mat4(1);
  mm = glm::translate(mm, vec3(0.0, -2.0, 0.0));
  mm = glm::scale(mm, vec3(1.0, 1.0, 1.0));
  g.tp->setModelMatrix(mm);
  setCullFace(false);
  drawTileFloor(0x9070a0);
  alanTuring();
  vonNeumann();
  daVinci();
  monaLisa();
  doSphere(0, 0, 0, OE_ShinySphere);
}
