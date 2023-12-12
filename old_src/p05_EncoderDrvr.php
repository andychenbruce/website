<?php
require('library.php');
do_top(5, 'EncoderDrvr');
?>

<?php imgCap('i05-Uxcell-Motor.jpg', 1.0,
	     'Uxcell gear motor with encoder'); ?>


<p>A <a href="https://en.wikipedia.org/wiki/Rotary_encoder">rotary encoder</a> is an electro-mechanical device that senses rotation of a motor shaft and converts it into a digital signal that can be processed by a computer.
</p>

  <?php imgCap('i05-Encoder-Wires.jpg', 1.0,
	       'Encoder on the end of the motor shaft'); ?>

<p>
  For my robot project, I used two Uxcell gear motors with
  <a href="https://en.wikipedia.org/wiki/Hall_effect_sensor">Hall effect encoders</a>.
  I bought the
  <a href="https://www.amazon.com/gp/product/B0792RX5X1/">motors from Amazon</a>.
  The encoders have 16 magnets and two hall effect sensors.  The magnets produce a rising
  and falling signal as they pass each sensor.  So there are 64 encoder interrupts per
  revolution of the motor.  The motors have a 30:1 gear ratio, so there are
  64*30 = 1920 pulses for each revolution of the gearbox shaft.
  That is plenty of resolution to navigate the robot.
  </p>

<hr class="clearLeft" />
  
  <?php imgCap('i05-Incremental_directional_encoder.gif', 1.0,
	       'Incremental directional encoder'); ?>

  <?php imgCap('i05-EncoderSignals.png', 0.5, 'Encoder signals'); ?>

  <?php imgCap('i05-EncoderFwdBwd.jpg', 0.5,
	       'Forward and backwards rotations'); ?>

<p>The sequence of interrupts tells us both the speed of the motor and the direction it is turning.
The rotary position (Position), the rate it is changing (Derivative), and the accumulated error (Integral) can be used in a Position-Derivative-Integral <a href="https://en.wikipedia.org/wiki/PID_controller">PID feedback control loop</a>.  The feedback is used to control the PWM signal to the motors.</p>
  
<hr class="clearLeft" />
  <p>
  Reading the encoder interrupts is done in the Linux kernel on the
  Raspberry Pi. The device driver receives an interrupt for each
  encoder tick and increments or decrements the count in a data
  structure. The data is mapped to the address of an application
  using mmap().</p>
    <p>The source code for the device driver is listed below.</p>
  <?php code('c05_encoder/encoderDriver.c'); ?>
<hr class="clearLeft" />
  
<hr class="clearLeft" />
  <p>This is the user program for reading the encoder counts.</p>
  <?php code('c05_encoder/encoderTestMmap.c'); ?>
<hr class="clearLeft" />
  <p>This is the Makefile for compiling the source code listed above.</p>
  <?php code('c05_encoder/Makefile'); ?>
<hr class="clearLeft" />
  <p>This is a Python script for the GUI to display the encoder
  counts in the video.</p>
  <?php code('c05_encoder/encoderPy.py'); ?>
<hr class="clearLeft" />
  
<?php videoCap('Encoder-480.mp4', 'Testing the encoder driver.'); ?>
  
<hr class="clearLeft" />
  
<?php do_footer(); ?>
