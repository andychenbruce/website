<?php
require('library.php');
do_top(6, 'GCode');
?>

<?php imgCap('Delta-X.jpg', 1.0, 'Delta-X Robot'); ?>

<p><a href="https://en.wikipedia.org/wiki/Delta_robot">Delta robots</a> are often used for
  factory "pick and place" operations.</p>
  <p>Delta robot arms are not as precise as some other robot arm geometries,
  but they can move fast and are easy to program.  The millimeter-level accuracy
  is good enough for my project to move an end effector over weeds or crop plants.</p>
  
<p>I bought the delta robot arm from <a href="https://deltaxrobot.com">Delta-X</a>,
  a Vietnamese company.  They shipped the robot from Da Nang, Vietnam to Hawaii.
  When it arrived, I connect it to the bottom of the robot frame.
</p>  

<hr class="clearLeft" />
  
  <?php imgCap('DeltaAnimation.gif', 1.0,
  'Delta robot kinematics, © Wikimedia'); ?>
<p>Delta robot kinematics (green arms are fixed length, at 90° to the blue axis that they rotate about)</p>
<hr class="clearLeft" />
  <?php videoCap('Delta-G-Code-720.mp4',
		 "Testing the delta robot arm with G-code"); ?>
  <p>The delta robot arm is programmed with
  <a href="https://en.wikipedia.org/wiki/G-code">G-code</a>,
  a simple language used for the control of many machine tools.
  <p>
<hr class="clearLeft" />
  <p>I wrote a program to run on the Raspberry Pi computer and communicate with the
  delta robot arm by sending G-Code commands over a serial port.  The program is listed below.</p>
<hr class="clearLeft" />
  <?php code('c06_001_GCode.cpp'); ?>
<hr class="clearLeft" />
  <?php code('c06_002_GCode.cpp'); ?>
<hr class="clearLeft" />
<?php do_footer(); ?>
