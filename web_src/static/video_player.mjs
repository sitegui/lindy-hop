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
<div id="help-modal" style="display: none">
  <h1>Aide</h1>
  <h2>Boutons</h2>
  <p><img src="/static/video_player/add_favorite.svg"> Ajouter un point de repère</p>
  <p><img src="/static/video_player/close.svg"> Fermer</p>
  <p><img src="/static/video_player/pause.svg"> Pause <img src="/static/video_player/play.svg"> Lecture</p>
  <h2>Gestes</h2>
  <p><img src="/static/video_player/tap.svg"> Tape pour pause ou lecture</p>
  <p><img src="/static/video_player/double_tag.svg"> Tape deux fois pour ajouter un point de repère</p>
  <p><img src="/static/video_player/long_press.svg"> Maintiens le doigt pour ralentir</p>
  <p><img src="/static/video_player/swipe_left.svg"> Glisse à gauche pour aller au repère précédent</p>
  <p><img src="/static/video_player/swipe_right.svg"> Glisse à droite pour aller au repère suivant</p>
  <div id="help-close"><img src="/static/video_player/close.svg"></div>
</div>
<style>
* {
  /* Mobile screens may have rounded corners */
  --screen-margin: 10px;
  --control-width: 50px;
  --control-height: 40px;
  --timeline-bar-height: 5px;
  --timeline-knob-size: 15px;
  --favorite-size: 20px;
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
  transition: left 0.05s linear;
}

#favorites {
  position: absolute;
  top: calc((var(--control-height) - var(--timeline-knob-size)) / 2 - var(--timeline-knob-size));
  left: calc(-1/2 * (var(--favorite-size) - var(--timeline-knob-size)));
  right: calc(var(--screen-margin) + (var(--favorite-size) + var(--timeline-knob-size)) / 2);
}

.favorite {
  position: absolute;
  width: var(--favorite-size);
  height: var(--favorite-size);
}

#help-modal {
  position: absolute;
  left: 0;
  right: 0;
  top: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.8);
  color: #fff;
  padding: 10px;
}

#help-modal img {
  vertical-align: middle;
}

#help-modal p {
  margin: 5px 0;
}

#help-close {
  position: absolute;
  right: 10px;
  top: 10px;
  cursor: pointer;
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
let favorites = []
const favoritesEl = shadowRoot.getElementById('favorites')
const addFavoriteEl = shadowRoot.getElementById('add-favorite')
addFavoriteEl.addEventListener('click', () => {
  if (!Number.isFinite(videoEl.currentTime)) {
    return
  }

  favorites.push(videoEl.currentTime)
  renderFavorites()

  // If the new favorite overlaps with any other, they both are removed
  const favoriteEls = Array.from(favoritesEl.children)
  const lastFavoriteRect = favoriteEls[favoriteEls.length - 1].getBoundingClientRect()
  const otherFavoriteEls = favoriteEls.slice(0, -1)
  for (let i = 0; i < otherFavoriteEls.length; i++) {
    const favoriteEl = otherFavoriteEls[i]
    const favoriteRect = favoriteEl.getBoundingClientRect()
    if (favoriteRect.left < lastFavoriteRect.right && favoriteRect.right > lastFavoriteRect.left) {
      favorites.pop()
      favorites.splice(i, 1)
      renderFavorites()
      break
    }
  }

  saveFavorites()
})

function renderFavorites() {
  favoritesEl.innerHTML = ''

  if (!Number.isFinite(videoEl.duration)) {
    videoEl.addEventListener('durationchange', renderFavorites, {once: true})
    return
  }

  for (const favorite of favorites) {
    const favoriteEl = document.createElement('img')
    favoriteEl.classList.add('favorite')
    favoriteEl.dataset.time = favorite
    favoriteEl.src = '/static/video_player/favorite.svg'
    favoriteEl.style.left = `${favorite / videoEl.duration * 100}%`
    favoritesEl.appendChild(favoriteEl)
  }
}

function loadFavorites() {
  const src = videoEl.src
  const favoritesStr = localStorage.getItem(`video_player:favorites:${src}`)
  if (favoritesStr) {
    favorites = JSON.parse(favoritesStr)
  } else {
    favorites = []
  }
}

function saveFavorites() {
  const src = videoEl.src
  localStorage.setItem(`video_player:favorites:${src}`, JSON.stringify(favorites))
}

// Timeline movement
const knobEl = shadowRoot.getElementById('timeline-knob')
const lastTimeBySource = new Map()
let isSeeking = false

function updateKnobPosition() {
  if (!isSeeking) {
    if (Number.isFinite(videoEl.currentTime) && Number.isFinite(videoEl.duration)) {
      lastTimeBySource.set(videoEl.src, videoEl.currentTime)
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
  if (!Number.isFinite(videoEl.duration)) {
    return
  }

  // Snap to favorites
  const favoriteEls = shadowRoot.querySelectorAll('.favorite')
  for (const favoriteEl of favoriteEls) {
    const favoriteRect = favoriteEl.getBoundingClientRect()
    if (x >= favoriteRect.left && x <= favoriteRect.right) {
      videoEl.currentTime = Number(favoriteEl.dataset.time)
      const ratio = videoEl.currentTime / videoEl.duration
      knobEl.style.left = `${ratio * 100}%`
      return
    }
  }

  const barRect = timelineBarEl.getBoundingClientRect()
  const ratio = Math.min(1.0, Math.max(0.0, (x - barRect.left) / barRect.width))


  videoEl.currentTime = ratio * videoEl.duration
  knobEl.style.left = `${ratio * 100}%`
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

// Help
const helpEl = shadowRoot.getElementById('help')
const helpModalEl = shadowRoot.getElementById('help-modal')
helpEl.addEventListener('click', () => {
  helpModalEl.style.display = 'block'
})
helpModalEl.addEventListener('click', () => {
  helpModalEl.style.display = 'none'
})


const VideoPlayer = {
  play(src) {
    const startTime = lastTimeBySource.get(src) || 0
    pageEl.style.display = 'block'
    videoEl.src = src
    videoEl.currentTime = startTime
    videoEl.play()

    loadFavorites()
    renderFavorites()
    pageEl.requestFullscreen({navigationUI: 'show'})
  }
}

window.VideoPlayer = VideoPlayer