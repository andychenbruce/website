//
//  encoderDriver.h
//

#pragma once

#define DEVICE_NAME "encoderDriver"

#define ENCODER_MAGIC   0xec0de400
#define ENCODER_VERSION 37

typedef  int32_t  SInt32;
typedef  uint32_t UInt32;
typedef  uint8_t  UInt8;

typedef struct {
  UInt32 magic;         // Sanity check that memory is mapped correctly.
  UInt32 version;	// Ensure the user and kernel are using the same header.
  SInt32 encoderCountA; // Clockwise ticks on motor A.
  SInt32 encoderCountB; // Clockwise ticks on motor B.
} EncoderInfo;

#define EncoderInfoSize sizeof(EncoderInfo)
