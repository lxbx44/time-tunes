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
    if (hTxt) {
        hTxt.value = val.toString();
    }
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
    if (hTxt) {
        hTxt.value = val.toString();
    }
});

// minutes
let mUp: HTMLButtonElement | null = document.querySelector('#minutes-up');
let mDown: HTMLButtonElement | null = document.querySelector('#minutes-down');
let mTxt: HTMLInputElement | null = document.querySelector('#minutes-input');

mUp?.addEventListener('click', () => {
    let val: number | undefined = mTxt?.valueAsNumber;
    if (val === undefined) {
        val = 1;
    } else if (val === 59) {
        let new_h_value: number | undefined = hTxt?.valueAsNumber;
        if (new_h_value === undefined) {
            new_h_value = 0;
        }
        new_h_value += 1;
        val = 0;
        if (hTxt) {
            hTxt.value = new_h_value.toString();
        }
    } else {
        val += 1;
    }
    if (mTxt) {
        mTxt.value = val.toString();
    }
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
    if (mTxt) {
        mTxt.value = val.toString();
    }
});

// seconds
let sUp: HTMLButtonElement | null = document.querySelector('#seconds-up');
let sDown: HTMLButtonElement | null = document.querySelector('#seconds-down');
let sTxt: HTMLInputElement | null = document.querySelector('#seconds-input');

sUp?.addEventListener('click', () => {
    let val: number | undefined = sTxt?.valueAsNumber;

    if (val === undefined) {
        val = 1;
    } else if (val === 59) {
        let new_m_value: number | undefined = mTxt?.valueAsNumber;

        if (new_m_value === undefined) {
            new_m_value = 1;
        } else if (new_m_value === 59) {
            let new_h_value: number | undefined = hTxt?.valueAsNumber;
            if (new_h_value === undefined) {
                new_h_value = 0;
            }
            new_h_value += 1;
            new_m_value = 0;
            if (hTxt) {
                hTxt.value = new_h_value.toString();
            }
            val = 0;
        } else {
            new_m_value += 1;
            val = 0;
        }
        if (mTxt) {
            mTxt.value = new_m_value.toString();
        }
        val = 0;
    } else {
        val += 1;
    }
    if (sTxt) {
        sTxt.value = val.toString();
    }
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
    if (sTxt) {
        sTxt.value = val.toString();
    }
});

function preventNonNumericInput(event: Event) {
    let inputElement = event.target as HTMLInputElement;
    inputElement.value = inputElement.value.replace(/\D/g, '0');
}
hTxt?.addEventListener('input', preventNonNumericInput);
mTxt?.addEventListener('input', preventNonNumericInput);
sTxt?.addEventListener('input', preventNonNumericInput);


function no_max_59(event: Event) {
    let inputElement = event.target as HTMLInputElement;
    if (inputElement.valueAsNumber > 59) {
        inputElement.value = "59";
    }
}

mTxt?.addEventListener('input', no_max_59);
sTxt?.addEventListener('input', no_max_59);

// Open choose folder
let folderButton: HTMLButtonElement | null = document.querySelector('#choose-folder');

let text1: string = "Choose folder";
let text2: string;
let musicFolder: string | string[] | null;

folderButton?.addEventListener('click', async () => {
    musicFolder = await open({
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



// THINGIES TO PASS TO RUST

let depthFactor: HTMLElement | null = document.querySelector('#depth-factor-num');

// Settings page

let settingsBtn: HTMLElement | null = document.querySelector('#open-settings');

let settingsContent: HTMLElement | null = document.querySelector('.settings');
let maincontent: HTMLElement | null = document.querySelector('.container');

settingsBtn?.addEventListener('click', () => {
    if (maincontent) {
        maincontent.style.display = 'none';
    }

    let depthFactorSlider: HTMLInputElement | null = document.querySelector('#depth-factor');

    if (depthFactor && depthFactorSlider) {
        depthFactor.textContent = depthFactorSlider.value;
    };


    if (settingsContent) {
        settingsContent.style.display = 'flex'; 
    }
    
    depthFactorSlider?.addEventListener('input', () => {
        if (depthFactor && depthFactorSlider) {
            depthFactor.textContent = depthFactorSlider.value;
        };
    });


});




interface Time {
    hours: number,
    minutes: number,
    seconds: number,
    to_seconds(): number,
}

class Time implements Time {
    constructor(public hours: number, public minutes: number, public seconds: number) {}

    to_seconds(): number {
        return this.hours * 60 * 60 + this.minutes * 60 + this.seconds;
    }
}

function formatTime(seconds: number): string {
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    const remainingSeconds = seconds % 60;

    const timeArray: string[] = [];

    if (hours > 0) {
        timeArray.push(`${hours}h`);
    }

    if (minutes > 0) {
        timeArray.push(`${minutes}m`);
    }

    if (remainingSeconds > 0) {
        timeArray.push(`${remainingSeconds}s`);
    }

    return timeArray.join(' ');
}

let form: HTMLFormElement | null = document.querySelector('#main-form');

form?.addEventListener('submit', (event: Event) => {
    event.preventDefault();

    if (hTxt?.valueAsNumber === undefined) {
        return;
    }
    if (mTxt?.valueAsNumber === undefined) {
        return;
    }
    if (sTxt?.valueAsNumber === undefined) {
        return;
    }

    let duration: Time = new Time(
        hTxt.valueAsNumber,
        mTxt.valueAsNumber,
        sTxt.valueAsNumber
    );

    let folderPath: string | string[] | null = musicFolder;

    if (folderPath === null || folderPath === undefined) {
        return;
    }

    let loader: HTMLElement | null = document.querySelector('#loader');

    if (maincontent) {
        maincontent.style.display = 'none';
    }
    if (loader) {
        loader.style.display = 'grid';
    }

    setTimeout(() => {
        invoke('get_playlist', {
            time: duration.to_seconds(),
            path: folderPath,
        }).then((s: [Array<[string, string]>, number] | unknown) => {
            let [songs_path, total_time] = s as [string[], number];

            if (loader) {
                loader.style.display = 'none';
            }

            let songsDiv: HTMLDivElement | null = document.querySelector('#putsongshere');
            let songsDivParent: HTMLDivElement | null = document.querySelector('.confirm');
            let timespan: HTMLSpanElement | null = document.querySelector('#changetotaltime');
            let timespanreal: HTMLSpanElement | null = document.querySelector('#changerealtime');

            if (timespan && timespanreal) {
                timespan.textContent = formatTime(total_time);
                timespanreal.textContent = formatTime(duration.to_seconds());
            }

            let n: number = 1;
            songs_path.forEach((song: string) => {
                let p = document.createElement('p');
                p.textContent = `${n}. ${song.split('/').slice(-1)[0]}`;
                songsDiv?.appendChild(p);
                n++;
            });

            if (songsDivParent) {
                songsDivParent.style.display = 'grid';
            }

            let cancelBtn: HTMLButtonElement | null = document.querySelector('#confirm-stop');
            let continueBtn: HTMLButtonElement | null = document.querySelector('#confirm-ok');

            cancelBtn?.addEventListener('click', () => {
                window.location.reload();
            });

            continueBtn?.addEventListener('click', () => {
                if (songsDivParent) {
                    songsDivParent.style.display = 'none';
                }

                songs_path.forEach((song: string) => {
                    invoke('get_metadata', {path: song})
                        .then((metadata) => {
                            let [title, artist, album, picture, mimetype, total_time] = metadata as [string, string, string, Uint8Array | null, string, number];
                            let base64Image = picture ? btoa(String.fromCharCode.apply(null, Array.from(picture))) : '';

                            let img: HTMLImageElement | null = document.querySelector('#d-album');
                            if (img) {
                                if (picture === null) {
                                    img.src = 'src/assets/default_album_cover.svg';
                                } else {
                                    img.src = `data:${mimetype};base64,${base64Image}`;
                                }
                            }

                            let hTitle: HTMLElement | null = document.querySelector('#d-title');
                            if (hTitle) {
                                hTitle.textContent = title;
                            }

                            let hArtist: HTMLElement | null = document.querySelector('#d-artist');
                            if (hArtist) {
                                hArtist.textContent = artist;
                            }

                            let hAlbum: HTMLElement | null = document.querySelector('#d-album-name');
                            if (hAlbum) {
                                hAlbum.textContent = album;
                            }
    
                            let dTotal_time: HTMLElement | null = document.querySelector('#d-total-time');
                            if (dTotal_time) {
                                function formatToTimeDots(t: number): string {
                                    const hours = Math.floor(t / 3600);
                                    const minutes = Math.floor((t % 3600) / 60);
                                    const remainingSeconds = t % 60;

                                    const timeArray: string[] = [];

                                    if (hours > 0) {
                                        timeArray.push(`${hours}`);
                                    }

                                    if (minutes > 0) {
                                        timeArray.push(`${minutes}`);
                                    }

                                    if (remainingSeconds > 0) {
                                        timeArray.push(`${remainingSeconds}`);
                                    }

                                    return timeArray.join(':');
                                }

                                dTotal_time.textContent = formatToTimeDots(total_time);
                            }


                            let display: HTMLDivElement | null = document.querySelector('.display');
                            if (display) {
                                display.style.display = 'grid';
                            }
                        })
                });

            });
            
        });
    }, 500);
});
