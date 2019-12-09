BigFloat32 = require('bigfloat').BigFloat32;

var canvas = document.getElementById("c");
var ctx = canvas.getContext("2d");

var image = new Image();
image.onload = function() {
  ctx.drawImage(image, 0, 0);
};

var state = {
  cx: new BigFloat32(0.0),
  cy: new BigFloat32(0.0),
  scale: new BigFloat32(0.05),
  zoomStep: new BigFloat32(0),
  width: 800,
  height: 800,
}

function setZoomStep() {
  state.zoomStep = state.scale.mul(0.5);
}

async function getData() {
  let url = `http://localhost:8088/?cx=${state.cx}&cy=${state.cy}&scale=${state.scale}&width=${state.width}&height=${state.height}`;
  let response = await fetch(url);

  if (response.ok) {
    let body = await response.text();
    image.src = body;
    console.log("Loaded")
  } else {
    alert("HTTP-Error: " + response.status);
  }

}

function zoom(directionIn) {
  zoomCenter(directionIn, state.width/2, state.height/2)
}

async function zoomCenter(directionIn, x, y) {
  // let step = directionIn ? state.zoomStep : -state.zoomStep;

  let nx = new BigFloat32(x);
  let ny = new BigFloat32(y);


  let sx = nx.sub(state.width / 2).mul(state.scale).add(state.cx);
  let sy = new BigFloat32(state.height / 2 - y).mul(state.scale).add(state.cy);

  if(directionIn) {
    state.scale = state.scale.sub(state.zoomStep);
  } else {

    state.scale = state.scale.add(state.zoomStep);
  }

  state.cx = sx;
  state.cy = sy;

  setZoomStep();
  getData();
}



canvas.addEventListener("click", function(event) {
  let x = event.offsetX;
  let y = event.offsetY;

  console.log(`x ${x} y ${y}`)

  zoomCenter(true, x, y)
});

canvas.oncontextmenu = function(event) {
  let x = event.offsetX;
  let y = event.offsetY;

  console.log(`x ${x} y ${y}`)
   zoomCenter(false, x, y)
   return false;
}

document.getElementById("in").addEventListener("click", () => zoom(true));
document.getElementById("out").addEventListener("click", () => zoom(false));

// Start off
setZoomStep()
getData()

let x = 32;
console.log(new BigFloat32(x).add(1).sub(x).toString());
