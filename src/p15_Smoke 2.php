<?php
require('library.php');
do_top(15, 'Smoke');
do_Buttons('Reset,Run,Pause');
do_WasmCanvas();
?>

<p>To draw the flame, we create a series of circles that rise, expand, and diminish in opacity.</p>
<p>To minimize the number of fragments, each circle is represented as a "billboard" rather than a sphere.  A billboard is a flat sprite that is rotate in 3D space to always face the viewer.</p>
<hr class="clearLeft"/>
  <p>To implement the billboard, I take the transformation matrix, and
  reverse the rotation while leaving any scaling or translation.
  This is done by removing the scaling and translation, and then inverting
  the matrix, and multiplying that inverted matrix back to the
  original tranformation matrix. The code is listed below.</p>
<?php code('c15_001_Billboard.cpp'); ?>
<hr class="clearLeft"/>
  <p>Here is the rest of the code to draw the flame.</p>
<?php code('c15_002_Smoke.cpp'); ?>
<hr class="clearLeft"/>
  <p>Here is the fragment shader that runs on the GPU.</p>
<?php code('c15_003_Smoke.frag'); ?>
<?php do_footer(); ?>
