CANVAS_WIDTH = 900;
CANVAS_HEIGHT = 700;

BOARD_SIZE = 5;

SQUARE_SIZE = 100; // pixels per square on board

// Types: r=room, p=passageway, b=blank
BOARD = [
  [
    {type:'r', label: 'Study'},{type:'p'},{type:'r',label:'Hall'},{type:'p'},{type:'r',label:'Lounge'},
  ],
  [
    {type:'p'},{type:'b'},{type:'p'},{type:'b'},{type:'p'},
  ],
  [
    {type:'r',label:'Library'},{type:'p'},{type:'r',label:'Billiard Room'},{type:'p'},{type:'r',label:'Dining Room'},
  ],
  [
    {type:'p'},{type:'b'},{type:'p'},{type:'b'},{type:'p'},
  ],
  [
    {type:'r',label:'Conservatory'},{type:'p'},{type:'r',label:'Ballroom'},{type:'p'},{type:'r',label:'Kitchen'},
  ],
];

function setUpCanvas() {
  var canvas = $('canvas#gameboard')[0];
  canvas.width = CANVAS_WIDTH;
  canvas.height = CANVAS_HEIGHT;
  var ctx = canvas.getContext('2d');
  for (var i = 0; i < BOARD_SIZE; i++) {
    for (var j = 0; j < BOARD_SIZE; j++) {
      current_node = BOARD[j][i];
      // Paint the tile
      if (current_node.type == 'r') {
        ctx.fillStyle = "yellow";
      } else if (current_node.type == 'p') {
        ctx.fillStyle = "lightgray";
      } else {
        ctx.fillStyle = "white";
      }
      ctx.fillRect(i * SQUARE_SIZE, j * SQUARE_SIZE, SQUARE_SIZE, SQUARE_SIZE); // x,y,width,height
      // Paint any labels
      if (current_node.label !== undefined) {
        console.log(current_node,i,j);
        ctx.font = "15px serif";
        ctx.fillStyle = "black";
        ctx.fillText(current_node.label, i * SQUARE_SIZE + 5, j * SQUARE_SIZE + 25);
      }
    }
  }
}

$(document).ready(function() {
  setUpCanvas();
})
