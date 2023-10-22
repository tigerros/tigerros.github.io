# Who needs fancy task runners when we have the god-given shell?
# I switched from Gulp and I'm satisfied.

set -x -e

trunk clean
trunk build --release

npx rollup --config

cd dist || exit

rm -rf ./snippets
npx html-minifier --file-ext html --input-dir . --output-dir . --output index.html --config-file ../html-min.json
cp index.html 404.html
touch .nojekyll

echo Finished