const pageEl = document.createElement('div')
pageEl.style.display = 'none'

const shadowRoot = pageEl.attachShadow({mode: 'open'})
shadowRoot.innerHTML = `
<video id="video"></video>
<div id="controls">
    <div id="controls-padding">
      <div id="help"><img class="icon-button" src="/static/help.svg"></div>
      <div id="add-favorite"><img class="icon-button" src="/static/add_favorite.svg"></div>
      <div id="close"><img class="icon-button" src="/static/close.svg"></div>
      <div id="pause" class="play-pause"><img class="icon-button" src="/static/pause.svg"></div>
      <div id="play" class="play-pause"><img class="icon-button" src="/static/play.svg"></div>
      <div id="timeline">
          <div id="timeline-bar"></div>
          <div id="timeline-knob-rail">
            <div id="timeline-knob"></div>
          </div>
      </div>
  </div>
</div>
<style>
* {
    /* Mobile screens may have rounded corners */
    --screen-margin: 10px;
    --control-width: 50px;
    --control-height: 40px;
    --timeline-bar-height: 5px;
    --timeline-knob-size: 15px;
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}

#video {
    width: 100%;
    height: 100%;
}

#controls {
    position: absolute;
    bottom: var(--screen-margin);
    left: 0;
    height: var(--control-height);
    width: 100%;
    background-color: #222;
}

#controls-padding {
    position: absolute;
    bottom: 0;
    left: var(--screen-margin);
    right: var(--screen-margin);
}

#help, #add-favorite, #close, .play-pause {
    width: var(--control-width);
    height: var(--control-height);
    position: absolute;
    background-color: #222;
    padding: 5px;
    cursor: pointer;
}

#help {
    left: 0;
    bottom: var(--control-height);
}

#add-favorite {
    left: var(--control-width);
    bottom: var(--control-height);
}

#close {
    left: 0;
    bottom: 0;
}

.play-pause {
    left: var(--control-width);
    bottom: 0;
}

#timeline {
    width: calc(100% - 2 * var(--control-width));
    height: var(--control-height);
    position: absolute;
    bottom: 0;
    right: 0;
}

.icon-button {
    width: 100%;
    height: 100%;
}

#timeline-bar {
    position: absolute;
    top: calc((var(--control-height) - var(--timeline-bar-height)) / 2);
    height: var(--timeline-bar-height);
    left: calc(var(--timeline-knob-size) / 2);
    width: calc(100% - var(--timeline-knob-size));
    background-color: #ccc;
}

#timeline-knob-rail {
    position: absolute;
    left: 0;
    right: var(--timeline-knob-size);
    top: calc((var(--control-height) - var(--timeline-knob-size)) / 2);
}

#timeline-knob {
    position: absolute;
    border-radius: 100%;
    background-color: #fff;
    width: var(--timeline-knob-size);
    height: var(--timeline-knob-size);
    cursor: pointer;
    left: 0;
    transition: left 0.1s linear;
}
</style>`

document.body.appendChild(pageEl)

const videoEl = shadowRoot.getElementById('video')

for (const event of ['resize', 'abort', 'canplay', 'canplaythrough', 'durationchange', 'emptied', 'encrypted', 'ended', 'error', 'loadeddata', 'loadedmetadata', 'loadstart', 'pause', 'playing', 'progress', 'ratechange', 'seeked', 'seeking', 'stalled', 'suspend', 'timeupdate', 'volumechange', 'waiting', 'waitingforkey']) {
  videoEl.addEventListener(event, () => {
    console.log(event)
  })
}

const playEl = shadowRoot.getElementById('play')
const pauseEl = shadowRoot.getElementById('pause')
playEl.addEventListener('click', () => {
  videoEl.play()
})
pauseEl.addEventListener('click', () => {
  videoEl.pause()
})
videoEl.addEventListener('play', () => {
  playEl.style.display = 'none'
  pauseEl.style.display = 'block'
})
videoEl.addEventListener('pause', () => {
  pauseEl.style.display = 'none'
  playEl.style.display = 'block'
})

const knobEl = shadowRoot.getElementById('timeline-knob')
videoEl.addEventListener('timeupdate', () => {
  knobEl.style.left = `${videoEl.currentTime / videoEl.duration * 100}%`
})

const VideoPlayer = {
  play(src) {
    pageEl.style.display = 'block'
    videoEl.src = src
    videoEl.play()

    pageEl.requestFullscreen({navigationUI: 'show'})
  }

}

window.VideoPlayer = VideoPlayer