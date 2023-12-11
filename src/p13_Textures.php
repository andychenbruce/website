<?php
require('library.php');
do_top(13, 'Textures');
do_Buttons('Reset,Run,Pause');
do_WasmCanvas();
?>
<canvas id="imgCanvas" width="900" height="900"></canvas>

  <p>For this project, I load several images into the GPU video memory and apply them as textures to 3D surfaces.  Then I compute ray traces from the sphere to the displayed images to create reflections.</p>

  <p>You can use your mouse and drag to rotate the 3D image, or use the scroll wheel to zoom in or out.</p>

  <p>The code samples listed below are the application code I wrote in C++,
  and the fragment shader code I wrote in GLSL.</p>

  
<hr class="clearLeft" />
  <?php code('c13_001_MonaLisa.cpp'); ?>
<hr class="clearLeft" />
  <?php code('c13_002_MonaLisa.frag'); ?>
<hr class="clearLeft" />
    
<?php do_footer(); ?>
