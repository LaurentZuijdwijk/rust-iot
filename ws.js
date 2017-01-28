var W3CWebSocket = require('websocket').w3cwebsocket;
 

for (let i = 10 - 1; i >= 0;) {
	
	let client = new W3CWebSocket('ws://192.168.0.9:3012/', 'echo-protocol');
	 
	client.onerror = function() {
	    console.log('Connection Error');
	};
	 
	client.onopen = function() {
	    console.log('WebSocket Client Connected');
	 
	    function sendNumber() {
	        if (client.readyState === client.OPEN) {
	            var number = Math.round(Math.random() * 0xFFFFFF);
	            client.send(number.toString());
	            setTimeout(sendNumber, 1000);
	        }
	    }
	    sendNumber();
	};
	 
	client.onclose = function() {
	    console.log('echo-protocol Client Closed');
	};
	 
	client.onmessage = function(e) {
	    if (typeof e.data === 'string') {
	        console.log("Received: '" + e.data + "'");
	    }
	};
}