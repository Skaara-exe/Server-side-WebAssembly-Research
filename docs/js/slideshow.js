pdfjsLib.GlobalWorkerOptions.workerSrc =
  'https://cdnjs.cloudflare.com/ajax/libs/pdf.js/3.11.174/pdf.worker.min.js';

const canvas    = document.getElementById('slide-canvas');
const ctx       = canvas.getContext('2d');
const btnPrev   = document.getElementById('btn-prev');
const btnNext   = document.getElementById('btn-next');
const counter   = document.getElementById('counter');
const loading   = document.getElementById('loading-msg');
const container = document.getElementById('canvas-container');
const controls  = document.getElementById('controls');

// Read PDF path from the canvas container's data attribute
const PDF_URL = container.dataset.pdf;

let pdfDoc      = null;
let currentPage = 1;
let rendering   = false;

async function renderPage(num) {
  if (rendering) return;
  rendering = true;

  const page     = await pdfDoc.getPage(num);
  const viewport = page.getViewport({ scale: 2 });

  canvas.width  = viewport.width;
  canvas.height = viewport.height;

  await page.render({ canvasContext: ctx, viewport }).promise;

  counter.textContent = `${num} / ${pdfDoc.numPages}`;
  btnPrev.disabled    = num <= 1;
  btnNext.disabled    = num >= pdfDoc.numPages;
  rendering           = false;
}

pdfjsLib.getDocument(PDF_URL).promise
  .then(pdf => {
    pdfDoc = pdf;
    loading.style.display   = 'none';
    container.style.display = 'flex';
    controls.style.display  = 'flex';
    renderPage(currentPage);
  })
  .catch(() => {
    loading.textContent = '⚠ Could not load PDF. Check the file path.';
  });

btnPrev.addEventListener('click', () => {
  if (currentPage > 1) renderPage(--currentPage);
});

btnNext.addEventListener('click', () => {
  if (currentPage < pdfDoc.numPages) renderPage(++currentPage);
});

document.addEventListener('keydown', e => {
  if (e.key === 'ArrowRight' || e.key === 'ArrowDown') btnNext.click();
  if (e.key === 'ArrowLeft'  || e.key === 'ArrowUp')   btnPrev.click();
});