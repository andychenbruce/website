//
//  button.js
//


document.addEventListener("DOMContentLoaded", function(event) {

  function getButtonArray() {
    if (typeof g.buttonArray !== 'undefined') {
      return g.buttonArray;
    }
    if (g.smallButtons) {
      return [
	[ 0, 'R', 'Reset' ],
	[ 0, 'r', 'Run'   ],
	[ 0, 'S', 'Pause' ],
	[ 0, 'f', 'Frags' ],
	[ 0, 'w', 'Wires' ],
	[ 0, 'c', 'Cage'  ],
	[ 0, 'l', 'Lights' ],
	[ 0, 'a', 'Anchor' ],
	[ 0, '1', 'A' ],
	[ 0, '2', 'B' ],
	[ 0, '3', 'C' ],
	[ 0, '4', 'D' ],
	[ 0, '5', 'E' ]
	[ 0, '6', 'F' ]
	[ 0, '7', 'G' ]
	[ 0, '8', 'H' ]
      ];
    }
    return [
      [ 2, 'R', 'Reset',  'm', 'Menu' ],
      [ 2, 'l', 'Lights', 'r', 'Run'  ],
      [ 2, 'a', 'Anchor', 'S', 'Pause' ],
      [ 2, 'w', 'Wires',  's', 'Step' ],
      [ 2, 'f', 'Frags',  'c', 'Cage' ],
      [ 4, 44 ],
      [ 1, 'Speed'    ],
      [ 1, 'Ambient'  ],
      [ 1, 'Diffuse'  ],
      [ 1, 'Whatever' ],
      [ 5, 'P' ],
      [ 2, 'd', 'Debug',  'Q', 'Quit' ]
    ];
  }

  function buttonClick() {
    var d = this.getAttribute('data');
    var n = parseInt(d);
    if (isNaN(n)) {
      n = d.charCodeAt(0);
    }
    //console.log("click n={" + n + "} - {" + th.text() + "}");
    _sendMessage(n, 1);
  }

  function getButtonHTML() {
    var i, j;
    var h = '';
    var buttons = getButtonArray();
    for (j = 0; j < buttons.length; ++j) {
      var bx = buttons[j];
      var bn = bx[1];
      switch (bx[0]) {
	
      case 0:
	h += '<button class=bb0 data=' + bx[1] + '>' + bx[2] + '</button>';
	break;
	
      case 1:
	h += '<button class=bb1 data=' + bn + '>' + bn + '</button>';
	break;

      case 2:
	h += '<button class=bb2 data=' + bx[1] + '>' + bx[2] + '</button>';
	h += '<button class=bb2 data=' + bx[3] + '>' + bx[4] + '</button>';
	break;

      case 4:
	for (i = 0; i < bx[1]; ++i) {
	  h += '<button class=bb4 data=' + i + '>' + i + '</button>';
	}
	break;

      case 5:
	var n = bn.charCodeAt(0);
	var a = 'A'.charCodeAt(0);
	for (i = a; i <= n; ++i) {
	  var s = String.fromCharCode(i);
	  h += '<button class=bb4 data=' + s + '>' + s + '</button>';
	}
	break;
	
      default:
	alert("Bad case");
	break;
      }
    }
    return h;
  }

  (function() {
    console.log("DOM loaded - buttons.js");

    function byId(id) { return document.getElementById(id); }
    function fixBut(dv1, dv2) {
      dv1.innerHTML = getButtonHTML();
      dv2.parentNode.removeChild(dv2);
    }
    var dvSmall = byId('dvSmallButtons');
    var dvLarge = byId('dvLargeButtons');
    var cv      = byId('canvas');
    
    if (g.smallButtons) {
      fixBut(dvSmall, dvLarge);
      cv.style.left = "0";
      dvSmall.style.zIndex = "999";
    } else {
      fixBut(dvLarge, dvSmall);
      cv.style.left = "256px";
    }
    var buttonElements = document.getElementsByTagName('button');
    for (i = 0; i < buttonElements.length; ++i) {
      buttonElements[i].onclick = buttonClick;
    }
  })();
});
