<?php
require('library.php');
do_top(4, 'MotorPWM');
?>

<hr class="clearLeft" />
  <?php imgCap('L298-H-Bridge.jpg', 2/3, 'L298 H-Bridge'); ?>
  <p>To control the motors, I used the same PWM circuit described on <a href="p02_PWM.php">this webpage</a>, but the outputs
  are connected to an <a href="https://en.wikipedia.org/wiki/H-bridge">H-Bridge</a> to control a stronger current.  Motors need much more power than LEDs.</p>
  <p>For the H-Bridge, I used an L298 Dual H-Bridge.  I bought <a href="https://www.amazon.com/gp/product/B07WS89781/">a pack of four on Amazon</a> for $11.</p>
  <?php imgCap('L298-Labels.jpg', 2/3, 'L298 with I/O Labels'); ?>
  
<hr class="clearLeft" />

  <?php imgCap('L298-Schematic.png', 2/3, 'L298 to RPi interface'); ?>
  <p>This is the wiring diagram for connecting the Raspberry Pi header to the H-Bridge.  I made the connections with
  <a href="https://en.wikipedia.org/wiki/Wire_wrap">wire wrap</a>.</p>

<hr class="clearLeft" />


  <?php imgCap('Buck-Converter.jpg', 1/2, '12v to 5v Buck Converter'); ?>

  <p>
  To convert the 12 volt power from the battery to the 5 volts
  needed by the Raspberry Pi and the encoder logic circuit, I
  used a 12v-to-5v <a href="https://en.wikipedia.org/wiki/Buck_converter">buck converter</a>.
  I bought a four pack of these buck converters
  <a href="https://www.amazon.com/gp/product/B087RHWTJW/">
  on Amazon for $12
  </a>.
  The 5 volt output is emitted on a USB port, which is very
  convenient for attaching to the Raspberry Pi.
  All I needed was a USB-A to USB-C cable.
  </p>

  <hr class="clearLeft" />

  <?php videoCap('wiringControlBoard-720.mp4',
		 'Wiring the control board.' .
		 ' My arms were sunburned ' .
		 ' when I made this video.'); ?>
  <p>
  This is a video of the assembly and wiring of the robot control board. The Raspberry Pi is the green board in the lower right.  The L298 H-Bridges are at the top of the board.  The metal angle holds a small USB fan used to keep the Raspberry Pi cool during intensive computer vision processing.
  </p>
  
  <hr class="clearLeft" />

  <?php videoCap('motorPwm-480.mp4', 640, 360,
		 'Controlling the motors with a cell phone.'); ?>

  <p>This video shows testing of the motors using Wifi between the Raspberry Pi and a cell phone.  The and speed and direction of the motors can be independently controlled.  The next step is to mount the motors on the robot to drive the wheels.  The independent control allows differential steering.
  </p>
  <hr class="clearLeft" />

  
  <?php imgCap('burnt-wires.jpg', 1, 'Burnt wires from an accidental short.'); ?>
  <p>Just to show that it isn't all just fun and games, here are some burnt alligator clips. I accidently caused a short sending the full current of the 12 volt battery through the wires, causing a loud pop and plenty of smoke.</p>
  
  <hr class="clearLeft" />
<?php do_footer(); ?>
