$(document).ready(function() {
  // When they type anything in the name box, show the next button
  $('#txtName').on('input', function() {
    var name = $('#txtName').val();
    if (name.length > 0) {
      $('#next1').show();
    } else {
      $('#next1').hide();
    }
  });
  // on #next1 click, show next screen
  $('#next1').click(function() {
    $('#screen1').hide();
    $('#screen2').show();
  });
  // When radio for random clicked, do not prompt for room name
  $('#roomType1').click(function() {
    $('#roomcode').hide();
    $('#next2').show();
  });
  // When radio for existing clicked, prompt for room name
  $('#roomType2').click(function() {
    $('#roomcode').show();
    checkRoomCode();
  });
  // When roomcode typed, show the next button
  $('#roomcode').on('input', function() {
    checkRoomCode();
  });
  // When they click Go!, hide this whole form
  $('#next2').click(function() {
    $('#wrapper').hide();
    startGame();
  });
});

function checkRoomCode() {
  var roomcode = $('#txtRoomCode').val();
  // As long as something is typed in the box, show next button
  if (roomcode.length > 0) {
    $('#next2').show();
  } else {
    $('#next2').hide();
  }
}

function startGame() {
  var canvas = $('canvas#gameboard')[0];
  canvas.width = window.innerWidth;
  canvas.height = window.innerHeight;
  var ctx = canvas.getContext('2d');
  ctx.fillStyle = 'lightblue';
  ctx.fillRect(0,0,canvas.width,canvas.height);
  ctx.fillStyle = 'black';
  ctx.font = '24pt serif';
  ctx.fillText('GameBoard here', 100, 100);
}
