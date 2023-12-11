//
//  rPiModel.c
//

static UInt
rPiModelNumber(void)
{
  static struct {
    UInt modelNum;
    const char *modelPrefix;
  } modelTab[] = {
   { RPi_Z, "Raspberry Pi Zero " },
   { RPi_3, "Raspberry Pi 3 " },
   { RPi_4, "Raspberry Pi 4 " },
  };
  static const char fn[] = "/sys/firmware/devicetree/base/model";
  char modelString[64];
  readFileToBuffer(fn, modelString, sizeof(modelString));
  for (UInt i = 0; i < NumItems(modelTab); ++i) {
    if (isPrefix(modelTab[i].modelPrefix, modelString)) {
      return modelTab[i].modelNum;
    }
  }
  fatal("Unsupported RPi model: %s", modelString);
}
