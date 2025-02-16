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

window.applyFilter = function (tag) {
  document.getElementById('tag-filter').value = tag

  for (const containerEl of document.querySelectorAll('.video-container')) {
    if (!tag) {
      containerEl.style.display = ''
    } else {
      const videoTags = JSON.parse(containerEl.dataset.tags)
      containerEl.style.display = videoTags.includes(tag) ? '' : 'none'
    }
  }

  for (const tagEl of document.querySelectorAll('.video-tag')) {
    if (!tag) {
      tagEl.classList.remove('video-tag-selected')
    } else {
      tagEl.classList.toggle('video-tag-selected', tagEl.textContent === tag)
    }
  }
}

window.toggleFilter = function (tag) {
  const current = document.getElementById('tag-filter').value

  if (current === tag) {
    window.applyFilter('')
  } else {
    window.applyFilter(tag)
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
