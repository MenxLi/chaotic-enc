
if (!window.WebAssembly) {
    alert("WebAssembly is not supported in your browser. Please use a modern browser.");
}

const secretInput = document.getElementById('secretInput');
const fileInput = document.getElementById('fileInput');
const limitMaxSideInput = document.getElementById('limitMaxSide');
const maxSideInput = document.getElementById('maxSide');
const inputImgDiv = document.getElementById('inputImg');
const outputDiv = document.getElementById('output');
const hintLabel = document.getElementById('hint');
const encodeButton = document.getElementById('encodeButton');
const decodeButton = document.getElementById('decodeButton');
const downloadBtn = document.getElementById('downloadBtn');
const errorDiv = document.getElementById('error');
const imTypeSelect = document.getElementById('imTypeSelect');
const downloadBtnContainer = document.getElementById('downloadBtn-container');
const outputContainer = document.getElementById('output-container');
const footer = document.getElementById('footer');

const worker = new Worker('./worker.js', { type: 'module' });

const urlParams = new URLSearchParams(window.location.search);
const secretFromUrl = urlParams.get('secret') || urlParams.get('s');
if (secretFromUrl) {
    secretInput.value = decodeURIComponent(secretFromUrl);
}

async function getInputBlob() {
    const file = fileInput.files[0];
    const arrayBuffer = await file.arrayBuffer();
    return new Uint8Array(arrayBuffer);
}

function checkInputRaise() {
    if (fileInput.files.length === 0) {
        showError("No image file selected");
        throw new Error("No image file selected");
    }
    if (secretInput.value.trim() === '') {
        showError("Secret cannot be empty");
        throw new Error("Secret cannot be empty");
    }
}

async function showError(msg) {
    ensureOutput();
    errorDiv.textContent = msg;
    errorDiv.style.display = 'block';
    console.error(msg);
}

function ensureOutput() {
    outputContainer.style.opacity = '1';
    footer.style.top = '0';
}

function resetOutput() {
    outputContainer.style.opacity = '0';
    inputImgDiv.innerHTML = '';
    outputDiv.innerHTML = '';
    errorDiv.textContent = '';
    errorDiv.style.display = 'none';
    downloadBtnContainer.style.display = 'none';
    hintLabel.textContent = '';
    footer.style.top = '-3rem';
}

async function showImage(type, imBlob, format) {
    resetOutput();
    ensureOutput();

    hintLabel.textContent = '';
    const blob = new Blob([imBlob], { type: `image/${format}` });
    const url = URL.createObjectURL(blob);

    const imgElem = document.createElement('img');
    imgElem.src = url;
    imgElem.alt = 'Image';
    outputDiv.appendChild(imgElem);

    downloadBtnContainer.style.display = 'block';
    downloadBtn.onclick = () => {
        const a = document.createElement('a');
        a.href = url;
        const uuid = crypto.randomUUID().slice(0, 8);
        a.download = `${type}-${uuid}.${format}`;
        document.body.appendChild(a);
        a.click();
        document.body.removeChild(a);
    };
}

async function encode_image() {
    checkInputRaise();
    resetOutput();

    const inputBlob = await getInputBlob();
    hintLabel.textContent = `Encoding...`;

    ensureOutput();
    worker.postMessage({
        type: 'encode', 
        buffer: inputBlob, 
        secret: secretInput.value, 
        maxSide: limitMaxSideInput.checked ? parseInt(maxSideInput.value) : -1, 
        outputAs: imTypeSelect.value
        });
}

async function decode_image() {
    checkInputRaise();
    resetOutput();

    const inputBlob = await getInputBlob();
    hintLabel.textContent = `Decoding...`;

    ensureOutput();
    worker.postMessage({
        type: 'decode',  
        buffer: inputBlob, 
        secret: secretInput.value, 
        maxSide: limitMaxSideInput.checked ? parseInt(maxSideInput.value) : -1, 
        outputAs: imTypeSelect.value
        });
}

worker.onmessage = async (event) => {
    if (event.data.error) {
        await showError(`Error: ${event.data.error}`);
        console.error(event.data.error);
        hintLabel.textContent = '';
        return;
    }
    await showImage(
        event.data.type,
        event.data.buffer, 
        event.data.format
    );
};

// Event listeners for inputs
{
    encodeButton.addEventListener('click', encode_image);
    decodeButton.addEventListener('click', decode_image);

    imTypeSelect.addEventListener('change', () => {
        if (imTypeSelect.value === 'jpeg') {
            encodeButton.disabled = true;
        }
        else {
            encodeButton.disabled = false;
        }
    });

    fileInput.addEventListener('change', ()=>{
        resetOutput();
        const file = fileInput.files[0];
        if (file) {
            ensureOutput();
            hintLabel.textContent = '(The selected image)';
            const imgElem = document.createElement('img');
            imgElem.src = URL.createObjectURL(file);
            imgElem.alt = 'Selected Image';
            inputImgDiv.innerHTML = '';
            inputImgDiv.appendChild(imgElem);
        }
    });
}

// handle drag and drop
{
    const dropZone = document.getElementById('drop-zone');
    let inDrag = false;
    let taskId = null;
    function hideDropZone() {
        if (inDrag) return;
        dropZone.style.display = 'none';
    }

    window.addEventListener('dragover', (event) => {
        event.preventDefault();
        dropZone.style.display = 'block';
        inDrag = true;
        if (taskId) {window.clearTimeout(taskId); }
        taskId = window.setTimeout(hideDropZone, 100);
    });
    
    window.addEventListener('dragmove', (event) => {
        event.preventDefault();
        dropZone.style.display = 'block';
        inDrag = true;
        if (taskId) { window.clearTimeout(taskId); }
        taskId = window.setTimeout(hideDropZone, 100);
    });

    window.addEventListener('dragleave', (event) => {
        event.preventDefault();
        inDrag = false;
        if (taskId) { window.clearTimeout(taskId); }
        taskId = window.setTimeout(hideDropZone, 100);
    });

    dropZone.addEventListener('drop', (event) => {
        event.preventDefault();
        if (event.dataTransfer.files.length > 0) {
            fileInput.files = event.dataTransfer.files;
            fileInput.dispatchEvent(new Event('change'));
        }
        inDrag = false;
        dropZone.style.display = 'none';
    });
}
