import './style.css'

const API_ENDPOINT = `https://${window.location.host}/qr?link=`

document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
  <div>
    <h1>QR Code Generator</h1>
    <form id="form">
      <input type="text" id="input" placeholder="Enter text/url here" />
      <button type="submit" id="submit">Generate</button>
    </form>
    <div id="qr-code">
    </div>
  </div>
  `

  const removeOldImg = () => {
    const img = document.querySelector<HTMLImageElement>('img')
    if (img) img.remove()
  }

const form = document.querySelector<HTMLFormElement>('#form')!
form.addEventListener('submit', (e) => {
  e.preventDefault()
  removeOldImg()
  const input = document.querySelector<HTMLInputElement>('#input')!
  const link = input.value
  if (!link) return
  const img = document.createElement('img')
  img.src = `${API_ENDPOINT}${link}`
  input.value = ''
  document.querySelector<HTMLDivElement>('#qr-code')!.appendChild(img)
});