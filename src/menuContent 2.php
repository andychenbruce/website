<?php

function menuLink($n) {
  $fn = getPageFileName($n);
  $hl = getHeadLine($n);
  p('<li><a href="' . $fn . '">' . $hl . '</a><hr/></li>');
}

function makeMenuContent() {
  $menuTabCount = getMenuTabCount();
  p('<h3><ol>');
  for ($i = 1; $i < $menuTabCount; ++$i) {
    menuLink($i);
    //p('<hr />');
    //p('<hr class="clearLeft" />');
  }
  p('</ol></h3>');
}

makeMenuContent();

?>
