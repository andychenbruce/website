<?php

//require('library.php');

function makeThumbPage($tn) {
return <<<END
<!DOCTYPE html>
<html lang="en">
<head>
 <meta charset="utf-8" />
 <meta http-equiv="cache-control" content="no-store" />
 <meta http-equiv="pragma" content="no-cache" />
</head>
<body>
<div class="thumbDiv">
<canvas id=canvas width=200 height=200 oncontextmenu="event.preventDefault()"></canvas>
</div>
<script>
var Module = {
 canvas: document.getElementById("canvas"),
 arguments: ["-tab", $tn, "-thumb" ]
};
</script>
<script src="wasm/index.js"></script>
</body>
</html>\n
END;
}

function main() {
  // $menuTabCount = getMenuTabCount();
  for ($i = 0; $i < 25; ++$i) {
    $s = makeThumbPage($i);
    $fn = sprintf("thumb/thumb_%02d.html", $i);
    file_put_contents($fn, $s);
  }
}

main();

?>
