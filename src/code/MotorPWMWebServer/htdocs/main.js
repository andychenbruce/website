//
//  main.js
//

document.addEventListener('DOMContentLoaded', function(event) {
  'use strict';

  var g = {};
  
  async function sendCmd() {
    var xhttp = new XMLHttpRequest();
    var cmd = 'M,' +
	g.redDir + ',' + g.redLabel.innerHTML  + ',' +
	g.bluDir + ',' + g.bluLabel.innerHTML  + ',';
    
    console.log('{' + cmd + '}');
    
    var url = 'doCommand?cmd=' + cmd;
    xhttp.open('GET', url, true);
    xhttp.send();
  }

  function preventDefaults() {
    document.body.addEventListener('touchmove',function(e) {
      e = e || window.event;
      var target = e.target || e.srcElement;
      //in case $altNav is a class:
      if (!target.className.match(/\baltNav\b/)) {
        e.returnValue = false;
        e.cancelBubble = true;
        if (e.preventDefault) {
          e.preventDefault();
          e.stopPropagation();
        }
        return false;
      }
    },false);
  }

  function setupButtons() {
    g.redDir = 0; // CW=0, CCW=1
    g.bluDir = 0; // CW=0, CCW=1
    g.redCW  = getId('redCW');
    g.redCCW = getId('redCCW');
    g.bluCW  = getId('bluCW');
    g.bluCCW = getId('bluCCW');
    g.redCW.onclick  = function () { g.redDir = 0; sendCmd(); };
    g.bluCW.onclick  = function () { g.bluDir = 0; sendCmd(); };
    g.redCCW.onclick = function () { g.redDir = 1; sendCmd(); };
    g.bluCCW.onclick = function () { g.bluDir = 1; sendCmd(); };
  }
  
  function setupSliders() {
    function getId(s) { return document.getElementById(s); }
    preventDefaults();
    g.redSlider = getId('redSlider');
    g.bluSlider = getId('bluSlider');
    g.redLabel  = getId('redLabel');
    g.bluLabel  = getId('bluLabel');
    g.redSlider.oninput  = function() { g.redLabel.innerHTML  = this.value; sendCmd(); };
    g.bluSlider.oninput  = function() { g.bluLabel.innerHTML  = this.value; sendCmd(); };
  }

  console.log('Starting up, DOM loaded (002)');
  setupButtons();
  setupSliders();
});
