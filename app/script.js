
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
const downloadBtnContainer = document.getElementById('downloadBtnContainer');
const downloadBtn = document.getElementById('downloadBtn');
const errorDiv = document.getElementById('error');
const imTypeSelect = document.getElementById('imTypeSelect');
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

function checkInput() {
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
    errorDiv.textContent = msg;
    errorDiv.style.display = 'block';
    console.error(msg);
}

function resetOutput() {
    inputImgDiv.innerHTML = '';
    outputDiv.innerHTML = '';
    errorDiv.textContent = '';
    errorDiv.style.display = 'none';
    downloadBtnContainer.style.display = 'none';
    hintLabel.textContent = '';
}

async function showImage(imBlob) {
    resetOutput();
    hintLabel.textContent = '';
    const blob = new Blob([imBlob], { type: 'image/png' });
    const url = URL.createObjectURL(blob);

    const imgElem = document.createElement('img');
    imgElem.src = url;
    imgElem.alt = 'Image';
    outputDiv.appendChild(imgElem);

    downloadBtnContainer.style.display = 'block';
    downloadBtn.onclick = () => {
        const a = document.createElement('a');
        a.href = url;
        const timeName = new Date().toISOString().replace(/[:.]/g, '-');
        a.download = `img-${timeName}.${imTypeSelect.value}`;
        document.body.appendChild(a);
        a.click();
        document.body.removeChild(a);
    };
}

async function encode_image() {
    checkInput();
    resetOutput();
    const inputBlob = await getInputBlob();
    hintLabel.textContent = `Encoding...`;
    worker.postMessage({
        type: 'encode', 
        buffer: inputBlob, 
        secret: secretInput.value, 
        maxSide: limitMaxSideInput.checked ? parseInt(maxSideInput.value) : -1, 
        outputAs: imTypeSelect.value
        });
}

async function decode_image() {
    checkInput();
    resetOutput();
    const inputBlob = await getInputBlob();
    hintLabel.textContent = `Decoding...`;
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
        return;
    }
    await showImage(event.data.buffer);
};

encodeButton.addEventListener('click', encode_image);
decodeButton.addEventListener('click', decode_image);
imTypeSelect.addEventListener('change', () => {
    const warningDiv = document.getElementById('warning');
    if (imTypeSelect.value === 'jpeg') {
        warningDiv.style.display = 'block';
    }
    else {
        warningDiv.style.display = 'none';
    }
});
fileInput.addEventListener('change', ()=>{
    resetOutput();
    hintLabel.textContent += ' (Below is the selected image)';
    const file = fileInput.files[0];
    if (file) {
        const imgElem = document.createElement('img');
        imgElem.src = URL.createObjectURL(file);
        imgElem.alt = 'Selected Image';
        inputImgDiv.innerHTML = '';
        inputImgDiv.appendChild(imgElem);
    }
});

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
        if (taskId) { window.clearTimeout(taskId); }
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
