//
//  main.js
//

document.addEventListener('DOMContentLoaded', function(event) {
  'use strict';

  var g = {};
  
  async function sendCmd() {
    var xhttp = new XMLHttpRequest();
    var cmd = 'L,' +
      g.redFreqLbl.innerHTML   + ',' +
      g.redWidthLbl.innerHTML  + ',' +
      g.blueFreqLbl.innerHTML  + ',' +
	g.blueWidthLbl.innerHTML + ',';
    
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
        return false;//or return e, doesn't matter
      }
    },false);
  }
  
  function setupSliders() {
    function getId(s) { return document.getElementById(s); }

    preventDefaults();
    
    g.redFreqSlider   = getId('redFreqSlider');
    g.redWidthSlider  = getId('redWidthSlider');
    g.blueFreqSlider  = getId('blueFreqSlider');
    g.blueWidthSlider = getId('blueWidthSlider');

    g.redFreqLbl   = getId('redFreqLbl');
    g.redWidthLbl  = getId('redWidthLbl');
    g.blueFreqLbl  = getId('blueFreqLbl');
    g.blueWidthLbl = getId('blueWidthLbl');
  
    g.redFreqSlider.oninput   = function() { g.redFreqLbl.innerHTML   = this.value; sendCmd(); };
    g.redWidthSlider.oninput  = function() { g.redWidthLbl.innerHTML  = this.value; sendCmd(); };
    g.blueFreqSlider.oninput  = function() { g.blueFreqLbl.innerHTML  = this.value; sendCmd(); };
    g.blueWidthSlider.oninput = function() { g.blueWidthLbl.innerHTML = this.value; sendCmd(); };
  }

  console.log('Starting up, DOM loaded (001)');
  setupSliders();
});
