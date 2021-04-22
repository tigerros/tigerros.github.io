var theme = document.getElementById('theme');

function toggleTheme() {
  if (theme.href == "https://tigerros.github.io/css/light.css") {
    theme.href = "https://tigerros.github.io/css/dark.css";
    console.log(theme.href);
  } else {
    theme.href = "https://tigerros.github.io/css/light.css";
    console.log(theme.href);
  }
}
