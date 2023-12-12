<?php

require('library.php');

// function x($s) { return "$s\n"; }

function pp($s, $t) {
  return $s . $t . "\n";
}

function makePage($n) {
  $name = getName($n);
  $s = '';
  $s = pp($s, "<?php");
  $s = pp($s, "require('library.php');");
  $s = pp($s, "do_top($n, '$name');");
  if (hasWasm($n)) {
    $s = pp($s, 'do_wasmCanvas();');
  }
  $s = pp($s, "?>");
  $s = pp($s, "<p>blah blah blah ...</p>");
  $s = pp($s, "<?php do_footer(); ?>");
  return $s;
}

function main() {
  //$menuTabCount = 2;
  $menuTabCount = getMenuTabCount();
  for ($i = 25; $i < $menuTabCount; ++$i) {
    $s = makePage($i);
    $fn = getPageFileName($i);
    // echo("//file_put_contents($fn, $s);\n");
    file_put_contents($fn, $s);  // FIXME  FIXME  FIXME  FIXME
  }
}

main();

?>
