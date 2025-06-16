
if (!window.WebAssembly) {
    alert("WebAssembly is not supported in your browser. Please use a modern browser.");
}

const cipherModeInput = document.getElementById('cipherModeInput');
const steganoModeInput = document.getElementById('steganoModeInput');
const msgInputContainer = document.getElementById('msgInput-container');
const msgInput = document.getElementById('msgInput');
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
const imTypeContainer = document.getElementById('imType-container');
const downloadBtnContainer = document.getElementById('downloadBtn-container');
const outputContainer = document.getElementById('output-container');
const footer = document.getElementById('footer');

const worker = new Worker('./worker.js', { type: 'module' });

function onModeChange(mode) {
    if (mode === 'crypto') {
        msgInputContainer.style.display = 'none';
        imTypeSelect.disabled = false;
        imTypeContainer.style.display = 'block';
    } else if (mode === 'stegano') {
        msgInputContainer.style.display = 'flex';
        imTypeSelect.value = 'png';
        imTypeSelect.dispatchEvent(new Event('change'));
        imTypeSelect.disabled = true;
        imTypeContainer.style.display = 'none';
    }
    const urlParams = new URLSearchParams(window.location.search);
    // remove existing mode and secret params
    urlParams.delete('mode');
    urlParams.delete('m');
    urlParams.set('m', mode);
    window.history.replaceState({}, '', `${window.location.pathname}?${urlParams.toString()}`);
}

// handle initial mode setup
{
    const urlParams = new URLSearchParams(window.location.search);
    const secretFromUrl = urlParams.get('secret') || urlParams.get('s');
    if (secretFromUrl) {
        secretInput.value = decodeURIComponent(secretFromUrl);
    }
    const modeFromUrl = urlParams.get('mode') || urlParams.get('m');
    switch (modeFromUrl) {
        case 'stega':
        case 'stegano':
        case 'steganography':
            cipherModeInput.checked = false;
            steganoModeInput.checked = true;
            onModeChange('stegano');
            break;
        case 'cipher':
        case 'crypto':
        case 'cryptography':
        default:
            cipherModeInput.checked = true;
            steganoModeInput.checked = false;
            onModeChange('crypto');
            break;
    }
}

function getMode() {
    if (cipherModeInput.checked) {
        return 'crypto';
    } else if (steganoModeInput.checked) {
        return 'stegano';
    }
    throw new Error("No mode selected");
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
        if (getMode() === 'crypto') {
            showError("Secret cannot be empty");
            throw new Error("Secret cannot be empty");
        }
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
    switch (getMode()) {
        case 'crypto':
            worker.postMessage({
                type: 'encode', 
                buffer: inputBlob, 
                secret: secretInput.value, 
                maxSide: limitMaxSideInput.checked ? parseInt(maxSideInput.value) : -1, 
                outputAs: imTypeSelect.value
            });
            break;
        case 'stegano':
            worker.postMessage({
                type: 'stega_encode', 
                buffer: inputBlob, 
                secret: secretInput.value, 
                message: msgInput.value,
                maxSide: limitMaxSideInput.checked ? parseInt(maxSideInput.value) : -1, 
            });
            break;
        default:
            throw new Error("Unknown mode");
    }
}

async function decode_image() {
    checkInputRaise();
    resetOutput();

    const inputBlob = await getInputBlob();
    hintLabel.textContent = `Decoding...`;

    ensureOutput();
    switch (getMode()) {
        case 'crypto':
            worker.postMessage({
                type: 'decode',  
                buffer: inputBlob, 
                secret: secretInput.value, 
                maxSide: limitMaxSideInput.checked ? parseInt(maxSideInput.value) : -1, 
                outputAs: imTypeSelect.value
            });
            break;
        case 'stegano':
            worker.postMessage({
                type: 'stega_decode',  
                buffer: inputBlob, 
                secret: secretInput.value, 
                maxSide: limitMaxSideInput.checked ? parseInt(maxSideInput.value) : -1, 
            });
            break;
        default:
            throw new Error("Unknown mode");
    }
}

worker.onmessage = async (event) => {
    if (event.data.error) {
        await showError(`Error: ${event.data.error}`);
        console.error(event.data.error);
        hintLabel.textContent = '';
        return;
    }
    if (event.data.buffer) {
        await showImage(
            event.data.type,
            event.data.buffer, 
            event.data.format
        );
    }
    if (event.data.message) {
        resetOutput();
        ensureOutput();
        hintLabel.textContent = '';
        const messageElem = document.createElement('pre');
        messageElem.textContent = event.data.message;
        outputDiv.appendChild(messageElem);
        messageElem.style.whiteSpace = 'pre-wrap';
        downloadBtnContainer.style.display = 'none';
    }
};

// Event listeners for inputs
{
    encodeButton.addEventListener('click', encode_image);
    decodeButton.addEventListener('click', decode_image);

    steganoModeInput.addEventListener('change', () => {
        if (steganoModeInput.checked) {
            onModeChange('stegano');
        }
    })
    cipherModeInput.addEventListener('change', () => {
        if (cipherModeInput.checked) {
            onModeChange('crypto');
        }
    })

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
