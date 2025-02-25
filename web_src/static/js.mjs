window.playVideo = function (thumbnailEl) {
  const videoEl = thumbnailEl.parentElement.querySelector('video')
  const baseUrl = thumbnailEl.dataset.baseUrl
  const video = thumbnailEl.dataset.video

  if (video) {
    showVideo(videoEl, thumbnailEl, `${baseUrl}/${video}`)
  } else {
    const accessRule = thumbnailEl.dataset.accessRule
    const accessIv = thumbnailEl.dataset.accessIv
    const accessCiphertext = thumbnailEl.dataset.accessCiphertext
    const accessSalt = thumbnailEl.dataset.accessSalt
    const accessIterations = thumbnailEl.dataset.accessIterations

    const password = getPassword(accessRule)
    if (password) {
      decrypt(password, accessSalt, Number(accessIterations), accessIv, accessCiphertext).then(video => {
        showVideo(videoEl, thumbnailEl, `${baseUrl}/${video}`)
        savePassword(accessRule, password)
      }).catch(error => {
        alert("Code incorrect")
        console.error(error)
      })
    }
  }
}

window.stopAllOtherVideos = function (videoEl) {
  for (const anotherVideoEl of document.querySelectorAll('video')) {
    if (anotherVideoEl !== videoEl) {
      anotherVideoEl.pause()
    }
  }
}

window.copyShareLink = function (el) {
  const relative = el.dataset.shareLink
  const absolute = new URL(relative, window.location).toString()
  navigator.clipboard.writeText(absolute).catch(error => {
    console.error('failed to write to clipboard', error)
  }).then(() => {
    el.querySelector('.copied-feedback').style.display = ''
  })
}

function getPassword(rule) {
  const stored = localStorage.getItem(`password:${rule}`)
  if (stored) {
    return stored
  }

  return prompt(`Cette video fait partie de la collection ${rule} et est protégée. Merci d'entrer le code d'accès`)
}

function savePassword(rule, password) {
  localStorage.setItem(`password:${rule}`, password)
}

function showVideo(videoEl, thumbnailEl, video) {
  videoEl.style.display = ''
  videoEl.src = video
  thumbnailEl.style.display = 'none'
  videoEl.play()
}

async function decrypt(password, salt, iterations, iv, ciphertext) {
  function stringToBytes(str) {
    return new TextEncoder().encode(str)
  }

  function hexToArrayBuffer(hex) {
    const bytes = []
    for (let i = 0; i < hex.length; i += 2) {
      bytes.push(parseInt(hex.slice(i, i + 2), 16))
    }
    return new Uint8Array(bytes)
  }

  const passwordAsKey = await crypto.subtle.importKey(
    "raw",
    stringToBytes(password),
    "PBKDF2",
    false,
    ["deriveKey"],
  )

  const key = await crypto.subtle.deriveKey(
    {
      name: "PBKDF2",
      salt: stringToBytes(salt),
      iterations,
      hash: "SHA-256",
    },
    passwordAsKey,
    {name: "AES-GCM", length: 256},
    false,
    ["decrypt"],
  )

  const plaintextBytes = await crypto.subtle.decrypt({
    name: "AES-GCM",
    iv: hexToArrayBuffer(iv)
  }, key, hexToArrayBuffer(ciphertext))

  return new TextDecoder().decode(plaintextBytes)
}

let searchIndex = null

window.runSearch = function (text) {
  const tagIndexes = new Set(search(text))

  const tags = document.getElementById('search-results').children
  for (let i = 0; i < tags.length; i++) {
    tags[i].style.display = tagIndexes.has(i) ? '' : 'none'
  }
}

/**
 * @param {string} text
 * @param {number} maxResults
 * @returns {number[]}
 */
function search(text, maxResults = 3) {
  if (searchIndex === null) {
    searchIndex = buildSearchIndex()
  }

  const scores = new Map()
  for (const trigram of getTrigrams(normalize(text))) {
    const indexes = searchIndex.get(trigram)
    if (indexes) {
      for (const index of indexes) {
        scores.set(index, (scores.get(index) ?? 0) + 1)
      }
    }
  }

  return (
    Array.from(scores.entries())
      .sort((a, b) => b[1] - a[1])
      .slice(0, maxResults)
      .map(each => each[0])
  )
}

function buildSearchIndex() {
  const tags = document.getElementById('search-results').children
  const indexesByTrigram = new Map()

  for (let i = 0; i < tags.length; i++) {
    for (let trigram of getTrigrams(normalize(tags[i].textContent))) {
      if (!indexesByTrigram.has(trigram)) {
        indexesByTrigram.set(trigram, [i])
      } else {
        indexesByTrigram.get(trigram).push(i)
      }
    }
  }

  return indexesByTrigram
}

/**
 * @param {string} text
 * @returns {string[]}
 */
function getTrigrams(text) {
  if (text.length === 0) {
    return []
  }
  
  const trigrams = []

  for (const word of text.split(' ')) {
    if (word.length < 3) {
      trigrams.push(word)
    } else {
      for (let j = 0; j < text.length - 2; j++) {
        trigrams.push(text.slice(j, j + 3))
      }
    }
  }

  return trigrams
}

/**
 * Lower case and remove all especial diacritics from the text
 * @param {string} text
 * @returns {string}
 */
function normalize(text) {
  return text.normalize('NFD').toLowerCase().replace(/[^0-9a-z ]/g, '')
}