var theme = document.getElementById('theme');
var themeButton = document.getElementsByClassName('themeButton');

function toggleTheme() {
  if (theme.href == 'https://tigerros.github.io/css/light.css') {
    theme.href = 'https://tigerros.github.io/css/dark.css';
    themeButton.innerHTML = 'Light Theme';
  } else {
    theme.href = 'https://tigerros.github.io/css/light.css';
    themeButton.innerHTML = 'Dark Theme';
  }
}
