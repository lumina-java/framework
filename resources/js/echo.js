class LuminaEcho {
    constructor(options) {
        this.host = options.host || `ws://${window.location.host}/lumina/echo`;
        this.subscriptions = new Map();
        this.connect();
    }

    connect() {
        this.socket = new WebSocket(this.host);

        this.socket.onopen = () => {
            console.log('✨ Lumina Echo connected');
            // Re-subscribe to existing channels on reconnect
            this.subscriptions.forEach((handlers, channel) => {
                this.send('subscribe', { channel });
            });
        };

        this.socket.onmessage = (event) => {
            const message = JSON.parse(event.data);
            const { channel, event: eventName, data } = message;

            if (this.subscriptions.has(channel)) {
                const handlers = this.subscriptions.get(channel);
                if (handlers.has(eventName)) {
                    handlers.get(eventName).forEach(handler => handler(data));
                }
            }
        };

        this.socket.onclose = () => {
            console.warn('⚡ Lumina Echo disconnected. Retrying in 3s...');
            setTimeout(() => this.connect(), 3000);
        };
    }

    channel(channelName) {
        if (!this.subscriptions.has(channelName)) {
            this.subscriptions.set(channelName, new Map());
            if (this.socket.readyState === WebSocket.OPEN) {
                this.send('subscribe', { channel: channelName });
            }
        }

        return {
            listen: (eventName, callback) => {
                const eventHandlers = this.subscriptions.get(channelName);
                if (!eventHandlers.has(eventName)) {
                    eventHandlers.set(eventName, []);
                }
                eventHandlers.get(eventName).push(callback);
                return this;
            }
        };
    }

    send(event, data) {
        this.socket.send(JSON.stringify({ event, ...data }));
    }
}

window.Echo = new LuminaEcho({});
