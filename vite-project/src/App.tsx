import { Link, MetaProvider } from "@solidjs/meta";
import './App.css';
import viteLogo from './assets/vite.svg';
// @ts-ignore
import { connect, sendData } from './client';

function App() {
  const WEBTRANSPORT_PORT = 0 /* REPLACED_BY_RUST_SERVER */;

  return (
    <>
      <MetaProvider>
        <Link rel="icon" href={viteLogo} />
      </MetaProvider>

      <h1>WTransport Example</h1>

      <div>
        <h2>Establish WebTransport connection</h2>
        <div class="input-line">
          <label for="url">URL:</label>
          <input type="text" name="url" id="url" value={`https://localhost:${WEBTRANSPORT_PORT}/`} />
            <input type="button" id="connect" value="Connect" onclick={connect} />
            </div>
        </div>

        <div>
          <h2>Send data over WebTransport</h2>
          <form name="sending">
            <textarea name="data" id="data"></textarea>
            <div>
              <input type="radio" name="sendtype" value="datagram" id="datagram" checked />
                <label for="datagram">Send a datagram</label>
            </div>
            <div>
              <input type="radio" name="sendtype" value="unidi" id="unidi-stream" />
                <label for="unidi-stream">Open a unidirectional stream</label>
            </div>
            <div>
              <input type="radio" name="sendtype" value="bidi" id="bidi-stream" />
                <label for="bidi-stream">Open a bidirectional stream</label>
            </div>
            <input type="button" id="send" name="send" value="Send data" disabled onclick={sendData} />
          </form>
        </div>

        <div>
          <h2>Event log</h2>
          <ul id="event-log">
          </ul>
        </div>
    </>
  )
}

export default App
