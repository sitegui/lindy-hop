const pageEl = document.createElement('div')
pageEl.style.display = 'none'

const shadowRoot = pageEl.attachShadow({mode: 'open'})
shadowRoot.innerHTML = `
<div id="video-wrapper">
  <div id="gesture-feedback" style="display: none">
    <img src="" id="gesture-feedback-img">
  </div>
  <video id="video"></video>
</div>
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

#video-wrapper, #video {
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

#gesture-feedback {
  position: absolute;
  left: 25%;
  width: 50%;
  top: 25%;
  height: 50%;
  opacity: 0.25;
  animation: gesture-feedback 0.5s linear;
}

#gesture-feedback > img {
  width: 100%;
  height: 100%;
}

@keyframes gesture-feedback {
  0% { opacity: 0; }
  50% { opacity: 1; }
  100% { opacity: 0; }
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
addFavoriteEl.addEventListener('click', addFavorite)

function addFavorite() {
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
}

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
  videoEl.pause()
  helpModalEl.style.display = 'block'
})
helpModalEl.addEventListener('click', () => {
  helpModalEl.style.display = 'none'
})

// Gesture detection
let gestureState = 'NONE'
let gestureStartTime = null
let gestureStartX = null
let gestureStartY = null
let gestureX = null
let gestureY = null
let gestureTimer = null
const videoWrapperEl = shadowRoot.getElementById('video-wrapper')
const maxTapDuration = 200
const maxTapDistance = 10
const minSwapDuration = 75
const maxSwapDuration = 750
const minSwipeXDistance = 75
const maxSwipeYRatio = 0.25
const maxDoubleTapWait = 250
const slowInitialWait = 500
const slowMaxInitialDistance = 10
videoEl.addEventListener('contextmenu', event => {
  if (gestureState !== 'NONE') {
    event.preventDefault()
  }
})
videoWrapperEl.addEventListener('pointerdown', event => {
  event.preventDefault()
  event.stopPropagation()
  gestureStartTime = Date.now()
  gestureStartX = event.clientX
  gestureStartY = event.clientY
  gestureX = event.clientX
  gestureY = event.clientY
  videoWrapperEl.setPointerCapture(event.pointerId)

  if (gestureState === 'NONE') {
    gestureState = 'DOWN'
  } else if (gestureState === 'ONE_TAP') {
    gestureState = 'DOWN_TWO'
  }

  if (gestureTimer) {
    clearTimeout(gestureTimer)
    gestureTimer = null
  }
  gestureTimer = setTimeout(() => {
    if (gestureState === 'DOWN' || gestureState === 'DOWN_TWO') {
      const distance = Math.hypot(gestureX - gestureStartX, gestureY - gestureStartY)
      if (distance < slowMaxInitialDistance) {
        gestureState = 'SLOW'
        applyStartSlow()
      }
    }
  }, slowInitialWait)
})
videoWrapperEl.addEventListener('pointermove', event => {
  gestureX = event.clientX
  gestureY = event.clientY
})
videoWrapperEl.addEventListener('pointerup', event => {
  const duration = Date.now() - gestureStartTime
  const dx = event.clientX - gestureStartX
  const dy = event.clientY - gestureStartY
  const distance = Math.hypot(dx, dy)

  if (gestureTimer) {
    clearTimeout(gestureTimer)
    gestureTimer = null
  }

  if (gestureState === 'DOWN') {
    if (duration < maxTapDuration && distance < maxTapDistance) {
      gestureState = 'ONE_TAP'
      gestureTimer = setTimeout(() => {
        if (gestureState === 'ONE_TAP') {
          gestureState = 'NONE'
          applyTap()
        }
      }, maxDoubleTapWait)
    } else if (duration > minSwapDuration && duration < maxSwapDuration && Math.abs(dx) > minSwipeXDistance && Math.abs(dy) < Math.abs(dx) * maxSwipeYRatio) {
      if (dx > 0) {
        applySwipe('RIGHT')
      } else {
        applySwipe('LEFT')
      }

      gestureState = 'NONE'
    } else {
      gestureState = 'NONE'
    }
  } else if (gestureState === 'DOWN_TWO') {
    gestureState = 'NONE'
    if (duration < maxTapDuration && distance < maxTapDistance) {
      applyDoubleTap()
    }
  } else if (gestureState === 'SLOW') {
    gestureState = 'NONE'
    applyEndSlow()
  } else {
    gestureState = 'NONE'
  }
})

// Gesture action
function applyTap() {
  if (!Number.isFinite(videoEl.currentTime)) {
    return
  }

  if (videoEl.paused) {
    videoEl.play()
    showGestureFeedback('play')
  } else {
    videoEl.pause()
    showGestureFeedback('pause')
  }
}

function applyDoubleTap() {
  if (!Number.isFinite(videoEl.currentTime)) {
    return
  }

  addFavorite()
  showGestureFeedback('add_favorite')
}

function applySwipe(direction) {
  const currentTime = videoEl.currentTime
  if (!Number.isFinite(currentTime)) {
    return
  }

  const candidates = favorites.filter(favorite => direction === 'RIGHT' ? favorite > currentTime : favorite < currentTime)
  if (direction === 'LEFT') {
    candidates.push(0)
  }
  if (candidates.length === 0) {
    return
  }

  candidates.sort((a, b) => a - b)

  let bestFavorite = undefined
  if (direction === 'RIGHT') {
    bestFavorite = candidates[0]
  } else {
    const lastCandidate = candidates[candidates.length - 1]
    if (candidates.length > 1 && currentTime - lastCandidate < 2) {
      // Allow double-swipe-left
      bestFavorite = candidates[candidates.length - 2]
    } else {
      bestFavorite = lastCandidate
    }
  }

  videoEl.currentTime = bestFavorite
  showGestureFeedback(direction === 'RIGHT' ? 'swipe_right' : 'swipe_left')
}

function applyStartSlow() {
  if (!Number.isFinite(videoEl.currentTime)) {
    return
  }
  videoEl.playbackRate = 0.5
  showGestureFeedback('long_press')
}

function applyEndSlow() {
  videoEl.playbackRate = 1
}

// Gesture feedback
const gestureFeedbackEl = shadowRoot.getElementById('gesture-feedback')
const gestureFeedbackImgEl = shadowRoot.getElementById('gesture-feedback-img')

function showGestureFeedback(name) {
  gestureFeedbackEl.style.display = 'none'
  gestureFeedbackImgEl.src = `/static/video_player/${name}.svg`

  setTimeout(() => {
    gestureFeedbackEl.style.display = ''
  }, 0)

  gestureFeedbackEl.addEventListener('animationend', () => {
    gestureFeedbackEl.style.display = 'none'
  }, {once: true})
}

export function play(src) {
  pageEl.style.display = 'block'
  videoEl.src = src
  videoEl.currentTime = lastTimeBySource.get(videoEl.src) || 0
  videoEl.play()

  loadFavorites()
  renderFavorites()
  pageEl.requestFullscreen({navigationUI: 'show'})
}
