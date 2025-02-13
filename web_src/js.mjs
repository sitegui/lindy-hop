window.playVideo = function (thumbnailEl) {
  const videoEl = thumbnailEl.parentElement.querySelector('video')
  const video = thumbnailEl.dataset.video
  const accessRule = thumbnailEl.dataset.accessRule
  const accessIv = thumbnailEl.dataset.accessIv
  const accessCiphertext = thumbnailEl.dataset.accessCiphertext
  const accessSalt = thumbnailEl.dataset.accessSalt
  const accessIterations = thumbnailEl.dataset.accessIterations

  if (video) {
    showVideo(videoEl, thumbnailEl, video)
  } else {
    const password = prompt(`Cette video fait partie de la collection ${accessRule} et est protégée. Merci d'entrer le code d'accès`)
    if (password) {
      decrypt(password, accessSalt, Number(accessIterations), accessIv, accessCiphertext).then(video => {
        showVideo(videoEl, thumbnailEl, video)
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

function showVideo(videoEl, thumbnailEl, video) {
  videoEl.style.display = ''
  videoEl.src = `videos/${video}`
  thumbnailEl.style.display = 'none'
  videoEl.play()
}

function stringToBytes(str) {
  return new TextEncoder().encode(str)
}

function bytesToString(bytes) {
  return new TextDecoder().decode(bytes)
}

function hexToArrayBuffer(hex) {
  const bytes = []
  for (let i = 0; i < hex.length; i += 2) {
    bytes.push(parseInt(hex.slice(i, i + 2), 16))
  }
  return new Uint8Array(bytes)
}

async function decrypt(password, salt, iterations, iv, ciphertext) {
  const key = await deriveKey(password, salt, iterations)

  return bytesToString(await crypto.subtle.decrypt({
    name: "AES-GCM",
    iv: hexToArrayBuffer(iv)
  }, key, hexToArrayBuffer(ciphertext)))
}

async function deriveKey(code, salt, iterations) {
  const codeAsKey = await crypto.subtle.importKey(
    "raw",
    stringToBytes(code),
    "PBKDF2",
    false,
    ["deriveKey"],
  )

  return await crypto.subtle.deriveKey(
    {
      name: "PBKDF2",
      salt: stringToBytes(salt),
      iterations,
      hash: "SHA-256",
    },
    codeAsKey,
    {name: "AES-GCM", length: 256},
    false,
    ["decrypt"],
  )
}
