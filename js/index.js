import detect from '../crate/Cargo.toml';
import Mustache from 'mustache';

const BUFFER_SIZE = 65536;

let fileinput = document.querySelector('[name=file]');
let resultDiv = document.querySelector('#result');
let resultTemplate = `<div>Encoding: <b>{{ encoding }}</b></div>
<div>Confidence (0&ndash;1): <b>{{ confidence }}</b></div>
<div>Decoded (limited to ${BUFFER_SIZE} bytes):</div>
<textarea rows="24" readonly>{{ decoded }}</textarea>`;
Mustache.parse(resultTemplate);

document.querySelector('form').addEventListener('submit', (event) => {
  let reader = new FileReader();
  reader.onload = () => {
    let buffer = new Uint8Array(reader.result);
    let result = detect.detect_and_decode(buffer);
    resultDiv.innerHTML = Mustache.render(resultTemplate, {
      encoding: result.encoding(),
      confidence: result.confidence().toFixed(3),
      decoded: result.decoded(),
    });
  };
  reader.readAsArrayBuffer(fileinput.files[0].slice(0, BUFFER_SIZE));
  event.preventDefault();
})
