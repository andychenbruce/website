<?php
require('library.php');
do_top(11, 'Icosahedron');
do_Buttons('Reset,Run,Pause');
do_WasmCanvas();
?>

<?php imgCap("GoldenRectangle.jpg", 1.0, "Golden Rectangle"); ?>
To draw a sphere with OpenGL, we start with a <a href="https://en.wikipedia.org/wiki/Golden_rectangle">Golden Rectangle</a>.
<hr class="clearLeft" /><?php smallWasm(28, 'A'); ?>
Next, we draw three orthoganal golden rectangles centered on the origin.
<hr class="clearLeft" /><?php smallWasm(28, 'B'); ?>
By connecting the corners of the triangle, we get a 20-face
<a href="https://en.wikipedia.org/wiki/Icosahedron">Icosahedron</a>.
<hr class="clearLeft" /><?php smallWasm(28, 'C'); ?>
Then we can remove the rectangles since they were only there for educational purposes.
<hr class="clearLeft" /><?php smallWasm(28, 'D'); ?>
Then we draw the fragments for each triangular face.
<hr class="clearLeft" /><?php smallWasm(28, 'E'); ?>
The next step is to split each of the 20 triangular faces into four smaller triangles,
and then normalize each point so they are all the same distance from the origin.

This creates a polyhedron with 80 faces.  
<hr class="clearLeft" /><?php smallWasm(28, 'F'); ?>
Then we recursively repeat the procedure by subdividing every face again, to create a polyhedron with 320 faces.
<hr class="clearLeft" /><?php smallWasm(28, 'H'); ?>
<p>Now, let's recurse one more time to get 1280 faces, add a rotating light, and add a diffuse light effect to the sphere.
  The GLSL shader program to do that is listed.</p>
<?php code('c11_beachBall.001.frag'); ?>
  
<hr class="clearLeft" /><?php smallWasm(28, 'I'); ?>
If the sphere has a shiny surface, there will also be a spectral light effect.
<?php code('c11_beachBall.002.frag'); ?>

<hr class="clearLeft" /><?php smallWasm(28, 'J'); ?>
Now, we can remove the wire framework.
  
<hr class="clearLeft" /><?php smallWasm(28, 'K'); ?>
Since a solid red sphere is boring, let's make it into a beachball.
<?php code('c11_beachBall.003.frag'); ?>
<hr class="clearLeft" />
<?php smallWasm(28, 'L'); ?>
<p>
  Finally, we turn the light back on, with both diffuse and spectral effects.  First, we call <span class="codeSpan">beachBall()</span> to calculate the color, and then pass that value to <span class="codeSpan">shadedSphereWithSpecular()</span>.  The code for the GLSL fragment shader to do this is listed below.
</p>
<?php code('c11_beachBall.004.frag'); ?>
<p>
And that how you draw a sphere in OpenGL.  
</p>
<p>You can click on any of the images to rotate and zoom.</p>

<?php do_footer(); ?>
