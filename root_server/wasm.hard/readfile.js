//
//  readfile.js
//

//  https://stackoverflow.com/questions/47313403/passing-client-files-to-webassembly-from-the-front-end

function readStringFromC(s, len) {
  var p = new Uint8Array(Module.HEAPU8.buffer, s, len);
  var string = '';
  for (var i = 0; i < len; ++i) {
    string += String.fromCharCode(p[i]);
  }
  return string;
}

function c_readFile(idx, fileID, fnLen, filename) {
  console.log("c_readFile A: idx=" + idx + ",fileID=" + fileID + ", fnLen=" + fnLen + ",buf=" + filename);
  fn = readStringFromC(filename, fnLen);
  console.log("c_readFile B: fileID=" + fileID + ", fn={" + fn + "}");
  var xhr = new XMLHttpRequest();
  xhr.open('GET', fn, true);
  xhr.responseType = 'blob';
  xhr.onload = function(e) {
    if (this.status == 200) {
      var blob = this.response;
      console.log("BBBB blob.size=" + blob.size);
      var fileReader = new FileReader();
      fileReader.onload = function(event) {
	var arrayBuffer = event.target.result;
	var uint8Array = new Uint8Array(arrayBuffer);
	var sz = blob.size;
	var buf = Module._malloc(sz);
	Module.HEAPU8.set(uint8Array, buf);
	Module._readFileCallback(idx, fileID, sz, buf);
      };
      fileReader.readAsArrayBuffer(blob);
    }
  };
  xhr.send();
}

function c_getImgRGB(idx, fileID, fnLen, filename) {
  fn = readStringFromC(filename, fnLen);
  console.log("c_getImgRGB(filename={" + fn + "}, fnLen=" + fnLen + ")");
  var img = new Image();
  img.onload = function() {
    img.onload = null; // Set to null to prevent double triggering.
    
    var nw = img.naturalWidth;
    var nh = img.naturalHeight;

    var w = img.width;
    var h = img.height;
    
    console.log("## id=" + fileID +", {" + fn + "}  w=" +   w + ", h=" +  h);
    console.log("## id=" + fileID +", {" + fn + "} nw=" +  nw + ",nh=" + nh);
    
    var imgCanvas = document.getElementById("imgCanvas");

    console.log("@@ Foobar 1");

    // var imgCanvas = document.createElement('canvas');
    
    var ctx = imgCanvas.getContext('2d');

    console.log("@@ Foobar 2");
    
    ctx.drawImage(img, 0, 0);
    console.log("@@ Foobar 3");
    
    var imgData = ctx.getImageData(0, 0, w, h).data;
    console.log("@@ Foobar 4");

    var sz = imgData.length;
    
    console.log("## id=" + fileID +", {" + fn + "} length=" + sz);

    if (sz != (w * h * 4)) {
      console.log("### BAD SIZE " + sz + " != " + (w*h*4));
    }
    var rgba = Module._malloc(sz);
    Module.HEAPU8.set(imgData, rgba);
    Module._imgRGBCallback(idx, fileID, sz, w, h, rgba);
    
    // imgCanvas.remove();

    console.log("@@ id=" + fileID);
    
  };
  img.src = fn;
}
