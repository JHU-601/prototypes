# Clueless Flask Prototype
from flask import Flask
from flask import render_template

SERVER_ADDR = '0.0.0.0'
SERVER_PORT = 5000

app = Flask(__name__)

@app.route('/')
def index():
    return render_template('index.html')

if __name__ == '__main__':
    app.run(host=SERVER_ADDR, port=SERVER_PORT)
