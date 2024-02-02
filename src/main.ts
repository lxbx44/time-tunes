import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";


// hours
let hUp: HTMLButtonElement | null = document.querySelector('#hours-up');
let hDown: HTMLButtonElement | null = document.querySelector('#hours-down');
let hTxt: HTMLInputElement | null = document.querySelector('#hours-input');

hUp?.addEventListener('click', () => {
    let val: number | undefined = hTxt?.valueAsNumber;
    if (val === undefined) {
        val = 1;
    } else {
        val += 1;
    }
    hTxt?.setAttribute('value', val.toString());
});

hDown?.addEventListener('click', () => {
    let val: number | undefined = hTxt?.valueAsNumber;
    if (val === undefined) {
        val = 0;
    } else if (val === 0) {
        val = 0;
    } else {
        val -= 1;
    }
    hTxt?.setAttribute('value', val.toString());
});


// minutes
let mUp: HTMLButtonElement | null = document.querySelector('#minutes-up');
let mDown: HTMLButtonElement | null = document.querySelector('#minutes-down');
let mTxt: HTMLInputElement | null = document.querySelector('#minutes-input');

mUp?.addEventListener('click', () => {
    let val: number | undefined = mTxt?.valueAsNumber;
    if (val === undefined) {
        val = 1;
    } else {
        val += 1;
    }
    mTxt?.setAttribute('value', val.toString());
});

mDown?.addEventListener('click', () => {
    let val: number | undefined = mTxt?.valueAsNumber;
    if (val === undefined) {
        val = 0;
    } else if (val === 0) {
        val = 0;
    } else {
        val -= 1;
    }
    mTxt?.setAttribute('value', val.toString());
});


// seconds
let sUp: HTMLButtonElement | null = document.querySelector('#seconds-up');
let sDown: HTMLButtonElement | null = document.querySelector('#seconds-down');
let sTxt: HTMLInputElement | null = document.querySelector('#seconds-input');

sUp?.addEventListener('click', () => {
    let val: number | undefined = sTxt?.valueAsNumber;
    if (val === undefined) {
        val = 1;
    } else {
        val += 1;
    }
    sTxt?.setAttribute('value', val.toString());
});

sDown?.addEventListener('click', () => {
    let val: number | undefined = sTxt?.valueAsNumber;
    if (val === undefined) {
        val = 0;
    } else if (val === 0) {
        val = 0;
    } else {
        val -= 1;
    }
    sTxt?.setAttribute('value', val.toString());
});



// Open choose folder

let folderButton: HTMLButtonElement | null = document.querySelector('#choose-folder');

let text1: string = "Choose folder";
let text2: string;

folderButton?.addEventListener('click', async () => {
    const musicFolder = await open({
        multiple: false,
        directory: true,
        filters: [{
            name: 'Music', 
            extensions: []
        }]
    });

    if (musicFolder === null) {
        console.error('error: no folder was selected');
    } else {
        if (folderButton) {
            folderButton.textContent = musicFolder.toString();
            text2 = musicFolder.toString();
        }
    }
});

folderButton?.addEventListener('mouseenter', () => {
    if (folderButton) {
        folderButton.textContent = text1;
    }
});

folderButton?.addEventListener('mouseleave', () => {
    if (folderButton) {
        if (text2) {
            folderButton.textContent = text2;
        } else {
            folderButton.textContent = text1;
        }
    }
});
