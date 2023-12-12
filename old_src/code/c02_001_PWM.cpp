static UInt32
getModelInfo(void)
{
  static const char fn[] = "/sys/firmware/devicetree/base/model";
  char modelString[64];
  readFileToBuffer(fn, modelString, sizeof(modelString));
  if (isPrefix("Raspberry Pi 3 ", modelString)) {
    return 3;
  }
  if (isPrefix("Raspberry Pi 4 ", modelString)) {
    return 4;
  }
  fatal("Unsupported RPi model: %s", modelString);
}
