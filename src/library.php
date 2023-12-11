<?php

$g_menuIndex = 0;
$g_buttonString = '';

function fatal($s) {
  fwrite(STDERR, "FATAL: $s", PHP_EOL);
  exit(1);
}

function g_cdn() {
  return '//cdnjs.cloudflare.com/ajax/libs/highlight.js/11.2.0';
}

function g_codeHighlightJS()  { return g_cdn() . '/highlight.min.js'; }
function g_codeHighlightCSS() { return g_cdn() . '/styles/default.min.css'; }

$g_menuTab = [
//  #  Tab Code Math UC
 [  0,  0,  0,   0,   0,   0,   0,   0, 'None',         'None - Zero not used' ],
 [  1,  0,  0,   0,   0,   0,   0,   0, 'Capstone',     'Capstone: A Robot for Sustainable Agriculture' ],
 [  2,  0,  1,   0,   0,   0,   0,   0, 'PWM',          'PWM on a Raspberry Pi using direct hardware access' ],
 [  3,  0,  1,   0,   0,   0,   0,   0, 'PhoneCtrl',    'Controlling a Raspberry Pi with a Cell Phone' ],
 [  4,  0,  1,   0,   0,   0,   0,   0, 'MotorPWM',     'Motor Speed Control using Raspberry Pi PWM' ],
 [  5,  0,  1,   0,   0,   0,   0,   0, 'EncoderDrvr',  'Linux Device Driver for a Motor Encoder' ],
 [  6,  0,  1,   0,   1,   0,   0,   0, 'GCode',        'Controlling a Delta-X Robot Arm with G-Code' ],
 [  7,  0,  1,   0,   0,   0,   0,   0, 'ImageSeg',     'Isolating and Segmenting Images of Seedlings' ],
 [  8,  0,  1,   0,   1,   0,   0,   0, 'DepthPercept', 'Depth Perception in Images of Seedlings' ],
 [  9,  0,  1,   0,   0,   0,   0,   0, 'TensorFlow',   'Deep Learning with TensorFlow and Keras to Classify Seedlings' ],
 [ 10,  3,  1,   1,   0,   0,   0,   0, 'Mandelbrot',   'Using a GPU to Draw the Mandelbrot Set' ],
 [ 11, 28,  1,   0,   0,   0,   0,   0, 'Icosahedron',  'Drawing a Sphere by Recursive Segmentation of an Icosahedron' ],
 [ 12, 12,  1,   1,   0,   0,   0,   0, 'Shadows',      'Lighting and Shadows in a 3D Scene' ],
 [ 13, 10,  1,   0,   0,   0,   0,   0, 'Textures',     'Apply Textures to 3D Surfaces' ],
 [ 14,  1,  1,   0,   1,   0,   0,   0, 'Earth',        'Apply Textures to a Sphere' ],
 [ 15, 29,  1,   0,   0,   0,   0,   0, 'Smoke',        'Smoke and Fire Simulation' ],
 [ 16, 20,  1,   1,   0,   0,   0,   0, 'Jello',        "Simulating Jello with Hooke's Law" ],
 [ 17, 17,  1,   1,   0,   0,   0,   0, 'Boltz',        "Non-Ideal Gas Simulation and Boltzmann's Law" ],
 [ 18, 23,  1,   0,   0,   0,   0,   0, 'Marbles',      'Liquid Simulation using Particles' ],
 [ 19, 26,  1,   1,   0,   0,   0,   0, 'NavierStokes', 'Fluid Flow Simulation with the Navier-Stokes Equation' ],
 [ 20,  9,  1,   1,   0,   0,   0,   0, '2Body',        'Closed Form Solution to the Two Body Problem' ],
 [ 21, 22,  1,   1,   0,   0,   0,   0, 'IMF',          'Intermolecular forces condensation simulation' ],
];

// [ 10,  0,  1,   0,   1,   0,   0,   0, 'OpenGLSimple', 'OpenGL and Shader Pipelines' ],
// [ 11,  0,  1,   0,   1,   0,   0,   0, 'Wasm',         'OpenGL in Webassembly' ],
// [ 13,  0,  1,   0,   1,   0,   0,   0, 'Shadows',      'Rotating and Scaling a 3D Scene' ],
// [ 16,  0,  1,   0,   1,   0,   0,   0, 'Reflections',  'Reflections and Raytracing' ],
// [ 22,  0,  1,   0,   1,   0,   0,   0, 'STL',          'Drawing 3D STL Files in OpenGL' ],

function p($s) {
  echo($s);
  echo("\n");
}

function getMenuTabEntry($n) {
  global $g_menuTab;
  return $g_menuTab[$n];
}

function getMenuTabEntryField($n, $f) {
  $t = getMenuTabEntry($n);
  return $t[$f];
} 

function getMenuTabCount() {
  global $g_menuTab;
  return count($g_menuTab);
}

function wasmTabNumber($n) { return getMenuTabEntryField($n, 1); }
function hasWasm($n)       { return getMenuTabEntryField($n, 1) > 0; }
function hasCode($n)       { return getMenuTabEntryField($n, 2) > 0; }
function hasMath($n)       { return getMenuTabEntryField($n, 3) > 0; }
function underConstruct($n){ return getMenuTabEntryField($n, 4) > 0; }
function getName($n)       { return getMenuTabEntryField($n, 8); }
function getHeadline($n)   { return getMenuTabEntryField($n, 9); }

function getPageFileName($n) {
  $name = getName($n);
  return sprintf('p%02d_%s.php', $n, $name);
}

function checkName($n, $name) {
  if (getName($n) != $name) {
    trigger_error("Bad name: $name", E_USER_ERROR);
  }
}

function getHeaderString($n, $title) {
  $s =<<<END
    <!DOCTYPE html>
    <html lang="en">
    <head>
      <meta charset="utf-8" />
      <meta http-equiv="cache-control" content="no-store" />
      <meta http-equiv="pragma" content="no-cache" />
    END;
  $s .= "\n<title>$title</title>\n";
  if (hasCode($n)) {
    $s .= '<link rel="stylesheet" href="' . g_codeHighlightCSS() . '">' . "\n";
  }
  $s .=<<<END
    <link rel="stylesheet" href="style.css" />  
    </head>
    <body>
    <div class="wrapperDiv">
    END;
  return $s;
}

function do_header($n, $title) {
  global $g_menuIndex;
  $g_menuIndex = $n;
  p(getHeaderString($n, $title));
}

function do_menubar() {
  function btn($fn, $t) {
    p('<li class="mb_li"><a class="button" href="' . $fn . '.php">' . $t . '</a></li>');
  }
  p('<div class="mainMenuBarDiv">');
  p('<ul class="mainMenuBarList">');
  btn('index',       'Home');
  btn('menuPage',    'Menu');
  btn('aboutPage',   'About');
  btn('contactPage', 'Contact');
  btn('helpPage',    'Help');
  p('</ul>');
  p('</div>');
}

function do_headerDiv($headline) {
  p('<div class="headerDiv">');
  p("<h1>$headline</h1>");
  p('</div>');
}

function do_top($n, $name) {
  $headline = getHeadline($n);
  if ($n == 0) {
    $headline = $name;
  } else {
    checkName($n, $name);
  }
  do_header($n, $headline);
  do_menuBar();
  do_headerDiv($headline);
  p('<div class="contentDiv">');
}

function highlightCode() {
  global $g_codeHighlightJS;
  p('<script src="' . g_codeHighlightJS() . '"></script>');
  p('<script>hljs.highlightAll();</script>');
}


function makeButtonArray() {
  global $g_buttonString;
  if ($g_buttonString == '') {
    return;
  }
  if ($g_buttonString == 'None') {
    p("g.buttonArray = [];\n");
    return;
  }
  $buttonKeys = array(
    'Reset'  => 'R',
    'Run'    => 'r',
    'Pause'  => 'S',
    'Frags'  => 'f',
    'Floor'  => 'z',
    'Wires'  => 'w',
    'Cage'   => 'c',
    'Lights' => 'l',
    'Anchor' => 'a',
    '1'      => '1',
    '2'      => '2',
    '3'      => '3',
    '4'      => '4',
    '5'      => '5');
  $s = "g.buttonArray = [\n";
  $ba = explode(',', $g_buttonString);
  foreach ($ba as $b) {
    $k = $buttonKeys[$b];
    if ($k == null) { fatal("Bad button label: $b"); }
    $s .= "[ 0, '$k', '$b' ],\n";
  }
  $s .= "];";
  p($s);			 
  return $s;			 
}

function echoWasmScripts($n) {
  $tn = wasmTabNumber($n);
  if ($tn == 0) {
    return;
  }
  // p('<script src="xlib/jquery-3.5.1.slim.min.js"></script>');
  p('<script>');
  p('var g = {};');
  p('g.smallButtons = true;');
  makeButtonArray();
  p('var Module = {');
  p(' canvas: document.getElementById("canvas"),');
  p(' arguments: ["-tab", "' . $tn . '", "-smallButtons" ]');
  p('};');
  p('</script>');
  p('<script src="wasm/readfile.js"></script>'); // FIXME
  p('<script src="wasm/buttons.js"></script>');
  p('<script src="wasm/index.js"></script>');
}

function loadMathScript() {
  p(<<<END
<script type="text/x-mathjax-config">
 MathJax.Hub.Config({tex2jax: {inlineMath: [['$','$'], ['\\\\(','\\\\)']]}});
</script>
<script type="text/javascript"
src="http://cdnjs.cloudflare.com/ajax/libs/mathjax/2.7.1/MathJax.js?config=TeX-AMS-MML_HTMLorMML">
</script>
END);
}

function do_footer() {
  global $g_menuIndex;
  $mi = $g_menuIndex;
  if (underConstruct($mi)) {
    p('<img src="imgs/Under-Construction.jpg" />');
  }
  p('<hr class="clearLeft" />');  // Final horizonal line
  p('</div>'); // Close contentDiv
  p('</div>'); // Close wrapperDiv
  if (hasCode($mi)) {
    highlightCode();
  }
  if (hasMath($mi)) {
    loadMathScript();
  }
  echoWasmScripts($mi);
  p('</body>');
  p('</html>');
}

function loadContent($fn) {
  p('<div class="contentDiv">');
  require("$fn.php");
  p('</div>');
}

function loadContent2($fn1, $fn2) {
  p('<div class="contentDiv">');
  require("$fn1.php");
  require("$fn2.php");
  p('</div>');
}

function do_wasmCanvas() {
  p('<div id="dvMain">');
  p('<div id="dvSmallButtons" class="dvB"></div>');
  p('<div id="dvLargeButtons" class="dvB"></div>');
  p('<canvas id="canvas" width="800" height="800" oncontextmenu="event.preventDefault()"></canvas>');
  p('</div>');
  p('<hr class="clearLeft" />');
}

function do_Buttons($s) {
  global $g_buttonString;
  $g_buttonString = $s;
}

function imgCap($img, $scale, $caption) {
  $img = "imgs/$img";
  $imgSz = getImageSize($img);
  if (! $imgSz) {
    throw new Exception("File not found: $img");
  }
  $w = (int) round($scale * $imgSz[0]);
  $h = (int) round($scale * $imgSz[1]);
  p('<div class="imageContainer" class="floatLeft">');
  printf('<img src="%s" width="%d" height="%d" />', $img, $w, $h);
  printf('<div class="imageCaption" style="width:%d px;">', $w);
  p($caption);
  p('</div></div>');
 }


function videoCap($vid, $caption) {
  p('<div class="imageContainer" class="floatLeft">');
  printf('<video class=floatLeft width="640" controls>');
  printf('<source src="video/%s" type="video/mp4" />', $vid);
  p('Your browser does not support the video tab.');
  p('</video>');
  printf('<div class="imageCaption" style="width:640px;">');
  p($caption);
  p('</div></div>');
}

function smallWasm($tab, $key) {
  p('<iframe class="size400" src="size400.html?t=' . "$tab&k=$key" . '"></iframe>');
}

function code($fn) {
  p('<pre><code class="codeClass" >');
  $s = file_get_contents("code/$fn");
  $s = str_replace('<', '&lt;', $s);
  $s = str_replace('>', '&gt;', $s);
  echo($s);
  p('</code></pre>');
}
