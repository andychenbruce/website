<?php
require('library.php');
do_top(1, 'Capstone');
?>

<?php videoCap("Farmbot-ParkingLot-001-720.mp4",
	       "FarmBot controlled with a cell phone.") ?>

<hr class="clearLeft" />
<p>
Every year, 24 billion tonnes of topsoil are lost to erosion, farms emit four billion tonnes of CO2 from fossil fuels, and two million tonnes of herbicides and pesticides leech into our environment. Agriculture consumes 90% of our freshwater. A third of humanity toils on farms, an enormous waste of human potential.
</p>

  
<p>
These problems can all be reduced with agricultural robotics and artificial intelligence.  Robots enable no-till farming without plowing.  Precise application of herbicides and pesticides can reduce the use of agricultural chemicals by 95%. Direct injection irrigation uses 90% less water than flood irrigation while reducing fertilizer consumption. Polyculture can replace monoculture to increase yields.
</p>

  
<p>
For my senior Capstone project, I have designed, built, and programmed an agricultural robot for
<a href="https://en.wikipedia.org/wiki/Sustainable_agriculture">sustainable agriculture</a>.
</p>
  
<p>
  <?php imgCap('hydrogel.jpg', 1.0, 'Hydrogel gives water and nutrients to crops but not weeds'); ?>
  
My robot can plant crops by dispensing the seeds suspended in hydro-gel into the ground. The gel forms a mini-reservoir of water and nutrients around the seeds, speeding germination while depriving weeds of moisture. Water consumption is greatly reduced, and growth is accelerated.
</p>
  
<p>
Conventional farms apply fertilizer to entire fields. My robot injects fertilizer directly into the root zone of individual crop plants, reducing the need for fertilizer by 90% while eliminating runoff and inhibiting the growth of weeds.
</p>
  
<p>
Conventional farms use plowing to turn under weeds and weed seeds. But plowing leads to erosion and two billion tonnes of soil carbon loss every year to oxidation. An alternative is
<a href="https://en.wikipedia.org/wiki/No-till_farming">no-till farming</a>
which requires heavy use of herbicides. Using robotics for the precise application of herbicides can reduce the use of these problematic chemicals. Organic alternatives can be used since a robot can distinguish crops from weeds and only spray the weeds.
</p>
<hr class="clearLeft" />
<p>
  <?php imgCap("mono-vs-poly.jpg", 1.0, "Monoculture vs polyculture"); ?>
  
Conventional mechanized farming depends on monoculture, increasing the risk of diseases and insect infestation. Robotics and AI allow polyculture. Different crops can be planted together, or the plantings can be staggered so that one crop can sprout and grow in the shade of a more mature crop for an additional harvest each year.
</p>

<hr class="clearLeft" />
<p>
  <?php imgCap("RaspberryPi-4B.jpg", 1.0, "Raspberry Pi 4B"); ?>

My robot is a <a href="https://en.wikipedia.org/wiki/Differential_wheeled_robot">differential wheeled robot</a> with two pivot wheels and two independently motorized wheels used to both steer and propel the robot. The robot's computer is a
<a href="https://en.wikipedia.org/wiki/Raspberry_Pi">Raspberry Pi 4B</a> running Linux, and the vision system is a stereoscopic Arducam.
</p>
  
<p>
My robot uses a
<a href="https://en.wikipedia.org/wiki/Delta_robot">
Delta robot arm
</a>
to control the position of the actuator used for spraying, injecting, and dispensing seeds. A delta robot is not as precise as other robot arm geometries, but the positioning is much faster, and the millimeter-level accuracy is adequate for my application. The delta arm is controlled with
<a href="https://en.wikipedia.org/wiki/G-code">
G-Code
</a>, a standard programming language for industrial automation.
</p>
  
<p>
Although I will not complete my robot until May of 2022, it is mostly working. The motor control and navigation software are done and working. The vision system performs depth analysis and uses a neural network trained with
<a href="https://en.wikipedia.org/wiki/TensorFlow">
TensorFlow</a>
  to accurately discriminate between crop seedlings and weeds. I am currently building the end-effectors, including the sprayer, seed dispenser, and fluid injector.
</p>
  
<p>
I learned a tremendous amount by doing this project.  I learned how to install Linux, interface to the
<a href="https://en.wikipedia.org/wiki/Pulse-width_modulation">PWM</a>
timers, use
<a href="https://en.wikipedia.org/wiki/Memory-mapped_I/O">
memory-mapped I/O</a>, and write a device driver to read the motor encoders. I now understand pixel-level processing of images and learned to use neural nets with TensorFlow and
<a href="https://en.wikipedia.org/wiki/Keras">Keras</a>.
</p>
  
<p>
My robot has some limitations. It can only operate in good weather. It has limited battery capacity, which could be improved with solar power. Lights attached to the vision system would allow the robot to operate 24/7.
</p>
  
<p>
The application of robotics, computer vision, and artificial intelligence to sustainable agriculture is a fascinating subject and is an area where I believe I can make a significant contribution.
</p>

<br class="clearLeft" />
<hr class="clearLeft" />
<?php
do_footer();
?>
