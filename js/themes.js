var theme = document.getElementById('theme'); // TODO: Toggle theme

function darkTheme(){
  if (theme.href === '../css/light.css') {
    theme.href = '../css/dark.css';
    console.log(theme.href);
  } else {
    theme.href = '../css/light.css';
    console.log(theme.href);
  }
}

//function lightTheme(path){
  //theme.href = path;
//}
