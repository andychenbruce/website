#version 410 core
//#version 300 es

precision mediump float;

#define PI 3.14159265358979

const int OE_ShinySphere = 24;
const int OE_TileFloor = 25;
const int OE_MonaLisa    = 51;
const int OE_EarthGlobe  = 52;
const int OE_JupiterGlobe= 53;
const int OE_AlanTuring  = 54;
const int OE_VonNeumann  = 55;
const int OE_DaVinci     = 56;

out vec4 fragColor;

uniform int objEnum;
uniform vec3 setColor;

uniform sampler2D sampler2; // MonaLisa
uniform sampler2D sampler3; // EarthGlobe
uniform sampler2D sampler4; // JupiterGlobe
uniform sampler2D sampler5; // AlanTuring
uniform sampler2D sampler6; // VonNeumann
uniform sampler2D sampler7; // DaVinci

float
distanceFromLineToPoint(vec3 v1, vec3 v2, vec3 p)
{
  return length(cross((p - v1), (p - v2))) / distance(v1, v2);	
}

float
distanceFromRayToPoint(vec3 startingPoint, vec3 directionUnitVec, vec3 p)
{
  vec3 endPoint = startingPoint + directionUnitVec;
  vec3 d1 = p - startingPoint;
  vec3 d2 = p - endPoint;
  if (length(d2) > length(d1)) {
    return length(d1);
  }
  return length(cross(d1, d2));
}


vec3
tileFloorColor(vec3 mpos)
{
  int x = int(2.5 * (2.0 + mpos.x));
  int z = int(2.5 * (2.0 + mpos.z));
  vec3 color;
  if ((x & 1) == (z & 1)) {
    color = setColor;
  } else {
    color = vec3(1.0, 1.0, 1.0); // White
  }
  return color;
}

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

vec3
doTexture(vec3 mpos, sampler2D sampler)
{
  float x = 0.5 * (1.0 - mpos.x);
  float y = 0.5 * (1.0 - mpos.y);
  return texture(sampler, vec2(x, y)).rgb;
}

vec3
monaLisa(vec3 mpos)
{
  float x = 0.5 * (1.0 - mpos.x);
  float y = 0.5 * (1.0 - mpos.y);
  return texture(sampler2, vec2(x, y)).rgb;
}

float
atan2(float y, float x)
{
  float ax = abs(x);
  float ay = abs(y);
  float t = (ax >= ay) ? atan(ay/ax) : (PI/2.0 - atan(ax/ay));
  return (y > 0.0) ?
    ((x > 0.0) ? t : (PI - t)) :
    ((x > 0.0) ? (2.0*PI - t) : (PI + t));
}

vec3
shinySphere(vec3 mpos)
{
  vec3 lightPos = lightLocation;
  //mpos = normalize(mpos);
  vec3 fakeCameraLocation = cameraLocation;
  vec3 v = normalize(mpos - fakeCameraLocation);   // Incident vector
  vec3 n = normalize(mpos); // Normal vector to surface of the sphere
  vec3 r = reflect(v, n);   // Reflected vector
  vec3 realmpos = mpos;
  float multiplyRByThis = ((-1.0)-realmpos.y)/r.y;
  if (multiplyRByThis > 0.0) {//not phantom reflection
    vec3 t =  multiplyRByThis*r;
    t = t + realmpos;
    if ((abs(t.x) < 2.0) && (abs(t.z) < 2.0)) {
      return tileFloorColor(t);
    }
  }
  if (length(mpos - lightPos) < length(lightPos)) {
    float d = distanceFromRayToPoint(mpos, r, lightPos);
    if (d < LightBulbRadius) {
      return vec3(1, 1, 1); // Lightbulb reflection
    }
  }
  //vec3 textureRotateVector = vec3(0, 1, 0);
  //float textureRotateRadians = 0.0;
  vec3 textureTranslate;
  float textureWH;
  float textureSC;
  bool isRotated;
  textureTranslate = vec3(1.2, 1.0, 2.0);
  textureWH = 0.671141;
  textureSC = 1.1;
  //isRotated = false;
  multiplyRByThis = ((textureTranslate.z)-realmpos.z)/r.z;
  if(multiplyRByThis > 0.0){//not phantom reflection
    vec3 t = r*multiplyRByThis;
    t *= vec3(1.0/(textureSC*textureWH), 1.0/textureSC, 1.0);
    t += -1.0*textureTranslate;
    if (((t.x) > -1.0) && ((t.x) < 1.0) && ((t.y) > -1.0) && ((t.y) < 1.0)) {
      return(doTexture(t, sampler2));//mona lisa
    }
  }

  textureTranslate = vec3(-1.2, 1.0, 2.0);
  textureWH = 0.714167;
  textureSC = 1.0;
  //isRotated = false;
  multiplyRByThis = ((textureTranslate.z)-realmpos.z)/r.z;
  if(multiplyRByThis > 0.0){//not phantom reflection
    vec3 t = r*multiplyRByThis;
    t *= vec3(1.0/(textureSC*textureWH), 1.0/textureSC, 1.0);
    t += -1.0*textureTranslate;
    if (((t.x) > -1.0) && ((t.x) < 1.0) && ((t.y) > -1.0) && ((t.y) < 1.0)) {
      return(doTexture(t, sampler7));//davinci
    }
  }

  
  textureTranslate = vec3(-1, 1.0, 2.0);
  textureWH = 1.000000;
  textureSC = 1.0;
  isRotated = true;
  if(isRotated){
    multiplyRByThis = ((textureTranslate.z)-realmpos.x)/r.x;
  }else{
    multiplyRByThis = ((textureTranslate.z)-realmpos.z)/r.z;
  }
  if(multiplyRByThis > 0.0){//not phantom reflection
    vec3 t = r*multiplyRByThis;
    if(isRotated){//undo rotation????
      float tmpX = t.x;
      t.x = -t.z;
      t.z = tmpX;
    }
    t *= vec3(1.0/(textureSC*textureWH), 1.0/textureSC, 1.0);
    t += -1.0*textureTranslate;
    if (((t.x) > -1.0) && ((t.x) < 1.0) && ((t.y) > -1.0) && ((t.y) < 1.0)) {
      return(doTextureGray(t, sampler5));//turing
    }
  }

  textureTranslate = vec3(1.0, 2.0, 2.0);
  textureWH = 0.760845;
  textureSC = 1.0;
  isRotated = true;
  if(isRotated){
    multiplyRByThis = ((textureTranslate.z)-realmpos.x)/r.x;
  }else{
    multiplyRByThis = ((textureTranslate.z)-realmpos.z)/r.z;
  }
  if(multiplyRByThis > 0.0){//not phantom reflection
    vec3 t = r*multiplyRByThis;
    if(isRotated){//undo rotation????
      float tmpX = t.x;
      t.x = -t.z;
      t.z = tmpX;
    }
    t *= vec3(1.0/(textureSC*textureWH), 1.0/textureSC, 1.0);
    t += -1.0*textureTranslate;
    if (((t.x) > -1.0) && ((t.x) < 1.0) && ((t.y) > -1.0) && ((t.y) < 1.0)) {
      return(doTextureGray(t, sampler6));//von new man
    }
  }
  return vec3(0, 1, 1); // Cyan
}

vec3
f3main()
{
  switch (objEnum) {
  case OE_SetColor:    return setColor;
  case OE_TileFloor:   return tileFloorColor(mPosition);
  case OE_ShinySphere: return shinySphere(mPosition);
  case OE_MonaLisa:    return doTexture(mPosition, sampler2);
  case OE_AlanTuring:  return doTextureGray(mPosition, sampler5);
  case OE_VonNeumann:  return doTexture(mPosition, sampler6);
  case OE_DaVinci:     return doTexture(mPosition, sampler7);
  default: return vec3(0, 1, 0); // Green
  }
}

void
main()
{
  vec3 f3 = f3main();
  fragColor = vec4(f3.rgb, 1);
}
