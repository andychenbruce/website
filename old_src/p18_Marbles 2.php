<?php
require('library.php');
do_top(18, 'Marbles');
do_Buttons('Reset,Run,Pause,Cage,Floor');
do_WasmCanvas();
?>
</br>
Interactive, try to drag, rotate and zoom. Allowed since it uses Web Assembly.
</br>
</br>
  This simulation is extremely simple. Each ball has a constant acceleration in a vector that goes downwards relative to the camera for gravity, and each time frame, if a ball is touching another ball apply an elastic collision according to <a href="https://www.sjsu.edu/faculty/watkins/collision.htm">this website</a>.
</br>
</br>
  I was origionally going to add surface tention by simulating the intermolecular-forces between the particles to see if it would look better, but the problem there is that each ball acts like a molecule and not a percentage of the water and basically as if it was on a microscopic scale, not macroscpic where the IMF's are usually neglegable. A link to the actual IMF simulation is found on the home page, but it doesn't look as good as this simulation.
</br>
<?php do_footer(); ?>
