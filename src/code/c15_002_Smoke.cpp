//
//  tab29_Smoke.cpp
//

class FireBall {
public:
  float x, y, z;
  float vx, vy, vr;
  float r; // Radius
  
  FireBall() {
    vx = 0;
    vy = 0;
    vr = 0;
    x = 0;
    y = 0;
    z = 0;
    r = 0;
  }
};

static const UInt NumFireBalls = 120;

static struct {
  FireBall fireBalls[NumFireBalls];
  float distanceToCamera[NumFireBalls];
  UInt16 drawingOrder[NumFireBalls];
  UInt fireBallCircularQueueCounter;
} o;


static void
checkDrawingOrder(void)
{
  auto checkOne = [] (UInt n) {
    for (UInt j = 0; j < NumFireBalls; ++j) {
      if (o.drawingOrder[j] == n) {
	return;
      }
    }
    fatal("CDO failed");
  };
  
  for (UInt i = 0; i < NumFireBalls; ++i) {
    checkOne(i);
  }
}

static void
initTab(void)
{
  for (UInt i = 0; i < NumFireBalls; ++i) {
    o.drawingOrder[i] = i;
  }
  checkDrawingOrder();
  g.tp->timerFlag = true;
}

static void
resetTab(void)
{
  g.tp->backgroundColor = 0xd0d0ff;
  g.tp->cameraLocation *= 2.0;
  g.tp->wireFlag = true;
  g.tp->isAnimated = true;
}

static void
restoreTab(void)
{
  glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
}

static int
cmpFireBalls(const void *p1, const void *p2)
{
  // Comparison function used to sort the fireballs.
  // The sorting is done so the balls are drawn back-to-front
  // by their distance from the camera.
  Assert(p1 >= &o.drawingOrder[0]);
  Assert(p2 >= &o.drawingOrder[0]);
  Assert(p1 < &o.drawingOrder[NumFireBalls]);
  Assert(p2 < &o.drawingOrder[NumFireBalls]);
  UInt a = *((UInt16 *) p1);
  UInt b = *((UInt16 *) p2);
  Assert(a < NumFireBalls);
  Assert(b < NumFireBalls);
  float d = o.distanceToCamera[a] - o.distanceToCamera[b];
  return signum(d);
}

static void
paintTab(void)
{
  mat4 mm = mat4(1);
  mm = glm::translate(mm, vec3(0.0, -2.0, 0.0));
  mm = glm::scale(mm, vec3(2.0, 2.0, 2.0));
  mm = glm::rotate(mm, 0.01f, vec3(1.0, 0.0, 0.0));
  mm = glm::rotate(mm, 0.2f, vec3(0.0, 1.0, 0.0));
  auto tick = [] () {
    // Move the fireballs on each time tick.
    static const float DVX = 0.00015;
    static const float DY  = 0.03;
    static const float DVR = 0.00005;
    static const float DR  = 0.004;
    for (UInt i = 0; i < NumFireBalls; ++i) {
      FireBall *fb = &o.fireBalls[i];
      fb->vx += DVX;
      // fb->vy += DVY;
      fb->vr += DVR;
      fb->x += fb->vx;
      fb->y += DY;
      //fb->r += fb->vr * (1.0 + randomFloat());
      fb->r += (fb->vr + DR) * (0.5 + 1.5 * randomFloat());
    }
  };
  auto addFireBall = [] () {
    // Add a fireball at the base of the flame.
    o.fireBalls[o.fireBallCircularQueueCounter] = FireBall();
    ++o.fireBallCircularQueueCounter;
    o.fireBallCircularQueueCounter %= NumFireBalls;
  };
  auto sortFireBalls = [mm] () {
    mat4 cm = g.tp->getCombinedMatrix(mm);  // Does this need to be inverted???
    for (UInt i = 0; i < NumFireBalls; ++i) {
      FireBall *fb = &o.fireBalls[i];
      vec4 pos4 = cm * vec4(fb->x, fb->y, fb->z, 1.0);
      o.distanceToCamera[i] = pos4.z;
    }
    qsort(&o.drawingOrder[0], NumFireBalls, sizeof(UInt16), cmpFireBalls);
  };
  auto drawFireBalls = [mm] () {
    for (UInt i = 0; i < NumFireBalls; ++i) {
      FireBall *fb = &o.fireBalls[o.drawingOrder[i]];
      mat4 m = glm::translate(mm, vec3(fb->x, fb->y, fb->z));
      mat4 cm = g.tp->getCombinedMatrix(m);
      setFireBallRadius(fb->r);
      drawBillboard(cm, fb->r);
    }
  };
  g.tp->setModelMatrix(mm);
  setCullFace(false);
  setDepthTest(false);
  drawTileFloor(0xff0000);
  setDepthTest(false);
  startBlending();
  addFireBall();
  tick();
  sortFireBalls();
  drawFireBalls();
  stopBlending();
}
