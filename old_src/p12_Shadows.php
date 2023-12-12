<?php
require('library.php');
do_top(12, 'Shadows');
do_buttons('Reset,Frags,Wires');
do_WasmCanvas();
?>
<p>For a point light source the amount of light hitting a surface is based on the cosine of the angle between the normal vector to the surface and the vector to the light source. For diffusion surfaces the brighness is almsot exacly linear interpolation between ambient and fully illuminated using the cosine of the angle.</p>

<p>the cosine of the angle can be calculated by the dot product definition</p>
$cos(\theta) = {\vec{n}\cdotp\vec{l} \over |\vec{n}||\vec{l}|}$
</br>
then if
</br>
$a$ = ambient multiplier (no light)
</br>
$f$ = full illumination multiplier (full light)
</br>
</br>
then the diffuse color would be about
</br>
diffuse color = $color_{diffuse}$ = (r', g', b') = (r, g, b)*lerp($a$, $b$, $cos(\theta)$)
</br>
where the "lerp" is equivelent to
</br>
$a + (b - a)cos(\theta)$
</br>
</br>
when implemented a problem where when $cos(\theta)$ is negative makes it so past 180 degrees away from the light can make negative brightness which doesn't make sense, so $cos(\theta)$ must be clamped between 0 and 1, or basically if it is below 0 set it to 0.
</br>
</br>
$\vec{l}$ is easy to calculate, it would just be the light position minus the point at which is being colored in (final minus initial)
</br>
</br>
$\vec{n}$, the normal vector, depends on the shape of the object, but for spheres it is really easy as it is just the difference between the point and the center of the sphere since a spheres surface is always tangent to the center so the normal vector is just straight out
</br>
</br>
For specular light it is based on the angle between the reflected vector from the camera and the light source, assuming the light source is a point.
</br>
</br>
the reflected vector would be the reflection of $\vec{l}$ and $\vec{n}$, where the equation for reflection is $l_{ref} = l-{2l\cdotp n \over |n|^2}n$, then to find $cos(\theta_{ref})$ just use the same dot product proof, then also clamp it between 0 and 1 again so that it avoid negative values
</br>
</br>
the actual color value is based on a power of $cos(\theta_{ref})$, the power being the shinyness
</br>
specular factor = $f = cos(\theta_{ref})^s$, where $s$ is shinyness, and then the actual color is lerp($color_{diffuse}$, $color_{light}$, $f$), which is the equivelent of
</br>
final color = $f*color_{light} + (1-f)*color_{diffuse}$
</br>
this means that when $f$ is big or when $cos(\theta_{ref})$ is near, meaning the angle is small the color is close to the light since it is reflecting into the camrea, and when $cos(\theta_{ref})$ is small or the angle is big the sphere returns to its regular diffuse color
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
<?php do_footer(); ?>
