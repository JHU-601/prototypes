# Clueless Flask Prototype
from flask import Flask
from flask import render_template
from flask_socketio import SocketIO, emit

SERVER_ADDR = '0.0.0.0'
SERVER_PORT = 5000

app = Flask(__name__)
socketio = SocketIO(app)

@app.route('/')
def index():
    return render_template('index.html')

@app.route('/gameroom')
def gameroom():
    return render_template('gameroom.html')

@socketio.on('connect')
def handle_connection():
    print('Client connected')
    emit('my event', {'data': 'hello there young connection'})
    print('sent welcome message')
@socketio.on('my event')
def handle_my_event(json):
    print("From client:",json['data'])

if __name__ == '__main__':
    socketio.run(app, host=SERVER_ADDR, port=SERVER_PORT, debug=True)
