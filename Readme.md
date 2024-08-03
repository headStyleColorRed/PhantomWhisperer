# Phantom Pulse: The Whisper in the Noise

<p align="center">
<img src="resources/square_wave.jpeg" alt="Phantom Pulse Logo" width="40%" height="auto" style="display: block; margin: auto;">
</p>

## 🌟 What's This Sorcery?

Phantom Pulse is a Rust-powered web application that turns your boring old text into secret audio whispers and back again! It's like having a digital secret agent in your browser. Using the arcane arts of frequency-shift keying (FSK), we transform your messages into WAV audio files that sound like robot dolphins having a chat.

## 🎭 Features That'll Blow Your Mind

- 🔊 Turn your texts into mysterious audio files
- 👂 Eavesdrop on those audio files and reveal their secrets
- 🌐 Web interface so slick, it makes spies jealous
- 🚀 API endpoints for when you're feeling extra techy

## 🏗️ Building Your Secret Lair

1. Make sure you've got Rust and Cargo. No Rust? Get it from https://www.rust-lang.org/.
2. Clone this bad boy:
   ```
   git clone https://github.com/your-repo/phantom-pulse.git
   cd phantom-pulse
   ```
3. Fire it up:
   ```
   cargo run
   ```


## 🕵️ How to Be a Digital Spy

### Web Interface: For the Visually Inclined

<p align="center">
<img src="resources/website.png" alt="Phantom Pulse Web Interface" width="70%" height="auto" style="display: block; margin: auto;">
</p>

1. Point your browser to http://localhost:3030
2. Type in your super-secret message
3. Hit that "Encode" button and watch the magic happen
4. To decode, upload your mysterious WAV file and reveal its secrets

### API Endpoints: For the Command Line Ninjas

- Encode your message:
  ```
  curl -X POST -H "Content-Type: application/json" -d '{"message":"Your secret here"}' http://localhost:3030/encode
  ```
  You'll get back a WAV file that sounds like a dial-up modem having a seizure.

- Decode a WAV file:
  ```
  curl -X POST -F "file=@path/to/your/secret.wav" http://localhost:3030/decode
  ```
  The server will spill the beans in JSON format.

- Health check (because even spies need to stay healthy):
  ```
  curl http://localhost:3030/health
  ```
  If it says "Server is up and running", you're golden!

## 🔬 The Science Behind the Magic

- We use Rust, because we're fast and we don't crash (unlike certain other languages we won't mention)
- Text goes in, binary comes out, then audio frequencies take over
- We use Binary FSK (Frequency-Shift Keying) because it sounds cooler than saying "beep boop"
- 1000 Hz = 0, 2000 Hz = 1 (but don't tell anyone, it's a secret)
- Audio nerds: We're rocking 44.1 kHz sample rate and 16-bit depth
- To decode, we use a correlation-based algorithm that's basically a very picky ear

## 🚨 Legal Mumbo Jumbo

This software is for educational purposes only. If you use it to plan a heist or overthrow a government, that's on you. We're not responsible for any international incidents, time paradoxes, or alien invasions that may result from using Phantom Pulse.

## 🤝 Join the Secret Society

Want to make Phantom Pulse even more phantom-y? Pull requests welcome! Just remember the first rule of Phantom Pulse: You don't talk about Phantom Pulse.

## 📜 License

MIT, because we're cool like that.

Now go forth and communicate in secret, you magnificent digital spy, you!
