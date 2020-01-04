const js = import("./node_modules/wasm/wasm.js");
js.then(js => {
  const e = document.getElementById("btn");
  e.addEventListener('click', () => {
	const ee = document.getElementById("src");
	js.analyze(ee.value);
  });
});
