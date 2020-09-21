function initGameroom() {
  var socket = io.connect();
  socket.on('connect', function() {
    console.log('connected')
    socket.emit('my event', {data: 'iron board games are the best'});
  });
  socket.on('my event', function(json) {
    console.log('from server: ' + json.data);
  })
}

$(document).ready(function() {
  if (window.location.pathname.includes('gameroom')) {
    initGameroom();
  }
});
