const pageEl = document.createElement('div')
pageEl.style.display = 'none'

const shadowRoot = pageEl.attachShadow({mode: 'open'})
shadowRoot.innerHTML = `
<video id="video"></video>
<div id="controls">
  <div id="help"><img class="icon-button" src="/static/video_player/help.svg"></div>
  <div id="add-favorite"><img class="icon-button" src="/static/video_player/add_favorite.svg"></div>
  <div id="close"><img class="icon-button" src="/static/video_player/close.svg"></div>
  <div id="pause" class="play-pause"><img class="icon-button" src="/static/video_player/pause.svg"></div>
  <div id="play" class="play-pause"><img class="icon-button" src="/static/video_player/play.svg"></div>
  <div id="timeline">
    <div id="timeline-bar"></div>
    <div id="favorites"></div>
    <div id="timeline-knob-rail">
      <div id="timeline-knob"></div>
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
  --favorite-size: 15px;
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
  left: 0;
  width: 100%;
  bottom: var(--screen-margin);
  height: var(--control-height);
  background-color: #222;
}

#help, #add-favorite, #close, .play-pause {
  position: absolute;
  width: var(--control-width);
  height: var(--control-height);
  background-color: #222;
  padding: 5px;
  cursor: pointer;
}

#help {
  left: var(--screen-margin);
  bottom: var(--control-height);
}

#add-favorite {
  left: calc(var(--screen-margin) + var(--control-width));
  bottom: var(--control-height);
}

#close {
  left: var(--screen-margin);
  bottom: 0;
}

.play-pause {
  left: calc(var(--screen-margin) + var(--control-width));
  bottom: 0;
}

#timeline {
  position: absolute;
  left: calc(var(--screen-margin) + 2 * var(--control-width));
  right: 0;
  height: var(--control-height);
  bottom: 0;
}

.icon-button {
  width: 100%;
  height: 100%;
}

#timeline-bar {
  position: absolute;
  left: calc(var(--timeline-knob-size) / 2);
  width: calc(100% - var(--screen-margin) - var(--timeline-knob-size));
  top: calc((var(--control-height) - var(--timeline-bar-height)) / 2);
  height: var(--timeline-bar-height);
  background-color: #ccc;
}

#timeline-knob-rail {
  position: absolute;
  left: 0;
  right: calc(var(--screen-margin) + var(--timeline-knob-size));
  top: calc((var(--control-height) - var(--timeline-knob-size)) / 2);
}

#timeline-knob {
  position: absolute;
  left: 0;
  width: var(--timeline-knob-size);
  height: var(--timeline-knob-size);
  border-radius: 100%;
  background-color: #fff;
  cursor: pointer;
  transition: left 0.1s linear;
}

#favorites {
  position: absolute;
  top: 0;
  left: calc(-1/2 * (var(--favorite-size) - var(--timeline-knob-size)));
  right: calc(var(--screen-margin) + (var(--favorite-size) + var(--timeline-knob-size)) / 2);
}

.favorite {
  position: absolute;
  width: var(--favorite-size);
  height: var(--favorite-size);
}
</style>`

document.body.appendChild(pageEl)

const videoEl = shadowRoot.getElementById('video')

// Play/pause
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

// Favorites
const favorites = []
const addFavoriteEl = shadowRoot.getElementById('add-favorite')
addFavoriteEl.addEventListener('click', () => {
  favorites.push(videoEl.currentTime)
  updateFavorites()
})
const updateFavorites = () => {
  const favoritesEl = shadowRoot.getElementById('favorites')
  favoritesEl.innerHTML = ''
  for (const favorite of favorites) {
    const favoriteEl = document.createElement('img')
    favoriteEl.classList.add('favorite')
    favoriteEl.src = '/static/video_player/favorite.svg'
    favoriteEl.style.left = `${favorite / videoEl.duration * 100}%`
    favoritesEl.appendChild(favoriteEl)
  }
}

// Timeline movement
const knobEl = shadowRoot.getElementById('timeline-knob')
let isSeeking = false
function updateKnobPosition() {
  if (!isSeeking) {
    if (Number.isFinite(videoEl.currentTime) && Number.isFinite(videoEl.duration)) {
      knobEl.style.left = `${videoEl.currentTime / videoEl.duration * 100}%`
    } else {
      knobEl.style.left = '0'
    }
  }
}
videoEl.addEventListener('emptied', updateKnobPosition)
videoEl.addEventListener('timeupdate', updateKnobPosition)
videoEl.addEventListener('durationchange', updateKnobPosition)

// Seek in timeline
const timelineEl = shadowRoot.getElementById('timeline')
const timelineBarEl = shadowRoot.getElementById('timeline-bar')
function seekToPress(x) {
  const barRect = timelineBarEl.getBoundingClientRect()
  const ratio = Math.min(1.0, Math.max(0.0, (x - barRect.left) / barRect.width))

  if (Number.isFinite(videoEl.duration)) {
    videoEl.currentTime = ratio * videoEl.duration
    knobEl.style.left = `${ratio * 100}%`
  }
}
timelineEl.addEventListener('pointerdown', event => {
  if (!isSeeking) {
    isSeeking = true

    videoEl.pause()
    seekToPress(event.clientX)

    timelineEl.setPointerCapture(event.pointerId)
  }
})
timelineEl.addEventListener('pointermove', event => {
  if (isSeeking) {
    seekToPress(event.clientX)
  }
})
timelineEl.addEventListener('pointerup', event => {
  if (isSeeking) {
    isSeeking = false

    seekToPress(event.clientX)
    videoEl.play()
  }
})

// Close
const closeEl = shadowRoot.getElementById('close')
closeEl.addEventListener('click', () => {
  videoEl.pause()
  pageEl.style.display = 'none'

  if (document.fullscreenElement) {
    document.exitFullscreen()
  }
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