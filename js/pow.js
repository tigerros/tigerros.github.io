function powerFunc() { // Multiplies the "base" number to the power of the "exponent"
  var base = document.getElementById('base').value
  var exponent = document.getElementById('exponent').value

  return Math.pow(base, exponent)
}

function resultFunc() { // Gets result
  var result = document.getElementById('result')

  result.innerHTML = powerFunc()
  console.log("Showed result")
}
