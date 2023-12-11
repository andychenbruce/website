<?php

require('library.php');

function php2html($fn) {
    $cmd = "php $fn.php | sed 's/\\.php\"/.html\"/g' > ../$fn.html";
    echo("$cmd\n");
//    system($cmd);
}

function generateHtmlFromPhp() {
  php2html('index');
  php2html('aboutPage');
  php2html('contactPage');
  php2html('helpPage');
  php2html('menuPage');
  $mc = getMenuTabCount();
  for ($i = 0; $i < $mc; ++$i) {
    $fn = sprintf("p%02d_%s", $i, getName($i));
    php2html($fn);
  }
}

function ww($fn) {
  echo("cp -p ../../glfw/bb-slim/$fn ../wasm.hard\n");
}

function copyWasmFiles() {
  ww('buttons.js');
  ww('index.js');
  ww('index.wasm');
  ww('readfile.js');
}

function main() {
  generateHtmlFromPhp();
  copyWasmFiles();
  echo('cp -p size400.html ..');
}

main();

?>
