<?php
require('library.php');
do_top(16, 'Jello');
do_Buttons('Reset,Run,Pause,Frags,Wires');
do_WasmCanvas();
?>
<h2>How it works</h2>
<p>A cube of 10x10x10 points are all connected up to 26 of their neighbors by springs following Hookes Law</p>
&nbsp;&nbsp;&nbsp;$F = -kx$
<p>where $k$ is a constant and $x$ is the distance from the spring's rest position.</p>
</br>
<p>An efficient way to index through the cube of points and apply the force for each point to its neightbors is to start indexing at 0, 0, 0 to n, n, n, by indexing across x, then y, then z, so doing row, column, then layer. For each point it's index can apply the force to its neighbors only one time by applying it to its x-1, the 3 on top of it, and the 9 behind it, making an L shape in front of a square. It's hard to see but when applied in order of row, column, then layer it links every particle to its 26 neighbors since it applies to the previous particles in layer, column, then row.</p>

  <p>If there is enough momentum in a particle it can phase through a layer and get stuck in the wrong position. To compensate particles are given a radius (in this simulation about 0.4 times the spring resting length) turning them into spheres, which are given elastic collissions if they hit any other sphere. The elastic collision from <a href="https://en.wikipedia.org/wiki/Elastic_collision#Two-dimensional">here</a>. It would force spheres back into position. They will still phase into the wrong layer if going fast enough in this simulation since each sphere can only have one elastic colission per physics tick. The solution would to keep applying collisions untill every sphere is relatively moving away from eachother, but that will slow down the simulation significanly with all the extra math per frame, so keeping velocities and forces low keep the simulation from breaking.</p>

<p>The interaction between the ball and the cube is just the exact same elastic collission as the particle to particle interaction, just the ball has a mutch bigger radius. This could also easily be applied to cube-cube interation since it is just more elastic collisions</p>

<p><b>To do:</b>  Add shadows.</p>
<?php do_footer(); ?>
