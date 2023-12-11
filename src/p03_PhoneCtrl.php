<?php
require('library.php');
do_top(3, 'PhoneCtrl');
?>

<p>
In <a href="p02_PWM.php">the previous project</a>, I implemented PWM on
a Raspberry Pi using direct hardware access.
</p>
  
<p>In this project, I will implement a web-app to control the
  PWM from a browser running on a phone.</p>

<p>The first step is to write an HTTP server to open a
  <a href="https://en.wikipedia.org/wiki/Network_socket">TCP/IP socket</a> and
  send requested files to the client browser.  The code to do that is listed
  below.</p>
								
<?php code('LEDWebServer/server.c'); ?>

<p>We also need some utility functions for dealing with the
  socket requests. These are listed below:</p>

<?php code('LEDWebServer/socket.c'); ?>

<p>Next, we need the HTML and JavaScript files to create the
  user interface in the phone's browser.</p>
<?php code('LEDWebServer/htdocs/index.html'); ?>
<?php code('LEDWebServer/htdocs/main.js'); ?>

<p>Ok.  Now let's run it.  A normal HTTP server is on port 80, but that port
  may already be in use, so we can use port 12345 instead.  It could be any
  unused port number, as long as the server and client agree. We also need
  the internet address of the Raspberry Pi.  To get that we run
  <span class="codeSpan">hostname -I</span>.</p>
<div class="terminalDiv">
<pre>
> hostname -I
192.168.0.115
>
>
</pre>
</div>
<?php videoCap('phonePwm480.mp4', 'PWM control with a phone'); ?>
<p>So we start the server on the Raspberry Pi, and then connect
  over Wifi by entering the URL
  <span class="codeSpan">http://192.168.0.115:12345/</span>.</p>
<p>When the interface is displayed, we can used the sliders
  to control the frequency and the pulse width (duty cycle) of
  the signal driving the LEDs. Click on the video to see a demo.</p>
</p>


  
<?php   
do_footer();
?>
