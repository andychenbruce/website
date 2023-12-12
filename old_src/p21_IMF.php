<?php
require('library.php');
do_top(21, 'IMF');
do_Buttons('Reset,Run,Pause,Cage,Floor');
do_wasmCanvas();
?>
<h2>Intermolecular force simulator</h2>
<br/>
<p>I modeled it off of the Lennard-Jones potential (<a href="https://en.wikipedia.org/wiki/Lennard-Jones_potential">wikipedia</a>)</p>
<br/>
<p>Each sphere is basically one atom or molecule and it's potential energy is the sum of its Lennards-Jones potential relative to every other atom</p>
$$E(r) = 4\epsilon [({\sigma \over r})^{12} - ({\sigma \over r})^{6}]$$
<br/>
where
$E(r)$ is the potential energy for that distance
<br/>
$r$ is the distance between particles
<br/>
$\epsilon$ is the depth of the potential well, basically the potential energy at the minimum or most stable
<br/>
$\sigma$ is where potential energy is zero, basically the size of the particle.
<br/>
<p>Note that particles won't go to the distance where potential energy is zero, but will go to the bottom of the potential well where potential energy is negative, slightly before the edge of the particle where potential energy is zero.</p>
<br/>
The bottom of the energy well is $r = {2^{1/6}}\sigma$
<br/>
<p>A more accurate simulation would put the potential energy into the potential energy function in Schrödinger's equation at every point to model the change in the probability waves of atoms. But that is really hard to program since it uses derivatives of vector fields. Instead, it can be approximated for a worse but way faster and easier numerical solution.</p>
<br/>
$v$ = change in distance of nuclei, like velocity but only the direction toward the other nuclei
<br/>
$r$ = distance between nuclei
<br/>
$K$ = kinetic energy of $v$
<br/>
<br/>
<br/>
$v = {dr \over dt}$
<br/>
$K = {1 \over 2}v^2$
<br/>
taking the derivative of $K$ with respect to $t$
<br/>
${dK \over dt} = v{dv \over dt}$
<br/>
dividing both sides by $v$
<br/>
${dK \over dt}{1 \over v} = {dv \over dt}$
<br/>
substitute $v$ for ${dr \over dt}$
<br/>
${dK \over dt}{1 \over {dr \over dt}} = {dv \over dt}$
<br/>
simplify to
<br/>
${dK \over dt}{{dt \over dr}} = {dv \over dt}$
<br/>
the $dt$'s cancel to become
<br/>
${dK \over dr} = {dv \over dt}$
<br/>
The change in velocity over the change in time is just acceleration so
<br/>
acceleration = ${dK \over dr}$
<br/>
Now let's assume that kinetic energy is the only energy exchanged with potential energy, ignoring electron levels, etc. So the only place a drop in potential energy can go is into kinetic energy, so
<br/>
acceleration = ${dK \over dr} = -{dE(r) \over dr}$
<br/>
Taking the derivative of the Lennards-Jones potential gives
<br/>
${dE(r) \over dr} = 4\epsilon [-12{{({\sigma \over r})^{13}} \over \sigma} + 6{{({\sigma \over r})^{7}} \over \sigma}]$
<br/>
So finally an easily implementable numeric solution
<br/>
$$ acceleration = 4\epsilon [-12{{({\sigma \over r})^{13}} \over \sigma} + 6{{({\sigma \over r})^{7}} \over \sigma}]$$
<br/>
where the accelation is directly towards the other atom
<br/>
<br/>
<br/>
<p>In the simulation above this exact equation is implemented except when $r$ approaches very close to $\sigma$ because the Lennards-Jones potential has an extremely steep incline, so the derivative makes an extremely large acceleration. Because the simulation only does one physics tick in a set time frame it can't simulate sharp changes. Realistically the atoms would be pushed apart before it reaches that close the simulation does have atoms get that close due to a large enought $dt$ causing innaccuraccies in each time tick. To compensate the equation is disabled for when $r < 1.07\sigma$, and instead applies an elastic collission which is closer to real life than the numerical solution. The elastic collision equation in 3D is taken from <a href="https://en.wikipedia.org/wiki/Elastic_collision#Two-dimensional">here</a></p>
<br/>
<p>There are still glitches in the elastic collision detection since each sphere is only allowed one collision with every other sphere. When around a lot of other spheres it is pulled in harder than the elastic collisions push it out. The solution would to keep colliding a sphere with its neightbor until the velocity vectors are moving apart within the same physics tick. But that uses a lot of computing power and the simulation already doesn't run well, but the basic concept of IMF simulation is still there</p>
<?php
do_footer();
?>
